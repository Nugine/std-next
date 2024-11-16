dev:
    just fmt
    just lint
    just test
    just unstable-test
    just miri

fmt:
    cargo fmt

lint:
    cargo clippy --all-features

test:
    cargo test --no-default-features
    cargo test --no-default-features --features alloc
    cargo test --no-default-features --features std

unstable-test:
    cargo test --no-default-features --features unstable
    cargo test --no-default-features --features unstable,alloc
    cargo test --no-default-features --features unstable,std

miri:
    cargo +nightly miri test --all-features

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features

ci:
    cargo fmt --all --check
    cargo clippy --all-features -- -D warnings
    just test
    just unstable-test
    just miri
