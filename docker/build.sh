#!/usr/bin/env bash

# This script must run IN DOCKER
# It build package defined in <package>.yml by calling it with
#
# /shared/docker/build.sh <package>

set -eu

source /shared/docker/.log.sh
source /shared/docker/.setup-rust.sh
source /shared/docker/.config.sh

mkdir -p /build

if [[ "$GIT_CLONE" == "dummy" ]]; then
    # dummy package
    export VERSION="0.0.$(date +%s)"
    BUILD_DIR="/build/$PACKAGE-$VERSION"
    mkdir "$BUILD_DIR"
else
    log "Cloning git repo"
    CLONE_OPTIONS="--filter=blob:none --recursive --shallow-submodules --depth=1 -q"
    if [[ "$VERSION" == "dummy" ]]; then
        export VERSION="0.0.$(date +%s)"
    else
        CLONE_OPTIONS="$CLONE_OPTIONS --branch v$VERSION"
    fi
    BUILD_DIR="/build/$PACKAGE-$VERSION"

    log "git clone $GIT_CLONE $CLONE_OPTIONS $BUILD_DIR"
    git clone "$GIT_CLONE" $CLONE_OPTIONS "$BUILD_DIR"
fi

cd "$BUILD_DIR"
ls -l

log "Installing dependencies $INSTALL"
apt update > /dev/null
apt-get -qq install -y $INSTALL > /dev/null

mkdir -p debian

log ""
log "Writing debian/changelog:"
envsubst '$PACKAGE $VERSION' < /shared/docker/changelog > debian/changelog
cat debian/changelog

log ""
log "Writing debian/compat:"
echo "10" > debian/compat
cat debian/compat

echo
log "Writing debian/control:"
envsubst '$PACKAGE $DEBIAN_DEPENDENCIES $DESCRIPTION' < /shared/docker/control > debian/control
cat debian/control

echo
log "Writing debian/rules:"
envsubst '$RULES' < /shared/docker/rules > debian/rules
cat debian/rules
chmod +x debian/rules

dh binary
cd ..
ls -l

DEB="${PACKAGE}_${VERSION}_amd64.deb"

log "Checking if compiled deb can be installed"
apt-get install -qq --dry-run "./$DEB" > /dev/null || true

log "Copying $DEB to host machine"
cp "$DEB" /shared
cp "$DEB" "/shared/deb-latest/${PACKAGE}.deb"
