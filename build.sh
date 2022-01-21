#!/bin/sh
set -e 

workdir=$(pwd)



build_local() {
    cargo build --release 
}

push_bin() {
    scp target/release/say yumo@pvfile:~/services/pvfile/env/say-$(uname)-$(arch)
}

build_local
push_bin
