#!/usr/bin/env bash

set -e

umask 002
sg rustlang -c 'cargo install sqlx-cli --no-default-features --features native-tls,postgres'
