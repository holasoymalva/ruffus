# Design Document: Ruffus - Fast, Minimalist Web Framework for Rust

## Overview

**Ruffus** es un framework web rápido y minimalista para Rust que proporciona una API ergonómica y expresiva inspirada en Express.js, permitiendo a los desarrolladores crear APIs web de manera rápida y simple. Ruffus aprovecha el sistema de tipos de Rust, async/await, y el ecosistema de Tokio para ofrecer alto rendimiento con garantías de seguridad en tiempo de compilación.

Ruffus se centra en tres pilares fundamentales:
1. **Minimalismo y Velocidad**: API simple y directa que maximiza el rendimiento
2. **Ergonomía**: API fluida y expresiva similar a Express.js
3. **Seguridad**: Aprovechamiento del sistema de tipos de Rust para prevenir errores comunes

## Architecture

El framework sigue una arquitectura en capas:

```
┌─────────────────────────────────────┐
│     Application Layer               │
│  (App, Router, Route Registration)  │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│     Middleware Layer                │
│  (Middleware Stack, Execution)      │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│     Routing Layer                   │
│  (Route Matching, Path Params)      │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│     Handler Layer                   │
│  (Request Processing, Response)     │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│     HTTP Layer                      │
│  (Hyper, Tokio Runtime)             │
└─────────────────────────────────────┘
```

### Key Design Decisions

1. **Hyper como base HTTP**: Utilizaremos Hyper (v1.x) como servidor HTTP subyacente por su madurez, rendimiento y soporte async
2. **Tokio como runtime**: Tokio proporciona el runtime asíncrono más maduro y eficiente para Rust
3. **Serde para serialización**: Serde es el estándar de facto para serialización/deserialización en Rust
4. **Extractors pattern**: Inspirado en Axum, usaremos extractors para obtener datos de requests de forma type-safe
5. **Tower middleware**: Aprovecharemos el ecosistema Tower para middleware reutilizable

## Components and Interfaces

### 1. Application (`App`)

El punto de entrada principal del framework.

```rust
pub struct App {
    router: Router,
    middleware: Vec<Box<dyn Middleware>>,
}

impl App {
    pub fn new() -> Self;
    pub fn get(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn post(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn put(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn delete(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn patch(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn use_middleware(&mut self, middleware: impl Middleware) -> &mut Self;
    pub fn mount(&mut self, prefix: &str, router: Router) -> &mut Self;
    pub async fn listen(&self, addr: &str) -> Result<()>;
}
```

### 2. Router

Gestiona grupos de rutas con prefijos comunes.

```rust
pub struct Router {
    prefix: String,
    routes: Vec<Route>,
    middleware: Vec<Box<dyn Middleware>>,
}

impl Router {
    pub fn new(prefix: &str) -> Self;
    pub fn get(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn post(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn put(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn delete(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn patch(&mut self, path: &str, handler: impl Handler) -> &mut Self;
    pub fn use_middleware(&mut self, middleware: impl Middleware) -> &mut Self;
}
```

### 3. Request

Representa una petición HTTP entrante.

```rust
pub struct Request {
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
    params: HashMap<String, String>,
    query: HashMap<String, String>,
    extensions: Extensions,
}

impl Request {
    pub fn method(&self) -> &Method;
    pub fn uri(&self) -> &Uri;
    pub fn headers(&self) -> &HeaderMap;
    pub fn param(&self, name: &str) -> Option<&str>;
    pub fn query(&self, name: &str) -> Option<&str>;
    pub async fn json<T: DeserializeOwned>(&mut self) -> Result<T>;
}
```

### 4. Response

Representa una respuesta HTTP saliente.

```rust
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: Body,
}

impl Response {
    pub fn new() -> Self;
    pub fn status(mut self, status: StatusCode) -> Self;
    pub fn header(mut self, key: &str, value: &str) -> Self;
    pub fn json<T: Serialize>(value: &T) -> Result<Self>;
    pub fn text(text: String) -> Self;
}
```

### 5. Handler Trait

Define la interfaz para handlers de rutas.

```rust
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self, req: Request) -> Result<Response>;
}

// Implementación automática para funciones async
impl<F, Fut> Handler for F
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response>> + Send,
{
    async fn handle(&self, req: Request) -> Result<Response> {
        self(req).await
    }
}
```

### 6. Middleware Trait

Define la interfaz para middleware.

```rust
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn handle(&self, req: Request, next: Next) -> Result<Response>;
}

pub struct Next {
    // Internal state for middleware chain
}

impl Next {
    pub async fn run(self, req: Request) -> Result<Response>;
}
```

