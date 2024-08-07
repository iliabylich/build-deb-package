#!/usr/bin/env bash

set -eu

if (( $# != 1 )); then
    echo "Usage: ./main.sh <package-name>"
    exit 1
fi

CONFIG_PATH="$1"

if ! [ -f "$CONFIG_PATH" ]; then
    echo "$CONFIG_PATH does not exist"
    exit 1
fi
echo "Building a package using configuration in $CONFIG_PATH"
mkdir -p deb-latest

sudo docker image rm -f debian-sid-builder

sudo docker build . -t debian-sid-builder
sudo docker run --rm -e CONFIG_PATH="/shared/$CONFIG_PATH" -t -v $PWD:/shared debian-sid-builder
