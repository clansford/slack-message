release := '\ '
target := shell("rustc -vV | sed -n 's|host: ||p'")
var_one := '\ '

default: test

hello var_one=var_one target=target:
    #!/usr/bin/env bash
    set -euxo pipefail
    echo {{var_one}}
    echo {{target}}


build release=release target=target:
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo build --workspace --release --target {{target}};
        cargo test --workspace --no-run --release --target {{target}};
    else
        cargo build --workspace --target {{target}};
        cargo test --workspace --no-run --target {{target}};
    fi

test release=release target=target: (build release target)
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo pretty-test --workspace --release --target {{target}};
    else
        cargo pretty-test --workspace --target {{target}};
    fi

test_integration release=release target=target: (build release)
    #!/usr/bin/env bash
    set -euxo pipefail
    if [[ {{release}} == "release" ]]; then
        cargo pretty-test --workspace --release -- --include-ignored  --target {{target}};
    else
        cargo pretty-test --workspace -- --include-ignored  --target {{target}};
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

