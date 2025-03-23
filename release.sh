#! /bin/bash
set -e

die () {
    echo "$1"
    exit 1
}



if [ ! "$(which jq)" ]; then
    die "jq not found in path. Please install jq then try again."
fi

if [ ! "$(which gh)" ]; then
    die "gh not found in path. Please install github cli then try again."
fi

git diff --exit-code >/dev/null 2>&1 || die "Repo is dirty, please commit & push changes."
git merge-base --is-ancestor HEAD '@{u}' 2>&1 || die "Changes have not been pushed."

rm -rf _build

VERSION="$(meson introspect meson.build --projectinfo | jq -r '.version')"
TAG="v$VERSION"

if [ "$(git tag -l "$TAG")" ]; then
    echo "Tag $TAG already exists -- bailing out. Change the version in the meson file and start again."
    exit 1
fi

./check.sh

meson dist -C _build

git tag "$TAG"
git push origin tag "$TAG"

gh create release "$TAG" --generate-notes "_build/meson-dist/tabela-$VERSION.tar.xz"





