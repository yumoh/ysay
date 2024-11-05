#!/bin/sh
set -e 

workdir=$(pwd)

test_local() {
    RUST_LOG=debug cargo run --features "client" -- "如是我闻:爱本是恨的来处"
}
build_local() {
    cargo build --release 
}

push_bin() {
    yumos upload target/release/ysay tools/yumos/say-$(uname)-$(arch)
}

case $1 in
    "build")
        build_local
        ;;
    "push")
        push_bin
        ;;
    "release")
        build_local
        push_bin
        ;;
    "test")
        test_local
        ;;
    *)
        ;;
esac
