release := '\ '

default: test

build release=release:
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo build --workspace --release;
        cargo nextest run --no-run --release;
    else
        cargo build --workspace;
        cargo nextest run --no-run;
    fi

test release=release: (build release)
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo nextest run --release;
    else
        cargo nextest run;
    fi

test_integration release=release: (build release)
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo nextest run --release -- --include-ignored ;
    else
        cargo nextest run -- --include-ignored ;
    fi

install: (test_integration "release")
    cargo install --path .

clean:
    cargo clean

format:
    cargo fmt --check

clippy:
    cargo clippy -- -W clippy::pedantic

audit:
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo audit --deny unsound --deny yanked --deny unmaintained || cargo-audit --deny unsound --deny yanked --deny unmaintained

