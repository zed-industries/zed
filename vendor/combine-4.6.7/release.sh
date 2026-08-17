#!/bin/bash

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Expected patch, minor or major"
    exit 1
fi

clog --$VERSION && \
    git add CHANGELOG.md && \
    git commit -m "Updated changelog" && \
    cargo release --execute $VERSION
