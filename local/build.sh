#!/usr/bin/env bash

set -eu

if (( $# != 1 )); then
    echo "Usage: $0 <package>"
    exit 1
fi

PACKAGE="$1"

if ! [ -f "$PACKAGE.yml" ]; then
    echo "$PACKAGE does not exist"
    exit 1
fi
echo "Building $PACKAGE"
mkdir -p deb-latest

sudo docker image rm -f debian-sid-builder

sudo docker build ./docker -t debian-sid-builder
sudo docker run --rm -e PACKAGE="$PACKAGE" -t -v $PWD:/shared debian-sid-builder
