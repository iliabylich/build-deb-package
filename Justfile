default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest

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

ags:
    @just build ags

mangl:
    @just build mangl

layer-shell:
    @just build layer-shell

build name:
    ./main.sh {{name}}.yml

deploy name:
    ./deploy.sh {{name}}
