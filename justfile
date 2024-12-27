release := '\ '

default: test

build release=release:
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo build --workspace --release;
        cargo test --workspace --no-run --release;
    else
        cargo build --workspace;
        cargo test --workspace --no-run;
    fi

test release=release: (build release)
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo pretty-test --workspace --release;
    else
        cargo pretty-test --workspace;
    fi

test_integration release=release: (build release)
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo pretty-test --workspace --release -- --include-ignored ;
    else
        cargo pretty-test --workspace -- --include-ignored ;
    fi

install: (test_integration release)
    cargo install --path .

clean:
    cargo clean

format:
    cargo fmt --check

clippy:
    cargo clippy -- -W clippy::pedantic
