#!/usr/bin/env bash

set -eux

hyprctl plugin load /usr/lib/x86_64-linux-gnu/libhyprexpo.so

hyprctl --batch "
keyword bind SUPER,grave,hyprexpo:expo,toggle;

keyword plugin:hyprexpo:columns 3;
keyword plugin:hyprexpo:gap_size 5;
keyword plugin:hyprexpo:bg_col rgb(111111);
keyword plugin:hyprexpo:workspace_method center current;

keyword plugin:hyprexpo:enable_gesture true;
keyword plugin:hyprexpo:gesture_fingers 3;
keyword plugin:hyprexpo:gesture_distance 300;
keyword plugin:hyprexpo:gesture_positive true
"
