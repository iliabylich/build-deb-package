default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest

all:
    @just full-hyprland
    @just metapackage
    @just blanket

full-hyprland:
    @just build hyprutils
    @just build hyprwayland-scanner
    @just build aquamarine
    @just build hyprland
    @just build hyprlock

metapackage:
    @just build metapackage

blanket:
    @just build blanket

waybar-network-applet:
    @just build waybar-network-applet

build name:
    ./main.sh {{name}}.yml
