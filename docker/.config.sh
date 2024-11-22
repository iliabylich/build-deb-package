yaml="$(cat "/shared/$PACKAGE.yml")"

getField() {
    local field="$1"
    echo "$yaml" | yq -r "$field"
}

export GIT_CLONE="$(getField ".git_clone")"
log "[config] GIT_CLONE: $GIT_CLONE"

export VERSION="$(getField ".version")"
log "[config] VERSION: $VERSION"

export INSTALL="$(getField "if has(\"install\") then .install else [] end | join(\" \")")"
log "[config] INSTALL: $INSTALL"

export DEBIAN_DEPENDENCIES="$(getField ".debian | if has(\"dependencies\") then .dependencies else [] end | join(\", \")")"
log "[config] DEBIAN_DEPENDENCIES: $DEBIAN_DEPENDENCIES"

export DESCRIPTION="$(getField ".debian.control.description")"
log "[config] DESCRIPTION=$DESCRIPTION"

export RULES="$(getField ".debian.rules")"
log "[config] RULES=$RULES"

log ""
