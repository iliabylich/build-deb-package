log() {
    local line="$1"
    echo -e "\033[0;33m$line\033[0m"
}

log "-----------------------------------"
log "|            DOCKER               |"
log "-----------------------------------"

log ""
log "Building $PACKAGE"
log ""
