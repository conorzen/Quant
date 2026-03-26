default:
    just --list

build:
    cargo build --workspace

test:
    cargo test --workspace

test-verbose:
    cargo test --workspace -- --nocapture