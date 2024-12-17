default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest

full-hyprland:
    @just build hyprutils
    @just build hyprwayland-scanner
    @just build hyprgraphics
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

syshud:
    @just build syshud

build package:
    sudo docker run --rm -e CONFIG_PATH=/shared/{{package}}.toml -t -v $PWD:/shared ghcr.io/iliabylich/debian-unstable-builder:latest

deploy package:
    ./remote/deploy.sh {{package}}
