#! /bin/bash
set -e

fix-translation-of-file () {
    if git diff --ignore-matching-lines='POT-Creation-Date' --exit-code "$1" ; then
        git restore "$1"
    fi
}

fix-translations () {
    for i in "po/"*.po; do
        fix-translation-of-file "$i"
    done
    fix-translation-of-file 'po/tabela.pot'
}

meson setup _build --prefix "$(pwd)/_install"

meson compile tabela-pot -C _build
meson compile tabela-gmo -C _build
meson compile tabela-update-po -C _build

fix-translations

meson compile -C _build
meson install -C _build
meson compile cargo-clippy -C _build
cargo fmt

echo ''
echo '-------------------------------------------------------'
echo ''
echo 'If the git repo is clean, everything is ready to merge.'
echo ''
echo 'Run with: _install/bin/tabela'
echo ''
echo '-------------------------------------------------------'
