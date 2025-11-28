//! Request extractors for type-safe data extraction
//!
//! Extractors provide a type-safe way to extract data from HTTP requests.
//! They implement the `FromRequest` trait which allows them to be used
//! as handler parameters.

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::{Error, Request, Result};

/// Trait for types that can be extracted from a request
#[async_trait]
pub trait FromRequest: Sized {
    /// Extract this type from the request
    async fn from_request(req: &mut Request) -> Result<Self>;
}

/// Extractor for path parameters
///
/// # Example
///
/// ```ignore
/// use ruffus::extractors::Path;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct UserParams {
///     id: String,
/// }
///
/// async fn get_user(Path(params): Path<UserParams>) -> Response {
///     // Use params.id
/// }
/// ```
pub struct Path<T>(pub T);

#[async_trait]
impl<T> FromRequest for Path<T>
where
    T: DeserializeOwned + Send,
{
    async fn from_request(req: &mut Request) -> Result<Self> {
        // Convert the params HashMap to a serde_json::Value
        // Try to intelligently convert string values to appropriate JSON types
        let params_map: HashMap<String, serde_json::Value> = req
            .params()
            .iter()
            .map(|(k, v)| {
                // Try to parse as number first, then boolean, fall back to string
                let value = if let Ok(num) = v.parse::<i64>() {
                    serde_json::Value::Number(num.into())
                } else if let Ok(num) = v.parse::<u64>() {
                    serde_json::Value::Number(num.into())
                } else if let Ok(num) = v.parse::<f64>() {
                    serde_json::Number::from_f64(num)
                        .map(serde_json::Value::Number)
                        .unwrap_or_else(|| serde_json::Value::String(v.clone()))
                } else if let Ok(b) = v.parse::<bool>() {
                    serde_json::Value::Bool(b)
                } else {
                    serde_json::Value::String(v.clone())
                };
                (k.clone(), value)
            })
            .collect();

        let value = serde_json::Value::Object(
            params_map
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect(),
        );

        let params: T = serde_json::from_value(value)
            .map_err(|e| Error::BadRequest(format!("Failed to parse path parameters: {}", e)))?;

        Ok(Path(params))
    }
}

/// Extractor for JSON request body
///
/// # Example
///
/// ```ignore
/// use ruffus::extractors::Json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     name: String,
///     email: String,
/// }
///
/// async fn create_user(Json(user): Json<CreateUser>) -> Response {
///     // Use user.name and user.email
/// }
/// ```
pub struct Json<T>(pub T);

#[async_trait]
impl<T> FromRequest for Json<T>
where
    T: DeserializeOwned + Send,
{
    async fn from_request(req: &mut Request) -> Result<Self> {
        let value = req.json().await?;
        Ok(Json(value))
    }
}

/// Extractor for query parameters
///
/// # Example
///
/// ```ignore
/// use ruffus::extractors::Query;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Pagination {
///     page: u32,
///     limit: u32,
/// }
///
/// async fn list_users(Query(params): Query<Pagination>) -> Response {
///     // Use params.page and params.limit
/// }
/// ```
pub struct Query<T>(pub T);

#[async_trait]
impl<T> FromRequest for Query<T>
where
    T: DeserializeOwned + Send,
{
    async fn from_request(req: &mut Request) -> Result<Self> {
        // Convert the query params HashMap to a serde_json::Value
        // Try to parse as numbers/booleans first, fall back to strings
        let query_map: HashMap<String, serde_json::Value> = req
            .query_params()
            .iter()
            .map(|(k, v)| {
                let value = if let Ok(num) = v.parse::<i64>() {
                    serde_json::Value::Number(num.into())
                } else if let Ok(num) = v.parse::<f64>() {
                    serde_json::Number::from_f64(num)
                        .map(serde_json::Value::Number)
                        .unwrap_or_else(|| serde_json::Value::String(v.clone()))
                } else if let Ok(b) = v.parse::<bool>() {
                    serde_json::Value::Bool(b)
                } else {
                    serde_json::Value::String(v.clone())
                };
                (k.clone(), value)
            })
            .collect();

        let value = serde_json::Value::Object(
            query_map
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect(),
        );

        let params: T = serde_json::from_value(value)
            .map_err(|e| Error::BadRequest(format!("Failed to parse query parameters: {}", e)))?;

        Ok(Query(params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use http::{HeaderMap, Method, Uri};
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestParams {
        id: u32,
        name: String,
    }

    #[tokio::test]
    async fn test_path_extractor() {
        let mut req = Request::new(
            Method::GET,
            Uri::from_static("/users/123"),
            HeaderMap::new(),
            Bytes::new(),
        );
        req.set_param("id".to_string(), "123".to_string());
        req.set_param("name".to_string(), "john".to_string());

        let Path(params): Path<TestParams> = Path::from_request(&mut req).await.unwrap();
        assert_eq!(params.id, 123);
        assert_eq!(params.name, "john");
    }

    #[tokio::test]
    async fn test_json_extractor() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct User {
            name: String,
            age: u32,
        }

        let json_body = r#"{"name":"Alice","age":30}"#;
        let mut req = Request::new(
            Method::POST,
            Uri::from_static("/users"),
            HeaderMap::new(),
            Bytes::from(json_body),
        );

        let Json(user): Json<User> = Json::from_request(&mut req).await.unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
    }

    #[tokio::test]
    async fn test_query_extractor() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Pagination {
            page: u32,
            limit: u32,
        }

        let mut req = Request::new(
            Method::GET,
            Uri::from_static("/users?page=1&limit=10"),
            HeaderMap::new(),
            Bytes::new(),
        );

        let Query(params): Query<Pagination> = Query::from_request(&mut req).await.unwrap();
        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 10);
    }

    #[tokio::test]
    async fn test_path_extractor_with_dash() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct PathParams {
            name: String,
            id: u32,
        }

        let mut req = Request::new(
            Method::GET,
            Uri::from_static("/users/-/0"),
            HeaderMap::new(),
            Bytes::new(),
        );
        req.set_param("name".to_string(), "-".to_string());
        req.set_param("id".to_string(), "0".to_string());

        let Path(params) = Path::<PathParams>::from_request(&mut req).await.unwrap();
        assert_eq!(params.name, "-");
        assert_eq!(params.id, 0);
    }
}
