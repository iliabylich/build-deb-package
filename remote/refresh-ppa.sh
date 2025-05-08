#!/usr/bin/env bash

set -eu

source remote/.wait-for-run-to-finish.sh

STAMP=$(date "+%Y-%m-%d %H:%M:%S")
RUN_NAME="Re-index at $STAMP"
gh workflow run -R iliabylich/ppa deploy -f runName="$RUN_NAME"
waitForRunToFinish iliabylich/ppa "$RUN_NAME"
notify-send "Finished re-indexing PPA"

waitForRunToFinish iliabylich/ppa "pages build and deployment"
notify-send "GH pages have been deployed!"
