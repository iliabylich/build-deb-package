#!/usr/bin/env bash

set -eu

cargo build
EXE=target/debug/build-deb-package

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
        -R "$git_url" \
        --json "tagName" \
        --jq ".tagName"
}

for config in *.toml; do
    if [[ "$config" == "Cargo.toml" ]]; then
        continue
    fi

    echo

    git_url="$(CONFIG_PATH=$config $EXE print-git-url)"
    git_tag_or_branch="$(CONFIG_PATH=$config $EXE print-git-tag-or-branch)"

    if [[ "$git_url" == "none" ]]; then
        info "skipping, no git url"
        continue
    fi

    if [[ "$git_tag_or_branch" == "master" || "$git_tag_or_branch" == "main" ]]; then
        info "skipping, not a tag ($git_tag_or_branch branch)"
        continue
    fi

    local_tag="$git_tag_or_branch"

    info "Github: $git_url"
    latest_remote_tag=$(get_latest_remote_tag)
    info "Latest remote tag: $latest_remote_tag"

    if [[ "$latest_remote_tag" == "$local_tag" ]]; then
        ok "NO UPDATES"
    else
        err "UPDATE AVAILABLE $local_tag -> $latest_remote_tag"
    fi
done
