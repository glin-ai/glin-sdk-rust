# Contributing to GLIN SDK - Rust

Thank you for your interest in contributing to the GLIN SDK for Rust! This document provides guidelines for contributing to this project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Process](#development-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## 📜 Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful and constructive in all interactions.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo
- Git

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:

```bash
git clone https://github.com/YOUR-USERNAME/glin-sdk-rust.git
cd glin-sdk-rust
```

3. Add upstream remote:

```bash
git remote add upstream https://github.com/glin-ai/glin-sdk-rust.git
```

### Build and Test

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## 🤝 How to Contribute

### Reporting Bugs

- Check existing issues to avoid duplicates
- Use the bug report template
- Include:
  - Description of the issue
  - Steps to reproduce
  - Expected vs actual behavior
  - Environment details (OS, Rust version)
  - Code samples or error messages

### Suggesting Enhancements

- Check existing issues and discussions
- Use the feature request template
- Clearly describe:
  - The problem you're trying to solve
  - Your proposed solution
  - Alternative solutions considered
  - Impact on existing functionality

### Good First Issues

Look for issues labeled `good-first-issue` or `help-wanted` for beginner-friendly contributions.

## 🔧 Development Process

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

- Write clear, concise commit messages
- Follow conventional commits format:
  ```
  feat: add contract metadata caching
  fix: correct balance query encoding
  docs: update installation instructions
  test: add tests for account management
  refactor: simplify RPC client creation
  ```

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p glin-client

# Run with all features
cargo test --all-features
```

### 4. Format and Lint

```bash
# Format code
cargo fmt

# Check for linter warnings
cargo clippy --all-targets --all-features -- -D warnings
```

### 5. Update Documentation

- Update README.md if needed
- Add doc comments to public APIs
- Update examples if API changes

## 💻 Coding Standards

### Rust Style Guide

- Follow the [official Rust style guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` for automatic formatting
- Maximum line length: 100 characters

### Documentation

- All public APIs must have doc comments
- Include examples in doc comments when possible:

```rust
/// Creates a new client connection to GLIN Network
///
/// # Examples
///
/// ```no_run
/// use glin_client::create_client;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = create_client("wss://testnet.glin.ai").await?;
///     Ok(())
/// }
/// ```
pub async fn create_client(rpc_url: &str) -> Result<GlinClient> {
    // implementation
}
```

### Error Handling

- Use `anyhow::Result` for application code
- Use `thiserror` for library errors
- Provide context for errors:

```rust
client.connect(url)
    .await
    .context("Failed to connect to GLIN Network")?;
```

### Naming Conventions

- Types: `PascalCase`
- Functions/variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

## 🧪 Testing Guidelines

### Unit Tests

- Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_creation() {
        let account = get_dev_account("alice");
        assert!(account.is_ok());
    }
}
```

### Integration Tests

- Place in `tests/` directory
- Test public APIs only
- Use realistic scenarios

### Test Coverage

- Aim for >80% coverage for new code
- All public APIs should have tests
- Test both success and error paths

## 📝 Pull Request Process

### Before Submitting

- [ ] Code builds without errors
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Commit messages follow conventions

### PR Template

Use the following template:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested your changes

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or noted in description)
```

### Review Process

1. Automated checks must pass (CI/CD)
2. At least one maintainer approval required
3. Address review feedback
4. Squash commits if requested
5. Maintainer will merge when ready

## 🔖 Release Process

Releases are managed by maintainers.

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes

### Changelog

All notable changes are documented in CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format.

## 🐛 Security Issues

**DO NOT** open public issues for security vulnerabilities.

Instead, email security@glin.ai with:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will respond within 48 hours.

## 📧 Contact

- **General questions**: dev@glin.ai
- **Discord**: https://discord.gg/glin-ai
- **GitHub Discussions**: https://github.com/glin-ai/glin-sdk-rust/discussions

## 📜 License

By contributing, you agree that your contributions will be licensed under the Apache-2.0 License.

---

Thank you for contributing to GLIN SDK! 🎉
