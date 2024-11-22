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

mangl:
    @just build mangl

layer-shell:
    @just build layer-shell

fx:
    @just build fx

build package:
    ./local/build.sh {{package}}

deploy package:
    ./remote/deploy.sh {{package}}
