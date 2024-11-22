#!/usr/bin/env bash

set -eu

color() {
    local color="$1"
    local line="$2"
    echo -e "[$config] $color$line\033[0m"
}

err() {
    color "\033[0;31m" "$1"
}
ok() {
    color "\033[0;32m" "$1"
}
info() {
    color "\033[0;33m" "$1"
}

get_latest_remote_tag() {
    gh release view \
        -R "$github_url" \
        --json "tagName" \
        --jq ".tagName" | sed -e "s/^v//"
}

for config in *.yml; do
    echo
    yaml=$(cat "$config")
    github_url=$(echo "$yaml" | yq -r ".git_clone")
    local_version=$(echo "$yaml" | yq -r ".version")

    if [[ "$github_url" == "dummy" ]]; then
        info "Skipping dummy repo"
    elif [[ "$local_version" == "dummy" ]]; then
        info "Skipping dummy version"
    else
        info "Github: $github_url"
        latest_remote_tag=$(get_latest_remote_tag)
        info "Latest remote tag: $latest_remote_tag"

        if [[ "$latest_remote_tag" == "$local_version" ]]; then
            ok "NO UPDATES"
        else
            err "UPDATE AVAILABLE $local_version -> $latest_remote_tag"
        fi
    fi
done
