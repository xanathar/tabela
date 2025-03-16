#! /bin/bash
set -e

meson setup _build --prefix "$(pwd)/_install"
meson compile tabela-pot -C _build
meson compile tabela-gmo -C _build
meson compile tabela-update-po -C _build
meson compile -C _build
meson install -C _build
echo ''
echo 'If everything went well, test build is ready.'
echo 'Run with: _install/bin/tabela'
