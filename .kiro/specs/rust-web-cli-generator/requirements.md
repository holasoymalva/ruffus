# Requirements Document

## Introduction

Ruffus es una herramienta CLI para scaffolding de servicios web en Rust, inspirada en la simplicidad y productividad de Flask para Python. El sistema automatiza la creación de servicios, rutas, middleware y módulos completos para desarrollo backend, soportando múltiples frameworks web de Rust (Axum, Actix-web, Warp, Rocket) con plantillas optimizadas y mejores prácticas integradas.

## Glossary

- **CLI**: Command Line Interface - Interfaz de línea de comandos para interactuar con la herramienta
- **Ruffus**: El sistema de generación de código CLI para servicios web en Rust
- **Framework**: Biblioteca web de Rust (Axum, Actix-web, Warp, Rocket) utilizada para construir servicios HTTP
- **Service**: Capa de lógica de negocio que contiene la funcionalidad principal de la aplicación
- **Route**: Endpoint HTTP que maneja peticiones y respuestas web
- **Guard**: Middleware que intercepta peticiones para autenticación, validación o logging
- **Module**: Conjunto completo de componentes (servicios, rutas, guards) que forman una funcionalidad cohesiva
- **Template**: Plantilla de código reutilizable con variables que se renderiza para generar componentes
- **Scaffolding**: Generación automática de estructura de código y archivos
- **CRUD**: Create, Read, Update, Delete - Operaciones básicas de datos

## Requirements

### Requirement 1

**User Story:** Como desarrollador backend, quiero generar servicios de lógica de negocio rápidamente, para poder enfocarme en implementar la funcionalidad específica sin escribir código boilerplate.

#### Acceptance Criteria

1. WHEN un desarrollador ejecuta el comando de generación de servicio con un nombre válido THEN Ruffus SHALL crear un archivo de servicio con estructura básica y métodos stub
2. WHEN un desarrollador especifica un módulo para el servicio THEN Ruffus SHALL crear el servicio dentro del directorio del módulo especificado
3. WHEN Ruffus crea un nuevo servicio THEN Ruffus SHALL validar que el nombre del servicio no existe previamente en el proyecto
4. WHEN Ruffus genera un servicio THEN Ruffus SHALL actualizar el archivo mod.rs correspondiente para exportar el nuevo servicio
5. WHEN la generación de servicio falla por cualquier razón THEN Ruffus SHALL revertir todos los cambios realizados y mostrar un mensaje de error descriptivo

### Requirement 2

**User Story:** Como desarrollador de APIs REST, quiero generar endpoints HTTP con routing automático, para poder crear APIs completas sin configurar manualmente cada ruta.

#### Acceptance Criteria

1. WHEN un desarrollador ejecuta el comando de generación de ruta con nombre y path THEN Ruffus SHALL crear un archivo de ruta con handlers para los métodos HTTP especificados
2. WHEN un desarrollador especifica métodos HTTP (GET, POST, PUT, DELETE) THEN Ruffus SHALL generar handlers específicos para cada método en la ruta
3. WHEN Ruffus genera una ruta THEN Ruffus SHALL validar que el path de la ruta sigue el formato correcto y no contiene caracteres inválidos
4. WHEN Ruffus crea una nueva ruta THEN Ruffus SHALL integrar la ruta en el archivo de configuración de routing principal del framework
5. WHEN un desarrollador genera una ruta con dependencias de servicio THEN Ruffus SHALL incluir la inyección de dependencias apropiada en el handler

### Requirement 3

**User Story:** Como desarrollador preocupado por la seguridad, quiero generar middleware para autenticación y validación, para poder proteger mis endpoints sin implementar la lógica de seguridad desde cero.

#### Acceptance Criteria

1. WHEN un desarrollador ejecuta el comando de generación de guard con un tipo específico THEN Ruffus SHALL crear middleware con la lógica de interceptación apropiada
2. WHEN un desarrollador especifica el tipo de guard (auth, validation, logging) THEN Ruffus SHALL generar el código específico para ese tipo de middleware
3. WHEN Ruffus genera un guard THEN Ruffus SHALL incluir manejo de errores apropiado para casos de autenticación fallida o validación incorrecta
4. WHEN un guard es generado THEN Ruffus SHALL proporcionar ejemplos de cómo aplicar el middleware a rutas específicas

### Requirement 4

**User Story:** Como desarrollador que construye features completas, quiero generar módulos completos con servicios, rutas y guards integrados, para poder crear funcionalidades end-to-end rápidamente.

#### Acceptance Criteria

1. WHEN un desarrollador ejecuta el comando de generación de módulo THEN Ruffus SHALL crear una estructura de directorio completa con subdirectorios para servicios, rutas y guards
2. WHEN un desarrollador especifica componentes para el módulo THEN Ruffus SHALL generar todos los componentes especificados (servicios, rutas, guards) de forma integrada
3. WHEN Ruffus genera un módulo THEN Ruffus SHALL crear archivos mod.rs en cada nivel para exportar correctamente todos los componentes
4. WHEN un módulo es generado THEN Ruffus SHALL integrar el módulo en la estructura principal del proyecto actualizando los imports necesarios
5. WHEN un desarrollador genera un módulo con flag de CRUD THEN Ruffus SHALL incluir operaciones create, read, update y delete en el servicio generado

### Requirement 5

**User Story:** Como desarrollador que trabaja con diferentes frameworks, quiero que la herramienta detecte automáticamente mi framework web, para que genere código compatible sin configuración manual.

#### Acceptance Criteria

1. WHEN Ruffus analiza un proyecto existente THEN Ruffus SHALL detectar el framework web examinando las dependencias en Cargo.toml
2. WHEN Ruffus no puede detectar el framework por dependencias THEN Ruffus SHALL analizar la estructura de archivos y patrones de imports comunes
3. WHEN Ruffus detecta múltiples frameworks en el proyecto THEN Ruffus SHALL solicitar al usuario que especifique cuál framework utilizar
4. WHEN un desarrollador inicializa un nuevo proyecto con un framework específico THEN Ruffus SHALL configurar el proyecto con la estructura y dependencias apropiadas para ese framework
5. WHEN Ruffus detecta un framework no soportado THEN Ruffus SHALL permitir al usuario especificar plantillas personalizadas para ese framework

### Requirement 6

**User Story:** Como desarrollador que necesita personalización, quiero usar plantillas customizables para la generación de código, para poder adaptar el código generado a los estándares de mi equipo.

#### Acceptance Criteria

1. WHEN Ruffus genera cualquier componente THEN Ruffus SHALL utilizar el motor de plantillas Handlebars para renderizar el código
2. WHEN un desarrollador proporciona plantillas personalizadas en el directorio de configuración THEN Ruffus SHALL utilizar esas plantillas en lugar de las plantillas built-in
3. WHEN Ruffus renderiza una plantilla THEN Ruffus SHALL inyectar variables de contexto incluyendo nombre del componente, timestamp, autor y variables personalizadas
4. WHEN una plantilla contiene helpers de conversión de caso (snake_case, PascalCase, kebab-case) THEN Ruffus SHALL aplicar la conversión correctamente a los nombres de componentes
5. WHEN un desarrollador configura variables personalizadas en el archivo de configuración THEN Ruffus SHALL incluir esas variables en el contexto de renderizado de todas las plantillas
