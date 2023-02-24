#!/usr/bin/env bash

# does not apply until after installing these dependencies
unset SCCACHE_DIR
unset RUSTC_WRAPPER

set -e

apt-get update
DEBIAN_FRONTEND=noninteractive apt-get -y install librust-openssl-dev
rm -rf /var/lib/apt/lists/*
umask 002
sg rustlang -c 'cargo install sccache'
