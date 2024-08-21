#!/usr/bin/env sh

set -eu

PACKAGES=$(cat metapackage.yml | yq -r ".debian.dependencies | join(\" \")")

for PACKAGE in $PACKAGES; do
    if [ "$PACKAGE" != '${shlibs:Depends}' ]
    then
        sudo apt-mark auto $PACKAGE
    fi
done
