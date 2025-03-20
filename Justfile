default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest/*.deb

hypr:
    @just build hypr/hyprutils,\
                hypr/hyprlang,\
                hypr/hyprcursor,\
                hypr/hyprland-qtutils,\
                hypr/hyprwayland-scanner,\
                hypr/hyprgraphics,\
                hypr/aquamarine,\
                hypr/hyprland,\
                hypr/hyprlock,\
                hypr/hyprsunset,\
                hypr/hyprsysteminfo,\
                hypr/hyprpolkitagent,\
                hypr/hyprland-qt-support,\
                hypr/xdg-desktop-portal-hyprland

cosmic:
    @just build cosmic/pop-icon-theme,\
                cosmic/pop-launcher,\
                cosmic/pop-gtk-theme,\
                cosmic/pop-fonts,\
                cosmic/appstream-data-pop,\
                cosmic/cosmic-applets,\
                cosmic/cosmic-app-library,\
                cosmic/cosmic-bg,\
                cosmic/cosmic-comp,\
                cosmic/cosmic-edit,\
                cosmic/cosmic-files,\
                cosmic/cosmic-greeter,\
                cosmic/cosmic-icons,\
                cosmic/cosmic-idle,\
                cosmic/cosmic-launcher,\
                cosmic/cosmic-notifications,\
                cosmic/cosmic-osd,\
                cosmic/cosmic-panel,\
                cosmic/cosmic-randr,\
                cosmic/cosmic-screenshot,\
                cosmic/cosmic-session,\
                cosmic/cosmic-settings,\
                cosmic/cosmic-settings-daemon,\
                cosmic/cosmic-store,\
                cosmic/cosmic-term,\
                cosmic/cosmic-wallpapers,\
                cosmic/cosmic-workspaces,\
                cosmic/xdg-desktop-portal-cosmic

metapackage:
    @just build metapackage

mangl:
    @just build mangl

layer-shell:
    @just build layer-shell

fx:
    @just build fx

ghostty:
    @just build ghostty

spot:
    @just build spot

make-btrfs-snapshot:
    @just build make-btrfs-snapshot

gaskpass:
    @just build gaskpass

xremap:
    @just build xremap

pipewire-dbus:
    @just build pipewire-dbus

wezterm:
    @just build wezterm

satty:
    @just build satty

unsplash-wallpaper:
    @just build unsplash-wallpaper

matugen:
    @just build matugen

libinput-gestures:
    @just build libinput-gestures

pwd := `pwd`
base_configs_dir := "/shared"
docker_image := "ghcr.io/iliabylich/debian-unstable-builder:latest"
docker_entrypoint := "/bin/build-deb-package"

run-in-docker command packages:
    sudo docker run --rm \
        -e BASE_CONFIGS_DIR="{{ base_configs_dir }}" \
        -e PACKAGES="{{ packages }}" \
        -t \
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
