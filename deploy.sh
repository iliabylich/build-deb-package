#!/usr/bin/env bash

set -eu

if (( $# != 1 )); then
    echo "Usage: ./deploy.sh <package-name>"
    exit 1
fi

PACKAGE="$1"
STAMP=$(date "+%Y-%m-%d %H:%M:%S")

source .wait-for-run-to-finish.sh

RUN_NAME="Building $PACKAGE at $STAMP"
gh workflow run release -f packageName="$PACKAGE" -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/build-deb-package "$RUN_NAME"
notify-send "Finished building $PACKAGE.deb"

RUN_NAME="Re-index at $STAMP"
gh workflow run -R iliabylich/ppa deploy -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/ppa "$RUN_NAME"
notify-send "Finished re-indexing PPA"

waitForRunToFinish iliabylich/ppa "pages build and deployment"
notify-send "GH pages have been deployed!"
