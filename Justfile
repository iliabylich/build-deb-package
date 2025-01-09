default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest/*.deb

hypr:
    @just build hypr/hyprutils,\
                hypr/hyprland-qtutils,\
                hypr/hyprwayland-scanner,\
                hypr/hyprgraphics,\
                hypr/aquamarine,\
                hypr/hyprland,\
                hypr/hyprlock,\
                hypr/hyprsunset,\
                hypr/hyprsysteminfo,\
                hypr/hyprpolkitagent

cosmic:
    @just build cosmic/pop-icon-theme
    @just build cosmic/pop-launcher
    @just build cosmic/pop-gtk-theme
    @just build cosmic/pop-fonts
    @just build cosmic/appstream-data-pop

    @just build cosmic/cosmic-applets
    @just build cosmic/cosmic-app-library
    @just build cosmic/cosmic-bg
    @just build cosmic/cosmic-comp
    @just build cosmic/cosmic-edit
    @just build cosmic/cosmic-files
    @just build cosmic/cosmic-greeter
    @just build cosmic/cosmic-icons
    @just build cosmic/cosmic-idle
    @just build cosmic/cosmic-launcher
    @just build cosmic/cosmic-notifications
    @just build cosmic/cosmic-osd
    @just build cosmic/cosmic-panel
    @just build cosmic/cosmic-randr
    @just build cosmic/cosmic-screenshot
    @just build cosmic/cosmic-session
    @just build cosmic/cosmic-settings
    @just build cosmic/cosmic-settings-daemon
    @just build cosmic/cosmic-store
    @just build cosmic/cosmic-term
    @just build cosmic/cosmic-wallpapers
    @just build cosmic/cosmic-workspaces
    @just build cosmic/xdg-desktop-portal-cosmic

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

ghostty:
    @just build ghostty

pwd := `pwd`
base_configs_dir := "/shared"
docker_image := "ghcr.io/iliabylich/debian-unstable-builder:latest"
docker_entrypoint := "/bin/build-deb-package"

run-in-docker command packages:
    sudo docker run --rm \
        -e BASE_CONFIGS_DIR="{{ base_configs_dir }}" \
        -e PACKAGES="{{ packages }}" \
        -it \
        -v "{{pwd}}:/shared" \
        --entrypoint {{docker_entrypoint}} \
        {{docker_image}} \
        {{command}}

build packages:
    @just run-in-docker run {{packages}}

parse packages:
    @just run-in-docker parse {{packages}}

explain packages:
    @just run-in-docker explain {{packages}}

deploy package:
    ./remote/deploy.sh {{package}}

docker-sh:
    sudo docker run --rm -it -v $PWD:/shared --entrypoint bash {{docker_image}}

unpack debfile:
    mkdir -p tmp
    dpkg-deb -R {{debfile}} tmp