### 7. Extractors

Patrones para extraer datos de requests de forma type-safe.

```rust
// Path parameters
pub struct Path<T>(pub T);

impl<T: DeserializeOwned> FromRequest for Path<T> {
    async fn from_request(req: &Request) -> Result<Self>;
}

// JSON body
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> FromRequest for Json<T> {
    async fn from_request(req: &mut Request) -> Result<Self>;
}

// Query parameters
pub struct Query<T>(pub T);

impl<T: DeserializeOwned> FromRequest for Query<T> {
    async fn from_request(req: &Request) -> Result<Self>;
}
```

## Data Models

### Route

```rust
struct Route {
    method: Method,
    pattern: PathPattern,
    handler: Box<dyn Handler>,
}

struct PathPattern {
    segments: Vec<Segment>,
}

enum Segment {
    Static(String),
    Dynamic(String), // Parameter name
}
```

### Method

```rust
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}
```

### Error Types

```rust
pub enum Error {
    RouteNotFound,
    MethodNotAllowed(Vec<Method>),
    BadRequest(String),
    InternalServerError(String),
    JsonParseError(serde_json::Error),
    Custom(Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    pub fn status_code(&self) -> StatusCode;
    pub fn into_response(self) -> Response;
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Application initialization creates empty state
*For any* newly created Application instance, the router and middleware stack should be initialized and empty.
**Validates: Requirements 1.1**

### Property 2: Route registration is preserved
*For any* route with a method and path, registering it in the router should make it retrievable and matchable.
**Validates: Requirements 1.2**

### Property 3: Matching requests invoke handlers
*For any* registered route and matching request, the framework should invoke the corresponding handler.
**Validates: Requirements 1.3**

### Property 4: Handler responses are sent correctly
*For any* response returned by a handler, the framework should send it with the correct status code and headers.
**Validates: Requirements 1.4**

### Property 5: Server binds to specified port
*For any* valid port number, starting the application should successfully bind to that port.
**Validates: Requirements 1.5**

### Property 6: Path parameters are extracted correctly
*For any* route pattern with path parameters and matching URL, the framework should extract parameter values in a type-safe manner and make them accessible.
**Validates: Requirements 2.1, 2.3**

### Property 7: Multiple path parameters are all extracted
*For any* route with multiple path parameters, all parameter values should be extracted and accessible.
**Validates: Requirements 2.2**

### Property 8: URL decoding round-trip
*For any* URL-encoded path parameter value, extracting and accessing it should return the decoded original value.
**Validates: Requirements 2.4**

### Property 9: Query parameters are parsed completely
*For any* request with query parameters, all parameters should be parsed and accessible.
**Validates: Requirements 3.1**

### Property 10: JSON deserialization round-trip
*For any* valid JSON body and corresponding Rust type, deserializing then serializing should produce equivalent JSON.
**Validates: Requirements 3.2, 5.1**

### Property 11: Invalid JSON returns 400 error
*For any* invalid JSON body, attempting to deserialize should return a 400 status code with an error message.
**Validates: Requirements 3.3**

### Property 12: All request headers are accessible
*For any* request with headers, all headers should be queryable and retrievable.
**Validates: Requirements 3.4**

### Property 13: Middleware registration order is preserved
*For any* sequence of middleware registrations, the middleware stack should maintain the registration order.
**Validates: Requirements 4.1**

### Property 14: Middleware executes in order
*For any* request and registered middleware, all middleware should execute in registration order before the handler, unless one returns early.
**Validates: Requirements 4.2, 4.5**

### Property 15: Request modifications propagate through chain
*For any* middleware that modifies the request, subsequent middleware and the handler should receive the modified request.
**Validates: Requirements 4.3**

### Property 16: Early middleware return skips remaining chain
*For any* middleware that returns a response early, the framework should skip all remaining middleware and the handler.
**Validates: Requirements 4.4**

### Property 17: JSON responses include correct Content-Type
*For any* successful JSON serialization, the response should have Content-Type header set to "application/json".
**Validates: Requirements 5.2**

### Property 18: Custom status codes are preserved
*For any* custom status code set by a handler, the response should include that exact status code.
**Validates: Requirements 5.3**

### Property 19: Custom headers are included in response
*For any* custom headers added by a handler, all headers should be included in the HTTP response.
**Validates: Requirements 5.4**

### Property 20: Serialization failures return 500
*For any* type that fails to serialize to JSON, the framework should return a 500 status code.
**Validates: Requirements 5.5**

### Property 21: Handler errors convert to HTTP responses
*For any* error returned by a handler, the framework should convert it to an appropriate HTTP error response.
**Validates: Requirements 6.1**

### Property 22: Unhandled errors return 500
*For any* unhandled error during request processing, the framework should return a 500 status code with a safe error message.
**Validates: Requirements 6.2**

### Property 23: Non-existent routes return 404
*For any* request to a non-registered route, the framework should return a 404 status code.
**Validates: Requirements 6.3**

### Property 24: Wrong method returns 405
*For any* request with a method not allowed for a route, the framework should return a 405 status code with allowed methods in headers.
**Validates: Requirements 6.4**

### Property 25: Error middleware handles all errors
*For any* error and registered error middleware, the error middleware should be invoked before sending the response.
**Validates: Requirements 6.5**

### Property 26: Router prefix prepends to all routes
*For any* router with a prefix and registered routes, all route paths should have the prefix prepended.
**Validates: Requirements 7.1**

### Property 27: Mounted router routes are registered
*For any* router mounted on an application, all routes from the router should be registered with their full paths.
**Validates: Requirements 7.2**

### Property 28: Nested router prefixes combine correctly
*For any* nested routers with prefixes, the final route paths should correctly combine all prefixes in order.
**Validates: Requirements 7.3**

### Property 29: Router middleware scopes correctly
*For any* middleware registered on a router, it should only apply to routes within that router, not to other routes.
**Validates: Requirements 7.4**

### Property 30: HTTP method matching is exclusive
*For any* route registered with a specific HTTP method, only requests with that exact method should match the route.
**Validates: Requirements 8.1, 8.2, 8.3, 8.4, 8.5**

### Property 31: Various handler types are accepted
*For any* valid handler type (closure, async function, etc.), the framework should accept and execute it correctly.
**Validates: Requirements 9.2**

### Property 32: Extractors work with various types
*For any* valid Rust type with appropriate traits, extractors should successfully extract request data into that type.
**Validates: Requirements 9.3**

### Property 33: Response builder methods work correctly
*For any* sequence of response builder method calls, the final response should reflect all modifications.
**Validates: Requirements 9.4**

### Property 34: Async handlers execute asynchronously
*For any* async handler, the framework should execute it using the async runtime without blocking.
**Validates: Requirements 10.1**

### Property 35: Concurrent requests are handled concurrently
*For any* set of concurrent requests, the framework should process them concurrently without blocking each other.
**Validates: Requirements 10.2**

### Property 36: Async middleware completes before proceeding
*For any* async middleware, the framework should await its completion before continuing to the next middleware or handler.
**Validates: Requirements 10.3**



## Error Handling

El framework implementa un sistema de manejo de errores robusto y ergonómico:

### Error Type Hierarchy

```rust
pub enum Error {
    // Client errors (4xx)
    RouteNotFound,
    MethodNotAllowed(Vec<Method>),
    BadRequest(String),
    
