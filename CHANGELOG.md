# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2024-11-28

### Added
- Enhanced documentation in lib.rs with comprehensive examples
- Added Quick Start example in crate-level documentation
- Added JSON API example in crate-level documentation
- Added Path Parameters example in crate-level documentation
- Added Middleware example in crate-level documentation
- Added docs.rs configuration in Cargo.toml
- Created DOCUMENTATION.md guide
- Created DOCS_RS_GUIDE.md quick reference

### Changed
- Updated lib.rs with detailed module documentation
- Fixed documentation examples to return Result types
- Improved code examples to be compilable and testable
- Updated doc tests (4 passing)

### Fixed
- Fixed unused variable warnings in src/request.rs
- Corrected documentation examples to match actual API

### Documentation
- All documentation examples now compile correctly
- Added comprehensive inline documentation
- Configured automatic docs.rs generation
- No functional code changes

## [0.1.1] - 2024-11-28

### Changed
- Updated README.md with comprehensive project information
- Added publication status and test coverage badges
- Improved installation instructions with Git repository option
- Added detailed publishing guide section
- Enhanced contributing section with specific test commands
- Added detailed roadmap with version milestones
- Added project statistics section
- Improved community and support information
- Added links to additional resources (CHANGELOG, CONTRIBUTING, design docs)
- Updated footer with publication status

### Documentation
- No code changes - documentation-only release
- All 107 tests still passing

## [0.1.0] - 2024-11-28

### Added

#### Core Features
- Initial release of Ruffus web framework
- Express.js-inspired API for Rust
- Full async/await support with Tokio runtime
- Built on Hyper 1.0 for high performance

#### Routing
- HTTP method routing (GET, POST, PUT, DELETE, PATCH)
- Path parameters with `:param` syntax
- Query parameter parsing
- Route matching and handler invocation
- Router support with path prefixes
- Nested router mounting

#### Request/Response
- Type-safe request handling
- JSON serialization/deserialization with Serde
- Custom headers support
- Status code management
- Request body parsing
- URL decoding for path parameters

#### Middleware
- Middleware system with execution order preservation
- Request modification propagation
- Early return support
- Error handling middleware
- Router-scoped middleware

#### Extractors
- `Path<T>` extractor for path parameters
- `Json<T>` extractor for JSON bodies
- `Query<T>` extractor for query parameters
- Type-safe data extraction with automatic deserialization

#### Error Handling
- Comprehensive error types
- Automatic error-to-HTTP response conversion
- 404 handling for non-existent routes
- 405 handling for wrong HTTP methods
- 400 handling for invalid JSON
- 500 handling for internal errors

#### Testing
- 43 property-based tests using QuickCheck
- 8 unit tests for core functionality
- 56 documentation tests
- Full test coverage for all correctness properties

#### Documentation
- Comprehensive API documentation
- Usage examples for all major features
- 6 example applications:
  - Basic server
  - JSON API
  - Middleware usage
  - Router organization
  - Extractors
  - Full REST API

### Technical Details
- Minimum Rust version: 1.70.0 (2021 edition)
- Dependencies:
  - tokio 1.35 (async runtime)
  - hyper 1.0 (HTTP server)
  - serde 1.0 (serialization)
  - async-trait 0.1 (async traits)
  - bytes 1.5 (efficient byte handling)

### Performance
- Zero-copy request parsing where possible
- Efficient routing with pattern matching
- Async I/O for non-blocking operations
- Minimal allocations in hot paths

[0.1.2]: https://github.com/holasoymalva/ruffus/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/holasoymalva/ruffus/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/holasoymalva/ruffus/releases/tag/v0.1.0
