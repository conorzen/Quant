default:
    just --list

build:
    cargo build --workspace

test:
    cargo test --workspace

test-verbose:
    cargo test --workspace -- --nocapture

bench:
   cargo bench --workspace

bench-report:
    cargo bench --workspace
    open target/criterion/report/index.html