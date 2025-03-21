#!/usr/bin/env sh

set -eu

PACKAGES=$(tomlq -r ".debian.control.dependencies | join(\" \")" metapackage.toml)

eval "sudo apt-mark auto $PACKAGES"
