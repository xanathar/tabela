# Contributing to Tabëla

## Prerequisites
If you want to contribute to the Tabëla project, you will need to have the
development tools appropriate for your operating system, including:

- Rust 1.85 or later
- Meson
- Ninja
- Gettext (19.7 or newer)

## How to build

Easiest way: using [GNOME Builder](https://flathub.org/apps/org.gnome.Builder).

Harder way: just launch the `check.sh` script.

Hardest way:
```bash
meson setup _build --prefix "$(pwd)/_install"
meson compile -C _build
meson install -C _build
_install/bin/tabela
```

## Before submitting a PR

Run the `check.sh` script. If the repository has changed files, please
commit them (after checking the changes).

