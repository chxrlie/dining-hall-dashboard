# Dining Hall Dashboard Development Guide

## 🛠️ Getting Started

This guide will help you set up a development environment for the Dining Hall Dashboard application.

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (version 1.70 or higher) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Git** - [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- **Cargo** (comes with Rust installation)

### Installation Steps

1. **Clone the repository**

   ```bash
   git clone https://github.com/chxrlie/dining-hall-dashboard.git
   cd dining-hall-dashboard
   ```

2. **Build the project**

   ```bash
   cargo build
   ```

3. **Run the application**

   ```bash
   cargo run
   ```

4. **Access the application**
   Open your browser to `http://localhost:8080`

### Project Structure

```
dining-hall-dashboard/
├── src/                 # Rust source code
│   ├── main.rs         # Application entry point
│   ├── auth.rs         # Authentication system
│   ├── handlers.rs     # HTTP request handlers
│   ├── storage.rs      # JSON storage system
│   └── scheduler.rs    # Automated scheduling system
├── data/               # JSON data files
│   ├── menu_items.json
│   ├── notices.json
│   ├── admin_users.json
│   ├── menu_presets.json
│   └── menu_schedules.json
├── templates/          # HTML templates
│   ├── base.html
│   ├── menu.html
│   ├── admin/
│   └── partials/
├── static/             # Static assets
│   ├── css/
│   └── images/
├── assets/             # Branding assets
├── tests/              # Integration and unit tests
├── Cargo.toml          # Rust package manifest
├── Cargo.lock          # Cargo lock file
├── .gitignore          # Git ignore file
└── README.md           # Project documentation
```

## 🧪 Testing

The project includes both unit tests and integration tests to ensure code quality and functionality.

### Running Tests

To run all tests:

```bash
cargo test
```

To run tests with output:

```bash
cargo test -- --nocapture
```

### Test Structure

Tests are organized in the `tests/` directory and within the source files themselves:

- **Unit tests**: Located within each source file (e.g., `src/storage.rs`)
- **Integration tests**: Located in the `tests/` directory

### Writing Tests

When adding new features, please include appropriate tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem {
            id: Uuid::new_v4(),
            name: "Test Item".to_string(),
            category: MenuCategory::Mains,
            description: "A test menu item".to_string(),
            allergens: vec![],
            is_available: true,
        };

        assert_eq!(item.name, "Test Item");
        assert_eq!(item.category, MenuCategory::Mains);
    }
}
```

## 📦 Dependencies

### Core Dependencies

| Crate         | Version | Purpose            |
| ------------- | ------- | ------------------ |
| actix-web     | 4.11.0  | Web framework      |
| serde         | 1.0.219 | Serialization      |
| tera          | 1.20.0  | Template engine    |
| argon2        | 0.5.3   | Password hashing   |
| serde_json    | 1.0.108 | JSON support       |
| chrono        | 0.4.31  | Date/time handling |
| uuid          | 1.7.0   | UUID generation    |
| actix-session | 0.8.0   | Session management |

### Development Dependencies

| Crate       | Version | Purpose             |
| ----------- | ------- | ------------------- |
| actix-rt    | 2.9.0   | Runtime             |
| actix-files | 0.6.6   | Static file serving |
| env_logger  | 0.10.1  | Logging             |
| actix-cors  | 0.7.1   | CORS support        |
| tokio       | 1.0     | Async runtime       |
| log         | 0.4     | Logging facade      |

### Managing Dependencies

To add a new dependency:

1. Add it to `Cargo.toml` in the `[dependencies]` section
2. Run `cargo build` to update `Cargo.lock`
3. Import the crate in your source files as needed

## 🎨 Code Style and Conventions

### Rust Style Guide

We follow the official [Rust Style Guide](https://github.com/rust-lang/rfcs/blob/master/style-guide/README.md) with some project-specific conventions:

1. **Naming conventions**:

   - Structs and enums: `PascalCase`
   - Functions and variables: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`

2. **Formatting**:

   - Use `cargo fmt` to automatically format code
   - Maximum line length: 100 characters
   - Use spaces, not tabs (4 spaces per indent)

