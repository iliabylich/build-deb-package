#!/usr/bin/env bash

set -eu

mkdir -p /build
cd /build

log() {
    echo -e "\033[0;33m$1\033[0m"
}

log "-----------------------------------"
log "|            DOCKER               |"
log "-----------------------------------"

log "Building using $CONFIG_PATH"

YAML="$(cat "$CONFIG_PATH")"

getField() {
    echo "$YAML" | yq -r "$1"
}

getLatestVersion() {
    GITHUB=$(getField ".code.github")
    log "Searching for the latest release"
    RELEASE_JSON=$(wget -qO- "https://api.github.com/repos/$GITHUB/releases/latest")
    TAG_NAME="$(echo $RELEASE_JSON | jq -r ".tag_name")"
    VERSION="${TAG_NAME#v}"
}

wgetLatestRelease() {
    log "Downloading .tar.gz release $VERSION"
    wget -q "https://github.com/$GITHUB/archive/refs/tags/$TAG_NAME.tar.gz" -O "v$VERSION.tar.gz"
    tar -xf "v$VERSION.tar.gz"
    rm -f "v$VERSION.tar.gz"
}

gitCloneLatestRelease() {
    log "Running git clone for $VERSION"
    git clone "https://github.com/$GITHUB.git" --filter=blob:none --branch "v$VERSION" --recursive --shallow-submodules --depth=1 "$PACKAGE_NAME-$VERSION"
}

buildPackageYourself() {
    VERSION="0.0.$(date +%s)"
    mkdir "$PACKAGE_NAME-$VERSION"
}

PACKAGE_NAME="$(getField ".name")"

STRATEGY="$(getField ".code.strategy")"

case "$STRATEGY" in
    "wget-latest-release")
        getLatestVersion
        wgetLatestRelease
        ;;

    "git-clone-latest-release")
        getLatestVersion
        gitCloneLatestRelease
        ;;

    "make-package-yourself")
        buildPackageYourself
        ;;

    *)
        echo "Unknown .code.strategy $STRATEGY, aborting"
        exit 1
        ;;
esac

cd "$PACKAGE_NAME-$VERSION"
ls -l

APT_INSTALL="$(getField "if has(\"install\") then .install else [] end | join(\" \")")"
log "Installing dependencies $APT_INSTALL"
apt-get -qq install -y $APT_INSTALL > /dev/null

mkdir -p debian

echo
log "Writing debian/changelog:"
cat <<EOF > debian/changelog
$PACKAGE_NAME ($VERSION) unstable; urgency=low

  * Release

 -- John Doe <john@doe.org>  Wed, 22 May 2024 17:54:24 +0000
EOF
cat debian/changelog

echo
log "Writing debian/compat:"
echo "10" > debian/compat
cat debian/compat

echo
log "Writing debian/control:"
DEPS="$(getField ".debian | if has(\"dependencies\") then .dependencies else [] end | join(\", \")")"
cat <<EOF > debian/control
Source: $PACKAGE_NAME
Section: utils
Priority: extra
Maintainer: John Doe <john@doe.org>
Standards-Version: 4.6.2

Package: $PACKAGE_NAME
Section: utils
Priority: extra
Architecture: amd64
Depends: $DEPS
Description: $(getField ".debian.control.description")
EOF
cat debian/control

echo
log "Writing debian/rules:"
cat <<EOF > debian/rules
#!/usr/bin/make -f
export DH_VERBOSE = 1

$(getField ".debian.rules")
EOF
cat debian/rules
chmod +x debian/rules

dh binary
cd ..
ls -l

DEB="${PACKAGE_NAME}_${VERSION}_amd64.deb"

log "Checking if compiled deb can be installed"
apt-get install -qq --dry-run "./$DEB" > /dev/null || true

log "Copying $DEB to host machine"
cp "$DEB" /shared
cp "$DEB" "/shared/deb-latest/${PACKAGE_NAME}.deb"
