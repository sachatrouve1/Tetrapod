#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Error, choose an image file to set as a wallpaper."
    exit 1
fi

# Ensure full PATH is available even in restricted systemd environments
export PATH="/run/wrappers/bin:/etc/profiles/per-user/$(whoami)/bin:/run/current-system/sw/bin:$HOME/.nix-profile/bin:$PATH"

# Auto-detect WAYLAND_DISPLAY if not set in the service environment
if [ -z "$WAYLAND_DISPLAY" ]; then
    for sock in /run/user/$(id -u)/wayland-*; do
        if [ -S "$sock" ] && [[ ! "$sock" =~ -daemon\.sock$ ]]; then
            export WAYLAND_DISPLAY=$(basename "$sock")
            break
        fi
    done
fi

wallpaper=$1

cp $wallpaper ~/.cache/wallpaper.png
awww img -t 'grow' $wallpaper
echo "Wallpaper $wallpaper applied."
wal -n --contrast 4.0 -i $wallpaper
echo "Theme applied."
~/.config/wal/hooks/bordercolor.sh
echo "Windows border color added."
~/.config/wal/hooks/cava.sh
echo "Cava theme applied."
# pywalfox update
# echo "Firefox theme updated."
# pywal-spicetify Default
# echo "Sptotify theme updated."
pkill -9 waybar
sleep 0.2
hyprctl dispatch 'hl.dsp.exec_cmd("waybar")' >/dev/null 2>&1 || hyprctl dispatch exec waybar >/dev/null 2>&1 || systemd-run --user waybar >/dev/null 2>&1
echo "Waybar reloaded."

if pidof swaync >/dev/null 2>&1; then
    swaync-client -R -rs
    echo "swaync notification reloaded."
else
    hyprctl dispatch 'hl.dsp.exec_cmd("swaync")' >/dev/null 2>&1 || swaync &
    echo "swaync notification started."
fi
