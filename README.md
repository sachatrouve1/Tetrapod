# Tetrapod

Tetrapod is a lightweight utility written in Rust that automatically fetches NASA's Astronomy Picture of the Day (APOD), saves the high-resolution image, and applies it as your desktop wallpaper.

## Features

- **No API Key Required**: Directly queries NASA's public Science REST endpoint.
- **Daily Cache**: Tracks the last fetched date in `~/.cache/tetrapod/last_date.txt` to prevent duplicate downloads on the same day.
- **High Resolution**: Fetches and downloads the uncompressed high-definition image (`hdurl`) when available.
- **Customizable Hook**: Executes `./set_wallpaper.sh` upon each successful download to handle wallpaper setting and desktop theme reloading.
- **Asynchronous**: Built with Rust, Tokio, and Reqwest for fast, non-blocking execution.

## Requirements

- **Rust**: Cargo and rustc supporting the 2024 edition.
- **Hook dependencies** (default script):
  - A Wayland wallpaper utility such as `awww` (or `swww` / `hyprpaper`).
  - Optional theming tools: `pywal`, `waybar`, `swaync`, `cava`.

The hook script can be edited to support any desktop environment or wallpaper manager (such as `feh` on X11 or native GNOME/KDE settings).

## Build

Clone the repository and compile the release binary:

```bash
git clone https://github.com/your-username/Tetrapod.git
cd Tetrapod
chmod +x set_wallpaper.sh
cargo build --release
```

The compiled binary will be placed at `target/release/Tetrapod`.

## Usage

Run the binary directly:

```bash
./target/release/Tetrapod
```

On the first run of the day, Tetrapod queries NASA's API, downloads the image to `./wallpaper.png`, updates the cache timestamp, and invokes `./set_wallpaper.sh`. Subsequent runs on the same day will report `Up to date` and exit immediately.

## Automation with Systemd

Systemd unit files (`tetrapod.service` and `tetrapod.timer`) are included in the repository.

### User Service (Recommended)

To run Tetrapod automatically under your user session:

1. Copy the unit files to your systemd user configuration directory:

```bash
mkdir -p ~/.config/systemd/user
cp tetrapod.service tetrapod.timer ~/.config/systemd/user/
```

2. Remove the `User=<user>` line in `~/.config/systemd/user/tetrapod.service` (user services run as the active user by default).

3. Reload systemd and enable the timer:

```bash
systemctl --user daemon-reload
systemctl --user enable --now tetrapod.timer
```

The timer runs Tetrapod 5 minutes after boot and every 1 hours thereafter, triggering a download only when a new image is available for the day.

## File Locations

| Path                              | Description                                         |
| --------------------------------- | --------------------------------------------------- |
| `~./wallpaper.png`                | Saved high-resolution wallpaper                     |
| `~/.cache/tetrapod/last_date.txt` | Cached timestamp of the last successful fetch       |
| `~/.cache/wallpaper.png`          | Working wallpaper copy referenced by pywal          |
| `./set_wallpaper.sh`              | Shell script executed after downloading a new image |

## Customizing the Wallpaper Script

The binary passes the path to the newly downloaded image as the first argument (`$1`) to `set_wallpaper.sh`. You can adapt this script to any window manager or desktop environment:

```bash
#!/usr/bin/env bash
wallpaper="$1"

# Example: X11 with feh
# feh --bg-fill "$wallpaper"

# Example: Hyprland with hyprpaper
# hyprctl hyprpaper preload "$wallpaper"
# hyprctl hyprpaper wallpaper ",$wallpaper"

# Example: GNOME
# gsettings set org.gnome.desktop.background picture-uri "file://$wallpaper"
```
