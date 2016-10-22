# Rust Build

Build scripts to aid Rust development.

## build.sh

This is a convenience build script which essentially wraps the following commands, but also provides basic diagnostics and coloured output:

```bash
# Inside the crate directory
# Syntax validation
cargo fmt -- --write-mode=diff 2>&1 && echo pass || echo fail

# Compile and test
cargo build
cargo test
```

## prepare_release.sh

Updates the changelog in a repository with the current date as the release date, then commits and tags the repository.

## prepare_next_version.sh

Updates the changelog and Cargo.toml in a repository to the next version.

```bash
# defaults to updating the minor version
rust_build/prepare_next_version.sh [path/to/repository] [--major|--minor|--patch]
```
