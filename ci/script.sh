set -ex

main() {
    sh build-docker-image.sh $TARGET

    if [ $TRAVIS_BRANCH = master ]; then
        return
    fi

    local examples=(
        _llseek
        create
        dup
        format
        hello
        instant
        ls
        open
        preadwrite
        stat
        stderr
        system-time
        vec
        zero
    )

    export QEMU_STRACE=1
    for example in ${examples[@]}; do
        cross run --target $TARGET --no-default-features --features naive_ralloc --example $example
    done

    for example in ${examples[@]}; do
        cross run --target $TARGET --no-default-features --features naive_ralloc --example $example --release
    done

    set +x
    pushd target/$TARGET/release/examples
    size ${examples[@]}
    popd
}

main
