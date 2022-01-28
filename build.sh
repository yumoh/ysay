#!/bin/sh
set -e 

workdir=$(pwd)



build_local() {
    cargo build --release 
}

push_bin() {
    scp target/release/say yumo@nas:~/services/pvfile/env/say-$(uname)-$(arch)
}

build_local
push_bin
