#!/usr/bin/env bash

set -eu

if (( $# != 1 )); then
    echo "Usage: $0 <just-command>"
    exit 1
fi

CONFIG_PATH="$1"
STAMP=$(date "+%Y-%m-%d %H:%M:%S")
RELEASE_NAME=$(echo "${CONFIG_PATH%%.*}" | tr "/" "-")

source remote/.wait-for-run-to-finish.sh

RUN_NAME="Building $CONFIG_PATH at $STAMP"
gh workflow run release -f configPath="$CONFIG_PATH" -f runName="$RUN_NAME" -f releaseName="$RELEASE_NAME"
waitForRunToFinish iliabylich/build-deb-package "$RUN_NAME"
notify-send "Finished building $CONFIG_PATH"

source remote/refresh-ppa.sh
