# Implementation Plan

- [x] 1. Set up project structure and core dependencies
  - Create Cargo.toml with necessary dependencies (clap, handlebars, serde, tokio, thiserror)
  - Set up basic project directory structure (src/cli, src/generators, src/templates, etc.)
  - Create main.rs with basic CLI entry point
  - _Requirements: 5.2, 6.1_

- [ ] 2. Implement core data models and error types
  - [ ] 2.1 Create error type hierarchy with thiserror
    - Define CliError, GenerationError, TemplateError, FileSystemError enums
    - Implement Display and Error traits for all error types
    - _Requirements: 1.5, 2.3, 3.4_

  - [ ] 2.2 Implement configuration data models
    - Create ProjectConfig and UserConfig structs with serde
    - Define Framework enum with supported web frameworks
    - Create ComponentType and GenerationRequest structs
    - _Requirements: 5.1, 5.3, 6.2_

  - [ ] 2.3 Create template and context models
    - Implement Template struct with metadata
    - Create TemplateContext with dynamic variables
    - Define TemplateVariable for template validation
    - _Requirements: 6.4, 6.5_

- [ ] 3. Build CLI interface and command parsing
  - [ ] 3.1 Implement main CLI structure with clap
    - Create Commands enum with Init, Generate, Config subcommands
    - Define GenerateComponent subcommand with Service, Route, Guard, Module options
    - Add argument parsing for names, paths, and options
    - _Requirements: 1.1, 2.1, 3.1, 4.1_

  - [ ] 3.2 Create command handlers and routing
    - Implement command dispatch logic in main.rs
    - Create handler functions for each command type
    - Add input validation for component names and paths
    - _Requirements: 1.3, 2.3, 3.3, 4.3_

- [ ] 4. Implement framework detection system
  - [ ] 4.1 Create framework detector trait and implementations
    - Define FrameworkDetector trait with detect method
    - Implement detectors for Axum, Actix-web, Warp, Rocket
    - Create ProjectAnalyzer that orchestrates detection
    - _Requirements: 5.1, 5.5_

  - [ ] 4.2 Build Cargo.toml and file structure analysis
    - Parse Cargo.toml to identify framework dependencies
    - Analyze project structure patterns for each framework
    - Implement fallback detection using common import patterns
    - _Requirements: 5.1, 5.2_

- [ ] 5. Create template engine and management system
  - [ ] 5.1 Implement core template engine with Handlebars
    - Set up Handlebars instance with custom helpers
    - Create TemplateEngine struct with registration methods
    - Implement template rendering with context variables
    - _Requirements: 6.3, 6.4_

  - [ ] 5.2 Build template provider system
    - Create TemplateProvider trait for different sources
    - Implement BuiltInTemplateProvider with embedded templates
    - Create CustomTemplateProvider for user templates
    - _Requirements: 6.1, 6.2_

  - [ ] 5.3 Add template validation and variable injection
    - Implement template syntax validation
    - Create helper functions for case conversion (snake_case, PascalCase)
    - Add timestamp, author, and custom variable injection
    - _Requirements: 6.3, 6.4_

- [ ] 6. Implement file system operations and management
  - [ ] 6.1 Create file system manager with safety checks
    - Implement FileSystemManager with path validation
    - Add atomic file operations with rollback capability
    - Create directory structure validation
    - _Requirements: 1.3, 1.5, 2.3, 4.2_

  - [ ] 6.2 Build module file update system
    - Implement mod.rs parsing and updating logic
    - Create export statement generation for new components
    - Add integration logic for main routing files
    - _Requirements: 1.4, 2.4, 4.4_

- [ ] 7. Create web service generators for rapid development
  - [ ] 7.1 Implement service layer generator
    - Create business logic services with CRUD operations
    - Generate service traits and implementations
    - Add dependency injection patterns and error handling
    - Implement automatic service registration
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [ ] 7.2 Implement REST API route generator
    - Create HTTP endpoints with standard REST patterns
    - Generate route handlers with request/response DTOs
    - Add automatic OpenAPI/Swagger documentation
    - Implement route registration and path parameters
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

  - [ ] 7.3 Implement middleware and guard generator
    - Create authentication and authorization middleware
    - Generate request validation and sanitization guards
    - Add logging, CORS, and rate limiting middleware
    - Implement custom middleware templates
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ] 7.4 Implement complete web service module generator
    - Generate full-featured web service modules
    - Create integrated service + routes + middleware
    - Add database models and migration templates
    - Implement testing scaffolds and examples
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 8. Build web service templates for popular frameworks
  - [ ] 8.1 Create Axum web service templates
    - Implement REST API service templates with async/await
    - Create route handlers with extractors and responses
    - Add middleware templates for auth, logging, CORS
    - Generate application state and dependency injection
    - _Requirements: 5.3, 5.4_

  - [ ] 8.2 Create Actix-web service templates
    - Implement web service templates with Actix patterns
    - Create route handlers with web::Json and web::Path
    - Add middleware templates for Actix ecosystem
    - Generate app configuration and service registration
    - _Requirements: 5.3, 5.4_

  - [ ] 8.3 Create Warp and Rocket service templates
    - Implement lightweight service templates for Warp
    - Create Rocket route templates with guards and fairings
    - Add framework-specific middleware patterns
    - Generate server configuration and startup code
    - _Requirements: 5.3, 5.4_

- [ ] 9. Implement configuration management system
  - [ ] 9.1 Create configuration loading and saving
    - Implement ProjectConfig loading from .ruffus.toml
    - Create UserConfig loading from home directory
    - Add configuration validation and defaults
    - _Requirements: 6.1, 6.2_

  - [ ] 9.2 Build configuration CLI commands
    - Implement config set/get commands
    - Add template path management commands
    - Create configuration validation commands
    - _Requirements: 6.5_

- [ ] 10. Add comprehensive error handling and logging
  - [ ] 10.1 Implement error propagation and user messages
    - Add context to all error types with helpful messages
    - Implement error recovery suggestions
    - Create user-friendly error display
    - _Requirements: 1.5, 2.3, 3.4_

  - [ ] 10.2 Add logging and debugging support
    - Implement structured logging with tracing
    - Add debug mode with verbose output
    - Create dry-run mode for testing operations
    - _Requirements: All requirements for debugging_

- [ ]* 11. Create comprehensive test suite
  - [ ]* 11.1 Write unit tests for core components
    - Test template engine with various inputs
    - Test framework detection with mock projects
    - Test file system operations with temporary directories
    - _Requirements: All requirements_

  - [ ]* 11.2 Create integration tests for generators
    - Test complete service generation workflow
    - Test route generation with different frameworks
    - Test module generation with multiple components
    - _Requirements: 1.1-1.5, 2.1-2.5, 3.1-3.5, 4.1-4.5_

  - [ ]* 11.3 Add end-to-end CLI tests
    - Test complete CLI workflows from command line
    - Test error scenarios and recovery
    - Test configuration management
    - _Requirements: All requirements_