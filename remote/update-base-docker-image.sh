#!/usr/bin/env bash

set -eu

source remote/.wait-for-run-to-finish.sh

STAMP=$(date "+%Y-%m-%d %H:%M:%S")
RUN_NAME="Rebuilding docker image at $STAMP"

gh workflow run build-base-docker-image -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/build-deb-package "$RUN_NAME"
notify-send "Finished rebuilding docker image"
