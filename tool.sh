#! /bin/bash
set -e

APP_ID="com.mastropaolo.www.tabela"

ERROR='\033[1;31m'
WARNING='\033[1;33m'
INFO='\033[0;36m'
CLEAR='\033[0m' # No Color

die () {
    error "$1"
    exit 1
}

error () {
    echo -e "${ERROR}$1${CLEAR}"
}


info () {
    echo -e "${INFO}$1${CLEAR}"
}

warning () {
    echo -e "${WARNING}$1${CLEAR}"
}

require () {
    if [ ! "$(which "$1")" ]; then
        die "'$1' not found in path. Please install '$1' then try again."
    fi
}

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

do_clean () {
    rm -rf _build _install .flatpak-builder builddir repo target
}

do_build () {
    if [ ! -d "$DIR" ]; then
        meson setup _build --prefix "$(pwd)/_install"
    fi

    meson compile -C _build
    meson install -C _build
}

do_qcheck () {
    if [ ! -d "$DIR" ]; then
        meson setup _build --prefix "$(pwd)/_install"
    fi

    meson compile tabela-pot -C _build
    meson compile tabela-gmo -C _build
    meson compile tabela-update-po -C _build
    meson test -C _build

    fix-translations

    meson compile cargo-clippy -C _build
    cargo fmt
}

do_publish () {
    info "Checking prerequisite programs are installed"

    require flatpak
    require jq
    require gh

    info "Checking repo is clean and pushed upstream"

    git diff --exit-code >/dev/null 2>&1 || die "Repo is dirty, please commit & push changes."
    git merge-base --is-ancestor HEAD '@{u}' 2>&1 || die "Changes have not been pushed."

    info "Checking version is valid"

    VERSION="$(meson introspect meson.build --projectinfo | jq -r '.version')"
    TAG="v$VERSION"

    if [ "$(git tag -l "$TAG")" ]; then
        echo "Tag $TAG already exists -- bailing out. Change the version in the meson file and start again."
        exit 1
    fi

    echo ""
    echo ""

    warning "Releasing version $VERSION"
    warning "Press Ctrl+C in 5 seconds to abort"
    echo "5..."
    sleep 1
    echo "4..."
    sleep 1
    echo "3..."
    sleep 1
    echo "2..."
    sleep 1
    echo "1..."
    sleep 1
    echo "GO!"

    info "Cleaning working directories..."

    do_clean

    info "Running check script..."

    do_qcheck

    info "Creating tarball..."

    meson dist -C _build

    info "Creating tag..."

    git tag "$TAG"
    git push origin tag "$TAG"

    info "Creating release..."

    gh release create "$TAG" --generate-notes "_build/meson-dist/tabela-$VERSION.tar.xz"

    info "Creating manifest..."

    URL=$(gh release view v0.1.2 --json assets | jq ".assets[0].url" -r)
    HASH=$(cut "_build/meson-dist/tabela-$VERSION.tar.xz.sha256sum" -b 1-64)

    m4 -DTARBALL_URL="$URL" -DTARBALL_HASH="$HASH" "build-aux/$APP_ID.json.in" > "_build/$APP_ID.json"

    info "Building flatpak..."

    flatpak run org.flatpak.Builder --force-clean --sandbox --user --install --install-deps-from=flathub --ccache --mirror-screenshots-url=https://dl.flathub.org/media/ --repo=repo builddir "_build/$APP_ID.json"

    info "Linting flatpak..."

    flatpak run --command=flatpak-builder-lint org.flatpak.Builder manifest "_build/$APP_ID.json"

    info "Linting flatpak repo..."

    if flatpak run --command=flatpak-builder-lint org.flatpak.Builder repo repo | jq ".errors" | grep -Ev '(\[|\]|appstream-screenshots-not-mirrored-in-ostree)'; then
        echo -e "${ERROR}Validation failed:${CLEAR}"
        flatpak run --command=flatpak-builder-lint org.flatpak.Builder repo repo
        exit 1
    fi

    info "Done."
}

do_check () {
    do_clean
    do_build
    do_qcheck

    echo ''
    echo '-------------------------------------------------------'
    echo ''
    echo 'If the git repo is clean, everything is ready to merge.'
    echo ''
    echo '-------------------------------------------------------'
}

usage () {
    echo "tool.sh <COMMAND> "
    echo ""
    echo "COMMAND:"
    echo "   clean - Cleans all build directories"
    echo "   build - Perform a quick build using meson"
    echo "   check - Performs a check that everything is ok before commits"
    echo "  qcheck - Performs a quicker check with no build"
    echo " publish - Publishes a new release of tarball for a future flatpak release"
}

main () {
    if [ -z "$1" ]; then
        usage
        exit 0
    fi

    case "$1" in
        "clean")
            do_clean
            ;;
        "build")
            do_build
            ;;
        "check")
            do_check
            ;;
        "qcheck")
            do_qcheck
            ;;
        "publish")
            do_publish
            ;;
        *)
            error "Invalid command"
            usage
            ;;
    esac
}

main "$1"

