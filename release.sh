#! /bin/bash
set -e

if [ ! "$(which jq)" ]; then
    echo "jq not found in path. Please install jq then try again."
    exit 1
fi

if [ ! "$(which gh)" ]; then
    echo "gh not found in path. Please install github cli then try again."
    exit 1
fi

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





