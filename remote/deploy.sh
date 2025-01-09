#!/usr/bin/env bash

set -eu

if (( $# != 1 )); then
    echo "Usage: $0 <just-command>"
    exit 1
fi

JUST_COMMAND="$1"
STAMP=$(date "+%Y-%m-%d %H:%M:%S")

source remote/.wait-for-run-to-finish.sh

RUN_NAME="Building $JUST_COMMAND at $STAMP"
gh workflow run release -f justCommand="$JUST_COMMAND" -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/build-deb-package "$RUN_NAME"
notify-send "Finished building $JUST_COMMAND"

RUN_NAME="Re-index at $STAMP"
gh workflow run -R iliabylich/ppa deploy -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/ppa "$RUN_NAME"
notify-send "Finished re-indexing PPA"

waitForRunToFinish iliabylich/ppa "pages build and deployment"
notify-send "GH pages have been deployed!"
