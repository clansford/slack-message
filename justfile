release := '\ '

default: test

build release=release:
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo build --workspace --release;
    else
        cargo build --workspace;
    fi

test release=release: (build release)
    #!/usr/bin/env bash
    if [[ {{release}} == "release" ]]; then
        cargo test --workspace --release;
    else
        cargo test --workspace;
    fi

install: (test release)
    cargo install --path .

clean:
    cargo clean

format:
    cargo fmt --check

clippy:
    cargo clippy -- -W clippy::pedantic
