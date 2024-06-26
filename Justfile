# This justfile is for the contrbutors of this project, not for the end user.
#
# Requirements for this justfile:
# - Linux distribution
# - just (Of course) <https://github.com/casey/just>
# - cargo (For the build and tests) <https://doc.rust-lang.org/cargo/getting-started/installation.html>

set shell := ["/usr/bin/bash", "-c"]

JUST_EXECUTABLE := "just -u -f " + justfile()
header := "Available tasks:\n"
# Get the MSRV from the Cargo.toml
msrv := `cat Cargo.toml | grep "rust-version" | sed 's/.*"\(.*\)".*/\1/'`


_default:
    @{{JUST_EXECUTABLE}} --list-heading "{{header}}" --list

# Run the CI
@ci: && msrv
    cargo build --all-targets
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings

# Check that the current MSRV is correct
@msrv:
    rustup toolchain install {{msrv}}
    echo "Checking MSRV ({{msrv}})"
    cargo +{{msrv}} check -q --workspace
    echo "MSRV is correct"

run:
    docker-compose up -d db
    RUST_LOG=debug cargo run -p oxidetalis -- --config config.toml

[private]
alias r := run