3. **Documentation**:
   - All public functions should have doc comments
   - Use `//!` for module-level documentation
   - Use `///` for function/item-level documentation

### Example Code Style

```rust
/// Represents a menu item in the dining hall
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    /// Unique identifier for the menu item
    pub id: Uuid,

    /// Name of the menu item
    pub name: String,

    /// Category of the menu item
    pub category: MenuCategory,

    /// Detailed description of the menu item
    pub description: String,

    /// List of allergens present in the item
    pub allergens: Vec<String>,

    /// Availability status of the item
    pub is_available: bool,
}

impl MenuItem {
    /// Creates a new menu item with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the menu item
    /// * `category` - The category of the menu item
    /// * `description` - A detailed description of the item
    ///
    /// # Returns
    ///
    /// A new `MenuItem` instance
    pub fn new(name: String, category: MenuCategory, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            category,
            description,
            allergens: Vec::new(),
            is_available: true,
        }
    }
}
```

## 🌿 Git Workflow

We follow the Gitflow workflow for development:

### Branch Naming Conventions

- `main` - Production-ready code
- `develop` - Development branch
- `feature/*` - New features (e.g., `feature/menu-scheduling`)
- `bugfix/*` - Bug fixes (e.g., `bugfix/login-issue`)
- `hotfix/*` - Critical production fixes
- `release/*` - Release preparation

### Commit Message Guidelines

Follow the conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks

Example:

```
feat(auth): add password reset functionality

Implement password reset via email with token-based verification

Closes #123
```

## 🤝 Contributing

We welcome contributions from the community! Here's how you can help:

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
4. **Write tests** for your changes
5. **Ensure all tests pass**
   ```bash
   cargo test
   ```
6. **Format your code**
   ```bash
   cargo fmt
   ```
7. **Commit your changes**
   ```bash
   git commit -am "feat: add your feature description"
   ```
8. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```
9. **Create a Pull Request**

### Code Review Process

All submissions require review. We use GitHub Pull Requests for this process:

1. PRs must pass all CI checks
2. Code must be reviewed by at least one maintainer
3. All discussions must be resolved before merging
4. Squash and merge is preferred for clean history

### Reporting Issues

Before creating an issue, please check if it already exists. When reporting a bug, include:

- A clear title and description
- Steps to reproduce the issue
- Expected vs actual behavior
- Screenshots if applicable
- Environment information (OS, Rust version, etc.)

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/) (SemVer):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backward-compatible functionality
- **PATCH** version for backward-compatible bug fixes

### Release Steps

1. **Update version in Cargo.toml**
2. **Update CHANGELOG.md**
3. **Create and push a tag**
   ```bash
   git tag -a v1.2.3 -m "Release version 1.2.3"
   git push origin v1.2.3
   ```
4. **Create GitHub Release**
5. **Publish to crates.io** (if applicable)
   ```bash
   cargo publish
   ```

## 🛠️ Development Tools

### Recommended IDE/Editor Setup

- **VSCodium** with Rust extensions:
  - rust-analyzer
  - CodeLLDB (for debugging)
  - Even Better TOML
- **IntelliJ IDEA** with Rust plugin
- **Vim/Neovim** with rust.vim

### Useful Cargo Commands

```bash
# Check for errors without building
cargo check

# Format code
cargo fmt

# Check code style
cargo clippy

# Run with specific features
cargo run --features "debug"

# Build for release
cargo build --release

# Generate documentation
cargo doc --open
```

### Debugging

To enable debug logging:

```bash
RUST_LOG=debug cargo run
```

To debug with breakpoints in VS Code:

1. Install CodeLLDB extension
2. Set breakpoints in your code
3. Run with debug configuration

## 📚 Additional Resources

- [Actix-web Documentation](https://actix.rs/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Serde Documentation](https://serde.rs/)
- [Tera Template Engine](https://tera.netlify.app/)

## 🆘 Getting Help

If you need help with development:

1. Check the existing documentation
2. Look at existing code examples
3. File an issue on GitHub
