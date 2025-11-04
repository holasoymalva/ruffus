# 🦀 Ruffus

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/workflow/status/holasoymalva/ruffus/CI)](https://github.com/holasoymalva/ruffus/actions)
[![Crates.io](https://img.shields.io/crates/v/ruffus.svg)](https://crates.io/crates/ruffus)

**Ruffus** is a powerful CLI tool for generating web components in Rust projects. It provides scaffolding and code generation capabilities for popular Rust web frameworks like Axum, Actix-web, Warp, and Rocket.

## ✨ Features

- 🚀 **Multi-Framework Support**: Works with Axum, Actix-web, Warp, Rocket, and custom frameworks
- 🎯 **Smart Code Generation**: Generate services, routes, guards/middleware, and complete modules
- 🔧 **Configurable Templates**: Customize templates or use built-in ones
- 📁 **Project Structure**: Automatically organizes your code with best practices
- ⚡ **Fast & Reliable**: Built with Rust for maximum performance
- 🎨 **Template Engine**: Powered by Handlebars with custom helpers

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
# Initialize a new project
ruffus init --framework axum --name my-web-app

# Generate a service
ruffus generate service UserService --module users

# Generate a route with multiple HTTP methods
ruffus generate route UserRoutes --methods GET,POST --path "/api/users"

# Generate middleware/guard
ruffus generate guard AuthGuard --guard-type auth

# Generate a complete module
ruffus generate module UserModule --components service,route,guard
```

## 📖 Documentation

### Commands

#### `ruffus init`
Initialize a new web project with the specified framework.

```bash
ruffus init --framework <FRAMEWORK> --name <PROJECT_NAME>
```

**Options:**
- `--framework, -f`: Target framework (axum, actix-web, warp, rocket, or custom)
- `--name, -n`: Project name

#### `ruffus generate`
Generate components for your web application.

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

Ruffus organizes your project with a clean, scalable structure:

```
my-web-app/
├── src/
│   ├── main.rs
│   ├── routes/          # HTTP route handlers
│   ├── services/        # Business logic services
│   ├── guards/          # Middleware and guards
│   ├── models/          # Data models
│   └── config/          # Configuration
├── templates/           # Custom templates (optional)
├── .ruffus.toml        # Project configuration
└── Cargo.toml
```

## ⚙️ Configuration

### Project Configuration (`.ruffus.toml`)

```toml
[project]
framework = "axum"
name = "my-web-app"
author = "Your Name"

[structure]
services_dir = "services"
routes_dir = "routes"
guards_dir = "guards"
models_dir = "models"

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

### Axum Example

```bash
# Initialize Axum project
ruffus init --framework axum --name todo-api

# Generate user service
ruffus generate service UserService --module users

# Generate authentication middleware
ruffus generate guard AuthGuard --guard-type auth

# Generate user routes
ruffus generate route UserRoutes --path "/api/users" --methods GET,POST,PUT,DELETE
```

### Actix-web Example

```bash
# Initialize Actix-web project
ruffus init --framework actix-web --name blog-api

# Generate complete blog module
ruffus generate module BlogModule --components service,route,guard
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