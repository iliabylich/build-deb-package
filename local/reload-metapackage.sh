#!/usr/bin/env sh

set -eu

PACKAGES=$(cat metapackage.toml | tomlq -r ".debian.control.dependencies | join(\" \")")

sudo apt-mark auto $PACKAGES
