set -ex

main() {
    curl https://sh.rustup.rs -sSf | \
        sh -s -- -y --default-toolchain $TRAVIS_RUST_VERSION

    source ~/.cargo/env

    cargo install --git https://github.com/japaric/cross --branch strace -f
}

main