    // Server errors (5xx)
    InternalServerError(String),
    
    // Specific errors
    JsonParseError(serde_json::Error),
    JsonSerializeError(serde_json::Error),
    
    // Custom errors
    Custom {
        status: StatusCode,
        message: String,
    },
}
```

### Error Conversion Strategy

1. **Automatic conversion**: Errores comunes (JSON parse, IO) se convierten automáticamente a respuestas HTTP apropiadas
2. **Custom error handlers**: Los usuarios pueden registrar middleware de manejo de errores personalizado
3. **Safe error messages**: Los errores internos no exponen detalles de implementación al cliente
4. **Error propagation**: Los errores se propagan a través de la cadena de middleware hasta llegar al error handler

### Error Response Format

```json
{
  "error": {
    "status": 400,
    "message": "Invalid JSON in request body",
    "details": "Expected closing bracket at line 5"
  }
}
```

## Testing Strategy

El framework utilizará una estrategia de testing dual que combina unit tests y property-based tests para garantizar corrección completa.

### Unit Testing

Los unit tests verificarán:

- **Ejemplos específicos**: Casos de uso comunes y documentados
- **Edge cases**: Casos límite como rutas vacías, headers especiales, etc.
- **Integración entre componentes**: Interacción entre router, middleware y handlers
- **Error conditions**: Comportamiento ante errores específicos

**Framework de testing**: Utilizaremos el framework estándar de Rust (`#[test]`) junto con `tokio::test` para tests asíncronos.

### Property-Based Testing

Los property-based tests verificarán las propiedades universales definidas en la sección de Correctness Properties.

**Framework de PBT**: Utilizaremos **quickcheck** para Rust, que es el framework de property-based testing más maduro y ampliamente usado.

