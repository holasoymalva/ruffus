# Requirements Document

## Introduction

Este documento especifica los requisitos para **Ruffus**, un framework web moderno para Rust inspirado en Express.js. Ruffus es un "Fast, minimalist web framework for Rust" que permitirá a los desarrolladores crear APIs web de manera simple, rápida y con una sintaxis ergonómica similar a Express.js, aprovechando las garantías de seguridad y rendimiento de Rust.

## Glossary

- **Ruffus**: El framework web completo - un sistema rápido y minimalista para construir aplicaciones web en Rust
- **Router**: Componente que gestiona el enrutamiento de peticiones HTTP a handlers específicos
- **Middleware**: Función que procesa peticiones HTTP antes de llegar al handler final o modifica respuestas
- **Handler**: Función que procesa una petición HTTP y genera una respuesta
- **Request**: Objeto que representa una petición HTTP entrante con método, ruta, headers y body
- **Response**: Objeto que representa una respuesta HTTP saliente con status, headers y body
- **Application**: Instancia principal del framework que configura y ejecuta el servidor web
- **Route**: Combinación de método HTTP y patrón de ruta asociado a un handler
- **PathParameter**: Variable extraída de la ruta URL (ej: `/users/:id`)
- **QueryParameter**: Parámetro extraído de la query string de la URL
- **JSONBody**: Cuerpo de petición o respuesta en formato JSON

## Requirements

### Requirement 1

**User Story:** Como desarrollador de APIs, quiero crear una aplicación web básica con rutas simples, para poder responder a peticiones HTTP rápidamente.

#### Acceptance Criteria

1. WHEN a developer creates a new Application instance, THEN Ruffus SHALL initialize the routing system and middleware stack
2. WHEN a developer defines a Route with a method and path, THEN Ruffus SHALL register the Route in the Router
3. WHEN the Application receives a Request matching a registered Route, THEN Ruffus SHALL invoke the corresponding Handler
4. WHEN a Handler returns a Response, THEN Ruffus SHALL send the Response to the client with correct status code and headers
5. WHEN the Application starts listening on a port, THEN Ruffus SHALL bind to the specified port and accept incoming connections

### Requirement 2

**User Story:** Como desarrollador de APIs, quiero definir rutas con parámetros dinámicos, para poder capturar valores de la URL y usarlos en mi lógica de negocio.

#### Acceptance Criteria

1. WHEN a Route pattern contains PathParameter syntax (`:name`), THEN Ruffus SHALL extract the parameter value from the Request URL
2. WHEN multiple PathParameters are defined in a Route, THEN Ruffus SHALL extract all parameters and make them accessible to the Handler
3. WHEN a Request URL matches a Route pattern with PathParameters, THEN Ruffus SHALL provide the extracted values in a type-safe manner
4. WHEN a PathParameter value is accessed, THEN Ruffus SHALL return the decoded string value from the URL segment

### Requirement 3

**User Story:** Como desarrollador de APIs, quiero acceder a query parameters y request body, para poder procesar datos enviados por el cliente.

#### Acceptance Criteria

1. WHEN a Request contains QueryParameters in the URL, THEN Ruffus SHALL parse and provide access to all query parameters
2. WHEN a Request contains a JSONBody, THEN Ruffus SHALL deserialize the JSON into the specified Rust type
3. WHEN JSON deserialization fails, THEN Ruffus SHALL return an error response with status 400 and descriptive error message
4. WHEN a Handler accesses Request headers, THEN Ruffus SHALL provide all HTTP headers in a queryable format
5. WHEN a Request body is empty and the Handler expects JSONBody, THEN Ruffus SHALL handle the case gracefully with appropriate error

### Requirement 4

**User Story:** Como desarrollador de APIs, quiero usar middleware para funcionalidades transversales, para poder implementar logging, autenticación y otras preocupaciones de manera modular.

#### Acceptance Criteria

1. WHEN a developer registers Middleware in the Application, THEN Ruffus SHALL add it to the middleware stack in registration order
2. WHEN a Request is received, THEN Ruffus SHALL execute all Middleware functions in order before invoking the Handler
3. WHEN Middleware modifies the Request, THEN Ruffus SHALL pass the modified Request to subsequent Middleware and the Handler
4. WHEN Middleware returns early with a Response, THEN Ruffus SHALL skip remaining Middleware and the Handler, returning the Response immediately
5. WHEN Middleware execution completes without early return, THEN Ruffus SHALL continue to the next Middleware or Handler

