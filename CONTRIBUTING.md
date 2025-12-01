# Contributing to protofolio

Thank you for your interest in contributing to `protofolio`! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.80 or later
- `cargo` (comes with Rust)

### Building

```bash
cargo build
```

### Testing

Run all tests:

```bash
cargo test
```

Run specific test suites:

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration_test

# Protocol tests
cargo test --test protocol_tests

# Schema cache tests
cargo test --test schema_cache_tests

# Error handling tests
cargo test --test error_handling_tests
```

### Macro Error Tests (trybuild)

The project uses `trybuild` to test compile-time error messages. These tests verify that macros produce helpful error messages.

**Important**: When you modify macro error messages, the expected stderr files may need to be updated:

```bash
cargo test --test macro_error_tests
```

If tests fail with "successfully created new stderr files", you need to:

1. Review the generated `.stderr` files in `protofolio/tests/macro_error_tests/ui/`
2. If the new error messages are correct, move them from `wip/` to the test directory
3. Commit the updated stderr files

## Code Style

- Follow standard Rust formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- The project uses `rust-version = "1.80"` - ensure your code is compatible

## Adding New Features

### Adding a New Protocol

1. Create a new file in `protofolio/src/protocol/` (e.g., `amqp.rs`)
2. Define protocol constants and types following the pattern in `nats.rs`, `kafka.rs`, or `mqtt.rs`
3. Add protocol-specific bindings in `protocol/bindings.rs`
4. Export the protocol in `protocol/mod.rs`
5. Update `protocol/mod.rs::validate_protocol()` to include the new protocol
6. Add tests in `tests/protocol_tests.rs`
7. Update documentation (README.md, PROTOCOLS.md)

### Adding New Validation Rules

1. Add validation logic to `protofolio/src/validation/validator.rs`
2. Add a new `ValidationError` variant in `error.rs` if needed
3. Add tests in `validation/validator.rs` (unit tests) or `tests/error_handling_tests.rs` (integration tests)
4. Update documentation

### Adding Builder Methods

1. Add the method to `AsyncApiBuilder` in `builder/builder.rs`
2. Follow the fluent API pattern (return `Self`)
3. Add tests in `builder/builder.rs`
4. Add examples to the method documentation

## Documentation

### When Adding New Features

- **README.md**: Update relevant sections (Features, Status, Examples)
- **API Documentation**: Add rustdoc examples to all public functions/types
- **ARCHITECTURE.md**: Update if architecture changes
- **PROTOCOLS.md**: Add protocol-specific documentation if adding a protocol

### Documentation Standards

- All public functions must have rustdoc comments with examples
- Examples should be compilable (use `no_run` if they can't be run in tests)
- Include error handling examples where relevant
- Link to related types/functions using markdown links

### Building Documentation

```bash
cargo doc --open
```

This will build and open the documentation in your browser. Check that:

- All examples compile (or are marked `no_run`)
- Links work correctly
- Documentation is clear and helpful

## Testing Guidelines

### Unit Tests

- Co-locate tests with the code they test (using `#[cfg(test)]` modules)
- Test both success and error cases
- Test edge cases

### Integration Tests

- Place in `tests/` directory
- Test complete workflows (macro → spec → validation → serialization)
- Test error handling paths

### Macro Tests

- Use `trybuild` for compile-time error testing
- Test that helpful error messages are generated
- Update stderr files when error messages change

## Commit Messages

Follow conventional commit format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions/changes
- `refactor`: Code refactoring
- `perf`: Performance improvements

Examples:

```
feat(protocol): add AMQP protocol support

docs(api): add examples to AsyncApi trait methods

fix(validation): improve error messages for invalid channels
```

## CI/CD Pipeline

This project uses GitHub Actions for continuous integration. The CI pipeline runs on every push and pull request and includes:

- **Check**: Code formatting and compilation checks
- **Test**: Runs all tests on stable Rust and MSRV (1.80)
- **Clippy**: Lint checks with all warnings treated as errors
- **Security**: Runs `cargo-deny` to check for vulnerabilities and license compliance
- **Documentation**: Builds documentation to ensure it compiles correctly
- **Coverage**: Generates code coverage reports using `cargo-llvm-cov`

### Running CI Checks Locally

Before submitting a PR, you can run the same checks locally:

```bash
# Format check
cargo fmt --all -- --check

# Compilation check
cargo check --workspace --all-targets

# Run tests
cargo test --workspace --all-targets

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings

# Security and license checks (requires cargo-deny)
cargo install cargo-deny
cargo deny check

# Generate coverage report (requires cargo-llvm-cov)
cargo install cargo-llvm-cov
cargo llvm-cov --workspace --all-targets --html
```

### Code Coverage

Code coverage is automatically generated and uploaded to Codecov (if configured). To generate coverage locally:

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate HTML coverage report
cargo llvm-cov --workspace --all-targets --html --output-dir coverage

# Open coverage/index.html in your browser
```

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes
3. Add tests for new functionality
4. Update documentation
5. Ensure all tests pass: `cargo test --workspace`
6. Run clippy: `cargo clippy --workspace --all-targets -- -D warnings`
7. Format code: `cargo fmt --all`
8. Ensure CI passes (all checks will run automatically on PR)
9. Create a pull request with a clear description

## Questions?

If you have questions or need help, please:
- Open an issue on [GitHub](https://github.com/rifmj/protofolio/issues)
- Contact the maintainer: **jum.rifm@gmail.com**
