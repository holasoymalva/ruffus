# 🦀 Ruffus

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/workflow/status/holasoymalva/ruffus/CI)](https://github.com/holasoymalva/ruffus/actions)
[![Crates.io](https://img.shields.io/crates/v/ruffus.svg)](https://crates.io/crates/ruffus)

**Ruffus** is a Flask-inspired CLI scaffolding tool for Rust web services. It provides rapid code generation and project scaffolding for building REST APIs and web services with popular Rust frameworks like Axum, Actix-web, Warp, and Rocket. Think of it as the "rails generate" or "flask cli" for Rust backend development.

## 🐍 Flask-Inspired Philosophy

Just like Flask makes Python web development simple and productive, Ruffus brings that same philosophy to Rust:

- **Convention over Configuration**: Sensible defaults that just work
- **Rapid Prototyping**: Go from idea to running API in minutes
- **Modular Design**: Generate only what you need, when you need it
- **Developer Friendly**: Intuitive commands that feel natural
- **Production Ready**: Generated code follows Rust best practices

## ✨ Features

- 🚀 **Flask-like Simplicity**: Intuitive CLI commands for rapid web service development
- 🎯 **REST API Scaffolding**: Generate complete CRUD services with one command
- 🔧 **Multi-Framework**: Support for Axum, Actix-web, Warp, Rocket out of the box
- 📁 **Clean Architecture**: Enforces service layer, routing, and middleware patterns
- ⚡ **Zero Config**: Start building immediately with sensible defaults
- 🎨 **Customizable Templates**: Extend with your own service patterns
- 🔍 **Smart Detection**: Automatically adapts to your existing project structure
- 📝 **Production Ready**: Generates idiomatic, testable, and maintainable code
- 🛡️ **Security First**: Built-in templates for auth, validation, and CORS
- 📊 **API Documentation**: Auto-generates OpenAPI/Swagger documentation

## 🚀 Quick Start

### Installation

```bash
# Install from crates.io (coming soon)
cargo install ruffus

# Or build from source
git clone https://github.com/holasoymalva/ruffus.git
cd ruffus
cargo install --path .
```

### Basic Usage

```bash
# Initialize a new web service project
ruffus init --framework axum --name todo-api

# Generate a complete CRUD service
ruffus generate service TodoService --crud --module todos

# Generate REST API endpoints
ruffus generate routes TodoRoutes --resource todos --methods GET,POST,PUT,DELETE

# Generate authentication middleware
ruffus generate middleware AuthMiddleware --type jwt

# Generate a complete web service module
ruffus generate module UserModule --with-auth --with-crud
```

## 📖 Documentation

### Commands

#### `ruffus init`
Initialize a new web backend project with the specified framework.

```bash
ruffus init --framework <FRAMEWORK> --name <PROJECT_NAME>
```

**Options:**
- `--framework, -f`: Target framework (axum, actix-web, warp, rocket, or custom)
- `--name, -n`: Project name

#### `ruffus generate`
Generate backend components for your web application.

##### Generate Service
```bash
ruffus generate service <NAME> [--module <MODULE>]
```

##### Generate Route
```bash
ruffus generate route <NAME> --path <PATH> [--methods <METHODS>]
```

##### Generate Guard/Middleware
```bash
ruffus generate guard <NAME> --guard-type <TYPE>
```

##### Generate Module
```bash
ruffus generate module <NAME> [--components <COMPONENTS>]
```

#### `ruffus config`
Manage configuration settings.

```bash
# Set configuration value
ruffus config set <KEY> <VALUE>

# Get configuration value
ruffus config get <KEY>

# List all configuration
ruffus config list
```

### Supported Frameworks

| Framework | Status | Version |
|-----------|--------|---------|
| **Axum** | ✅ Supported | 0.7+ |
| **Actix-web** | ✅ Supported | 4.0+ |
| **Warp** | ✅ Supported | 0.3+ |
| **Rocket** | ✅ Supported | 0.5+ |
| **Custom** | ✅ Supported | Any |

## 🏗️ Project Structure

Ruffus organizes your backend project with a clean, scalable structure:

```
my-api/
├── src/
│   ├── main.rs          # Application entry point
│   ├── routes/          # HTTP route handlers
│   ├── services/        # Business logic layer
│   ├── guards/          # Middleware and authentication
│   ├── models/          # Data models and DTOs
│   ├── config/          # Application configuration
│   └── lib.rs           # Library exports
├── templates/           # Custom code templates (optional)
├── .ruffus.toml        # Ruffus project configuration
└── Cargo.toml          # Rust project manifest
```

## ⚙️ Configuration

### Project Configuration (`.ruffus.toml`)

```toml
[project]
framework = "axum"
name = "my-api"
author = "Your Name"

[structure]
services_dir = "services"
routes_dir = "routes"
guards_dir = "guards"
models_dir = "models"

[generation]
auto_format = true
auto_imports = true

[templates]
custom_path = "./templates"
```

### User Configuration (`~/.ruffus/config.toml`)

```toml
[user]
default_author = "Your Name"
preferred_framework = "axum"

[editor]
auto_format = true
auto_import = true

[templates]
custom_paths = [
    "~/.ruffus/templates",
    "./project-templates"
]
```

## 🎨 Custom Templates

Ruffus uses Handlebars templates with custom helpers for code generation:

```handlebars
// Service template example
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{pascal_case component_name}}Request {
    // Add your fields here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct {{pascal_case component_name}}Response {
    // Add your response fields here
}

pub struct {{pascal_case component_name}}Service {
    // Add dependencies here
}

impl {{pascal_case component_name}}Service {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self, request: {{pascal_case component_name}}Request) -> Result<{{pascal_case component_name}}Response, ServiceError> {
        // Implement your business logic here
        todo!("Implement {{snake_case component_name}} logic")
    }
}
```

### Available Template Helpers

- `{{pascal_case name}}` - PascalCase conversion
- `{{snake_case name}}` - snake_case conversion  
- `{{kebab_case name}}` - kebab-case conversion

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/holasoymalva/ruffus.git
cd ruffus

# Install dependencies
cargo build

# Run tests
cargo test

# Run the CLI locally
cargo run -- --help
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run integration tests
cargo test --test integration
```

## 📝 Examples

### Building a Todo API with Axum

```bash
# Initialize new web service
ruffus init --framework axum --name todo-api
cd todo-api

# Generate complete todo service with CRUD operations
ruffus generate service TodoService --crud --module todos
# Creates: src/todos/service.rs with create, read, update, delete methods

# Generate REST API routes
ruffus generate routes TodoRoutes --resource todos
# Creates: src/todos/routes.rs with GET, POST, PUT, DELETE endpoints

# Generate JWT authentication middleware
ruffus generate middleware AuthMiddleware --type jwt
# Creates: src/middleware/auth.rs with JWT validation

# Run your web service
cargo run
```

### Building a Blog API with Actix-web

```bash
# Initialize blog service
ruffus init --framework actix-web --name blog-api

# Generate complete blog module with authentication
ruffus generate module BlogModule --with-auth --with-crud
# Creates complete blog service with posts, comments, and user auth

# Generate admin routes
ruffus generate routes AdminRoutes --path "/admin" --protected
# Creates protected admin endpoints

# Add rate limiting middleware
ruffus generate middleware RateLimitMiddleware --type rate-limit

# Generate API documentation routes
ruffus generate route DocsRoutes --path "/docs" --methods GET
```

## 🗺️ Roadmap

- [ ] **v0.2.0**: Enhanced template system with conditional generation
- [ ] **v0.3.0**: Database integration templates (SQLx, Diesel, SeaORM)
- [ ] **v0.4.0**: Testing utilities and test generation
- [ ] **v0.5.0**: Docker and deployment templates
- [ ] **v1.0.0**: Stable API and comprehensive documentation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Clap](https://github.com/clap-rs/clap) for CLI parsing
- Template engine powered by [Handlebars](https://github.com/sunng87/handlebars-rust)
- Inspired by modern web development tools and the Rust ecosystem

## 📞 Support

- 📖 [Documentation](https://github.com/holasoymalva/ruffus/wiki)
- 🐛 [Issue Tracker](https://github.com/holasoymalva/ruffus/issues)
- 💬 [Discussions](https://github.com/holasoymalva/ruffus/discussions)
- 📧 [Email Support](mailto:support@ruffus.dev)

---

<div align="center">
  <strong>Made with ❤️ by the Ruffus team</strong>
</div>