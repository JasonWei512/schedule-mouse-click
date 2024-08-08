# schedule-mouse-click

[![Crate](https://img.shields.io/crates/v/schedule-mouse-click.svg?color=orange)](https://crates.io/crates/schedule-mouse-click)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/JasonWei512/schedule-mouse-click)](https://github.com/JasonWei512/schedule-mouse-click/releases)

A command line tool to schedule a mouse click at a specific time.


# Install

- Download prebuilt binary from [GitHub release page](https://github.com/JasonWei512/schedule-mouse-click/releases).
  
- 🦀 Install with [Cargo](https://rustup.rs/):
  
  ```
  cargo install schedule-mouse-click
  ```

## ⚠ About Linux 🐧

This program uses [enigo](https://github.com/enigo-rs/enigo) to simulate mouse clicks. Linux users may have to install `libxdo-dev` if they are using `X11`.

For example, on Debian-based distros:

```Bash
apt-get install libxdo-dev
```

On Arch:

```Bash
pacman -S xdotool
```

On Fedora:

```Bash
dnf install libX11-devel libxdo-devel
```


# Usage

## Click at a specific time

Click at 18:00:00 :

```Bash
schedule-mouse-click at 18:00:00
schedule-mouse-click at 18:00
schedule-mouse-click at 6:00pm
```

This program uses [waltzofpearls/dateparser](https://github.com/waltzofpearls/dateparser) to parse time.

## Click after a specific amount of time

Click in 2 minutes and 30 seconds:

```Bash
schedule-mouse-click in 2m30s
schedule-mouse-click in 150s
schedule-mouse-click in 150
```

This program uses [tailhook/humantime](https://github.com/tailhook/humantime) to parse duration.

## Click now

```Bash
schedule-mouse-click now
```

## Double-click

Add `--double` or `-d` flag to double-click:

```Bash
schedule-mouse-click --double at 18:00:00
schedule-mouse-click --double in 2m30s
schedule-mouse-click -d now
```