# Workspace Lints Configuration

This project uses centralized workspace lints (Rust 1.74+ feature) to ensure consistent linting across all crates in the workspace.

## Configuration

Workspace lints are defined in the root `Cargo.toml` under `[workspace.lints]` and inherited by all workspace members.

### Rust Lints

- `unsafe_code = "deny"` - Prevents unsafe code across the workspace
- `missing_docs = "warn"` - Warns on missing documentation
- `missing_debug_implementations = "warn"` - Warns on missing Debug implementations

### Rustdoc Lints

- `broken_intra_doc_links = "deny"` - Ensures all documentation links are valid
- `missing_doc_code_examples = "allow"` - Allows missing code examples (can be added incrementally)
- `private_intra_doc_links = "allow"` - Allows private items to link to each other

### Clippy Lints

- `pedantic = "warn"` - Enables pedantic lints (most useful lints)
- `nursery = "warn"` - Enables new/experimental lints
- `cargo = "warn"` - Enables cargo-specific lints
- `unwrap_used = "warn"` - Warns on `unwrap()` usage
- `expect_used = "warn"` - Warns on `expect()` usage

Several style-related lints are allowed for flexibility:
- `multiple_crate_versions` - Some dependencies may have multiple versions
- `missing_docs_in_private_items` - Can be fixed incrementally
- `result_large_err` - Fine for proc macros
- `too_many_lines` - Macro code is inherently long
- Style preferences (must_use_candidate, use_self, etc.)

## Usage

Each crate in the workspace inherits these lints by including:

```toml
[lints]
workspace = true
```

Individual crates can still override specific lints in their `lib.rs` or `main.rs` files if needed, but the workspace configuration provides a consistent baseline.

## Benefits

1. **Centralized Configuration**: All lint settings in one place
2. **Consistency**: All crates use the same lint rules
3. **Easy Updates**: Change lint rules once, apply everywhere
4. **Modern Rust**: Uses the latest Rust features (1.74+)

## Running Lints

```bash
# Check with cargo
cargo check --workspace

# Run clippy
cargo clippy --workspace --all-targets

# Fix automatically where possible
cargo clippy --workspace --all-targets --fix
```

