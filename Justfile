default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest

build-all:
    @just build-full-hyprland
    @just build-metapackage
    @just build-blanket

build-full-hyprland:
    @just build hyprutils
    @just build hyprwayland-scanner
    @just build aquamarine
    @just build hyprland
    @just build hyprlock

build-metapackage:
    @just build metapackage

build-blanket:
    @just build blanket

build name:
    ./main.sh {{name}}.yml
