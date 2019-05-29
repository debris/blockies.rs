#!/bin/bash

set -exu

VERSION=$(grep "^version" ./Cargo.toml | sed -e 's/.*"\(.*\)"/\1/')
cargo publish $@

echo "Tagging version $VERSION"
git tag -a v$VERSION -m "Version $VERSION"
git push --tags
