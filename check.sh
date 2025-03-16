#! /bin/bash
set -e

meson setup _build --prefix "$(pwd)/_install"
meson compile tabela-pot -C _build
meson compile tabela-gmo -C _build
meson compile tabela-update-po -C _build
meson compile -C _build
meson install -C _build
meson compile cargo-clippy -C _build
meson compile cargo-fmt -C build
echo ''
echo '-------------------------------------------------------'
echo ''
echo 'If the git repo is clean, everything is ready to merge.'
echo ''
echo 'Run with: _install/bin/tabela'
echo ''
echo '-------------------------------------------------------'
