#!/bin/sh
set -e 

workdir=$(pwd)



build_local() {
    cargo build --release 
}

push_bin() {
    scp target/release/say root@nas:/volume2/homes/yumo/Drive/pvfile/env/say-$(uname)-$(arch)
}

build_local
push_bin
