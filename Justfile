default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest/*.deb

image := "ghcr.io/iliabylich/debian-unstable-builder:latest"

run-in-podman command config_path:
    podman run --rm \
        -e BASE_CONFIGS_DIR="/shared" \
        -e CONFIG_PATH="{{ config_path }}" \
        -t \
        -v "$PWD:/shared" \
        --entrypoint "/bin/build-deb-package" \
        {{image}} \
        {{command}}

build config_path:
    @just run-in-podman run {{config_path}}

parse config_path:
    @just run-in-podman parse {{config_path}}

explain config_path:
    @just run-in-podman explain {{config_path}}

deploy config_path:
    ./remote/deploy.sh {{config_path}}

podman-sh:
    podman run --rm -it -v $PWD:/shared --entrypoint bash {{image}}

unpack debfile:
    mkdir -p tmp
    dpkg-deb -R {{debfile}} tmp

shellcheck:
    shellcheck -x **/*.sh

bump config_path:
    BASE_CONFIGS_DIR=$PWD CONFIG_PATH={{config_path}} cargo run -- bump-version-trailer
