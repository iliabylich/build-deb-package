default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest/*.deb

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

validate package:
    sudo docker run --rm -e CONFIG_PATH=/shared/{{package}}.toml -it -v $PWD:/shared --entrypoint /bin/build-deb-package ghcr.io/iliabylich/debian-unstable-builder:latest parse

explain package:
    sudo docker run --rm -e CONFIG_PATH=/shared/{{package}}.toml -it -v $PWD:/shared --entrypoint /bin/build-deb-package ghcr.io/iliabylich/debian-unstable-builder:latest explain

deploy package:
    ./remote/deploy.sh {{package}}

docker-sh:
    sudo docker run --rm -it -v $PWD:/shared --entrypoint bash ghcr.io/iliabylich/debian-unstable-builder:latest
