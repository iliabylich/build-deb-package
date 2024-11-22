#!/usr/bin/env bash

set -eu

sudo apt autoremove \
    -o Apt::AutoRemove::RecommendsImportant=false \
    -o Apt::AutoRemove::SuggestsImportant=false
