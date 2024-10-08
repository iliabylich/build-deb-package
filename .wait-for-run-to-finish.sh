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
