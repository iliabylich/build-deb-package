default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest/*.deb

docker_image := "ghcr.io/iliabylich/debian-unstable-builder:latest"

run-in-docker command config_path:
    sudo docker run --rm \
        -e BASE_CONFIGS_DIR="/shared" \
        -e CONFIG_PATH="{{ config_path }}" \
        -t \
        -v "$PWD:/shared" \
        --entrypoint "/bin/build-deb-package" \
        {{docker_image}} \
        {{command}}

build config_path:
    @just run-in-docker run {{config_path}}

parse config_path:
    @just run-in-docker parse {{config_path}}

explain config_path:
    @just run-in-docker explain {{config_path}}

deploy config_path:
    ./remote/deploy.sh {{config_path}}

docker-sh:
    sudo docker run --rm -it -v $PWD:/shared --entrypoint bash {{docker_image}}

unpack debfile:
    mkdir -p tmp
    dpkg-deb -R {{debfile}} tmp

shellcheck:
    shellcheck -x **/*.sh

bump config_path:
    BASE_CONFIGS_DIR=$PWD CONFIG_PATH={{config_path}} cargo run -- bump-version-trailer
