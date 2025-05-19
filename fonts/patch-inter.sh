#!/usr/bin/env bash

set eux

parallel -j $(nproc) fontforge -script font-patcher --variable-width-glyphs -q -c {} -out ./patched &> /dev/null ::: extras/otf/*.otf
