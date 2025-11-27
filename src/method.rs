//! HTTP Method type

use std::fmt;

/// HTTP request methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// GET method
    GET,
    /// POST method
    POST,
    /// PUT method
    PUT,
    /// DELETE method
    DELETE,
    /// PATCH method
    PATCH,
    /// OPTIONS method
    OPTIONS,
    /// HEAD method
    HEAD,
}

impl Method {
    /// Convert from hyper/http Method
    pub fn from_hyper(method: &http::Method) -> Option<Self> {
        match *method {
            http::Method::GET => Some(Method::GET),
            http::Method::POST => Some(Method::POST),
            http::Method::PUT => Some(Method::PUT),
            http::Method::DELETE => Some(Method::DELETE),
            http::Method::PATCH => Some(Method::PATCH),
            http::Method::OPTIONS => Some(Method::OPTIONS),
            http::Method::HEAD => Some(Method::HEAD),
            _ => None,
        }
    }
}

impl From<http::Method> for Method {
    fn from(method: http::Method) -> Self {
        Method::from_hyper(&method).unwrap_or_else(|| {
            panic!("Unsupported HTTP method: {}", method)
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PATCH => write!(f, "PATCH"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::HEAD => write!(f, "HEAD"),
        }
    }
}

impl From<Method> for http::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::GET => http::Method::GET,
            Method::POST => http::Method::POST,
            Method::PUT => http::Method::PUT,
            Method::DELETE => http::Method::DELETE,
            Method::PATCH => http::Method::PATCH,
            Method::OPTIONS => http::Method::OPTIONS,
            Method::HEAD => http::Method::HEAD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hyper_conversion() {
        assert_eq!(Method::from(http::Method::GET), Method::GET);
        assert_eq!(Method::from(http::Method::POST), Method::POST);
        assert_eq!(Method::from(http::Method::PUT), Method::PUT);
        assert_eq!(Method::from(http::Method::DELETE), Method::DELETE);
        assert_eq!(Method::from(http::Method::PATCH), Method::PATCH);
        assert_eq!(Method::from(http::Method::OPTIONS), Method::OPTIONS);
        assert_eq!(Method::from(http::Method::HEAD), Method::HEAD);
    }

    #[test]
    fn test_display() {
        assert_eq!(Method::GET.to_string(), "GET");
        assert_eq!(Method::POST.to_string(), "POST");
        assert_eq!(Method::PUT.to_string(), "PUT");
        assert_eq!(Method::DELETE.to_string(), "DELETE");
        assert_eq!(Method::PATCH.to_string(), "PATCH");
        assert_eq!(Method::OPTIONS.to_string(), "OPTIONS");
        assert_eq!(Method::HEAD.to_string(), "HEAD");
    }

    #[test]
    fn test_equality() {
        assert_eq!(Method::GET, Method::GET);
        assert_ne!(Method::GET, Method::POST);
        assert_eq!(Method::POST, Method::POST);
    }

    #[test]
    fn test_clone() {
        let method = Method::GET;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }
}