**Configuración de PBT**:
- Cada test de propiedad ejecutará un mínimo de **100 iteraciones** para asegurar cobertura adecuada
- Cada test debe estar etiquetado con un comentario que referencie explícitamente la propiedad del documento de diseño
- Formato del tag: `// Feature: rust-web-framework, Property {number}: {property_text}`
- Cada propiedad de corrección debe ser implementada por UN SOLO test de propiedad

**Generadores personalizados**:
- `Arbitrary` implementations para `Request`, `Response`, `Route`, etc.
- Generadores inteligentes que producen datos válidos dentro del dominio del problema
- Shrinking automático para encontrar casos mínimos de fallo

### Test Organization

```
src/
├── lib.rs
├── app.rs
├── router.rs
├── request.rs
├── response.rs
├── middleware.rs
└── tests/
    ├── unit/
    │   ├── app_tests.rs
    │   ├── router_tests.rs
    │   └── middleware_tests.rs
    └── property/
        ├── routing_properties.rs
        ├── middleware_properties.rs
        └── serialization_properties.rs
```

### Integration Testing

Además de unit y property tests, se incluirán integration tests que:
- Inicien un servidor real en un puerto de prueba
- Realicen peticiones HTTP reales usando un cliente HTTP
- Verifiquen el comportamiento end-to-end del framework

## Performance Considerations

### Async Runtime

- **Tokio multi-threaded runtime**: Aprovecha múltiples cores para máximo throughput
- **Work-stealing scheduler**: Distribuye carga automáticamente entre threads
- **Efficient task spawning**: Minimiza overhead de creación de tareas

### Memory Management

- **Zero-copy parsing**: Usa `Bytes` para evitar copias innecesarias de datos
- **Efficient routing**: Usa trie o radix tree para matching rápido de rutas
- **Connection pooling**: Reutiliza conexiones HTTP cuando sea posible

### Optimization Strategies

1. **Route compilation**: Pre-compilar patrones de rutas en estructuras eficientes
2. **Middleware caching**: Cachear resultados de middleware cuando sea seguro
3. **Header parsing**: Parsing lazy de headers solo cuando se acceden
4. **Body streaming**: Soporte para streaming de request/response bodies grandes

## Dependencies

### Core Dependencies

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
hyper = { version = "1.0", features = ["server", "http1", "http2"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"
bytes = "1.5"
http = "1.0"

[dev-dependencies]
quickcheck = "1.0"
quickcheck_macros = "1.0"
```

### Rationale

- **Tokio**: Runtime asíncrono más maduro y eficiente
- **Hyper**: Servidor HTTP de bajo nivel, rápido y confiable
- **Serde**: Estándar de facto para serialización en Rust
- **async-trait**: Permite traits con métodos async
- **Bytes**: Manejo eficiente de buffers de bytes
- **quickcheck**: Framework maduro para property-based testing

## API Examples

### Basic Application

```rust
use ruffus::{App, Request, Response};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    
    app.get("/", |_req: Request| async {
        Response::text("Hello, World!".to_string())
    });
    
    app.listen("127.0.0.1:3000").await.unwrap();
}
```

### With Path Parameters

```rust
app.get("/users/:id", |req: Request| async {
    let id = req.param("id").unwrap();
    Response::json(&json!({ "user_id": id }))
});
```

### With JSON Body

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

app.post("/users", |mut req: Request| async {
    let body: CreateUser = req.json().await?;
    let user = User {
        id: 1,
        name: body.name,
        email: body.email,
    };
    Response::json(&user)
});
```

### With Middleware

```rust
use ruffus::middleware::{Middleware, Next};

struct Logger;

#[async_trait]
impl Middleware for Logger {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        println!("{} {}", req.method(), req.uri());
        next.run(req).await
    }
}

app.use_middleware(Logger);
```

### With Router

```rust
let mut api = Router::new("/api");

api.get("/users", |_req: Request| async {
    Response::json(&json!({ "users": [] }))
});

api.post("/users", |mut req: Request| async {
    let body: CreateUser = req.json().await?;
    Response::json(&body)
});

app.mount("/", api);
// Routes are now available at /api/users
```

## Future Enhancements

Posibles mejoras para versiones futuras:

1. **WebSocket support**: Soporte nativo para WebSockets
2. **Static file serving**: Middleware para servir archivos estáticos
3. **Template engines**: Integración con motores de templates
4. **Session management**: Middleware para manejo de sesiones
5. **CORS middleware**: Middleware built-in para CORS
6. **Rate limiting**: Middleware para rate limiting
7. **Compression**: Soporte automático para gzip/brotli
8. **OpenAPI generation**: Generación automática de especificaciones OpenAPI