### Requirement 5

**User Story:** Como desarrollador de APIs, quiero construir respuestas JSON fácilmente, para poder enviar datos estructurados al cliente de forma simple.

#### Acceptance Criteria

1. WHEN a Handler returns a Rust type that implements serialization, THEN Ruffus SHALL serialize it to JSONBody automatically
2. WHEN JSON serialization succeeds, THEN Ruffus SHALL set the Content-Type header to `application/json`
3. WHEN a Handler sets a custom status code, THEN Ruffus SHALL include that status code in the Response
4. WHEN a Handler adds custom headers to the Response, THEN Ruffus SHALL include all custom headers in the HTTP response
5. WHEN JSON serialization fails, THEN Ruffus SHALL return an error response with status 500

### Requirement 6

**User Story:** Como desarrollador de APIs, quiero manejar errores de manera consistente, para que mi API tenga un comportamiento predecible ante fallos.

#### Acceptance Criteria

1. WHEN a Handler returns an error, THEN Ruffus SHALL convert it to an appropriate HTTP error response
2. WHEN an unhandled error occurs during Request processing, THEN Ruffus SHALL return status 500 with a safe error message
3. WHEN a Route is not found for a Request, THEN Ruffus SHALL return status 404 with a not found message
4. WHEN a method is not allowed for a Route, THEN Ruffus SHALL return status 405 with allowed methods in headers
5. WHEN error handling Middleware is registered, THEN Ruffus SHALL invoke it for all errors before sending the Response

### Requirement 7

**User Story:** Como desarrollador de APIs, quiero organizar rutas en grupos con prefijos comunes, para poder estructurar mi API de manera lógica y modular.

#### Acceptance Criteria

1. WHEN a developer creates a Router with a path prefix, THEN Ruffus SHALL prepend the prefix to all Routes registered on that Router
2. WHEN a Router is mounted on the Application, THEN Ruffus SHALL register all Routes from the Router with their full paths
3. WHEN nested Routers are created, THEN Ruffus SHALL combine all path prefixes correctly
4. WHEN Middleware is registered on a Router, THEN Ruffus SHALL apply it only to Routes within that Router

### Requirement 8

**User Story:** Como desarrollador de APIs, quiero soporte para todos los métodos HTTP comunes, para poder implementar APIs RESTful completas.

#### Acceptance Criteria

1. WHEN a developer registers a Route with GET method, THEN Ruffus SHALL match only GET Requests to that Route
2. WHEN a developer registers a Route with POST method, THEN Ruffus SHALL match only POST Requests to that Route
3. WHEN a developer registers a Route with PUT method, THEN Ruffus SHALL match only PUT Requests to that Route
4. WHEN a developer registers a Route with DELETE method, THEN Ruffus SHALL match only DELETE Requests to that Route
5. WHEN a developer registers a Route with PATCH method, THEN Ruffus SHALL match only PATCH Requests to that Route

### Requirement 9

**User Story:** Como desarrollador de APIs, quiero una sintaxis ergonómica y expresiva similar a Express.js, para poder escribir código conciso y legible.

#### Acceptance Criteria

1. WHEN defining Routes, THEN Ruffus SHALL provide a fluent API with method chaining
2. WHEN registering Handlers, THEN Ruffus SHALL accept closures and async functions
3. WHEN extracting Request data, THEN Ruffus SHALL provide extractor patterns that work with Rust's type system
4. WHEN building Responses, THEN Ruffus SHALL provide builder methods for status, headers, and body
5. WHEN working with the API, THEN Ruffus SHALL minimize boilerplate code required

### Requirement 10

**User Story:** Como desarrollador de APIs, quiero soporte para operaciones asíncronas, para poder manejar I/O de manera eficiente sin bloquear el servidor.

#### Acceptance Criteria

1. WHEN a Handler is defined as async, THEN Ruffus SHALL execute it asynchronously using Rust's async runtime
2. WHEN multiple Requests arrive concurrently, THEN Ruffus SHALL handle them concurrently without blocking
3. WHEN Middleware is async, THEN Ruffus SHALL await its completion before proceeding
4. WHEN async operations are in progress, THEN Ruffus SHALL not block the event loop
