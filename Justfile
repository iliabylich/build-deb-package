default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest/*.deb

hypr:
    @just build hypr/hyprutils
    @just build hypr/hyprwayland-scanner
    @just build hypr/hyprgraphics
    @just build hypr/aquamarine
    @just build hypr/hyprland
    @just build hypr/hyprlock

cosmic:
    @just build cosmic/pop-icon-theme
    @just build cosmic/cosmic-applets
    @just build cosmic/cosmic-app-library
    @just build cosmic/cosmic-bg
    @just build cosmic/cosmic-comp
    @just build cosmic/cosmic-edit
    @just build cosmic/cosmic-files
    @just build cosmic/cosmic-greeter
    @just build cosmic/cosmic-icons

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

parse package:
    sudo docker run --rm -e CONFIG_PATH=/shared/{{package}}.toml -it -v $PWD:/shared --entrypoint /bin/build-deb-package ghcr.io/iliabylich/debian-unstable-builder:latest parse

explain package:
    sudo docker run --rm -e CONFIG_PATH=/shared/{{package}}.toml -it -v $PWD:/shared --entrypoint /bin/build-deb-package ghcr.io/iliabylich/debian-unstable-builder:latest explain

deploy package:
    ./remote/deploy.sh {{package}}

docker-sh:
    sudo docker run --rm -it -v $PWD:/shared --entrypoint bash ghcr.io/iliabylich/debian-unstable-builder:latest
