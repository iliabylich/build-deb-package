#!/usr/bin/env bash

set -eux

hyprctl plugin load /usr/lib/x86_64-linux-gnu/libhyprspace.so

hyprctl --batch "
keyword bind SUPER,grave,overview:toggle;

keyword plugin:overview:panelHeight 200;
keyword plugin:overview:hideTopLayers true;
keyword plugin:overview:gapsIn 5;
keyword plugin:overview:gapsOut 5;
keyword plugin:overview:exitOnSwitch true;
keyword plugin:overview:workspaceActiveBorder rgb(ffffff)
"
