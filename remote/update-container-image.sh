#!/usr/bin/env bash

set -eu

source remote/.wait-for-run-to-finish.sh

STAMP=$(date "+%Y-%m-%d %H:%M:%S")
RUN_NAME="Rebuilding image at $STAMP"

gh workflow run rebuild-builder-bin -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/build-deb-package "$RUN_NAME"
notify-send "Finished rebuilding image"

podman image pull ghcr.io/iliabylich/debian-unstable-builder:latest
