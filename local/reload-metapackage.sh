#!/usr/bin/env sh

set -eu

PACKAGES=$(cat metapackage.yml | yq -r ".debian.dependencies | join(\" \")")

sudo apt-mark auto $PACKAGES
