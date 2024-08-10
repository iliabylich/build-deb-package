#!/usr/bin/env bash

set -eu

if (( $# != 1 )); then
    echo "Usage: ./deploy.sh <package-name>"
    exit 1
fi

PACKAGE="$1"
STAMP=$(date +%s)

# REPO -> RUN_NAME
function waitForRunToFinish() {
    echo "Waiting for run '$2' to appear"
    while : ; do
        echo "[$1] Trying to find '$2'..."
        CREATED_RUN_ID="$(
            gh run list \
                -R "$1" \
                --json "name,databaseId" \
                --limit 1 \
                -q ".[] | select(.name == \"$2\") | .databaseId"
        )"

        if [ ! -z "$CREATED_RUN_ID" ]; then
            break
        fi
    done

    echo "Created run ID: $CREATED_RUN_ID"
    gh run watch -R "$1" "$CREATED_RUN_ID"
}

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
