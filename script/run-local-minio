#!/bin/bash -e

which minio > /dev/null || (echo "installing minio..."; brew install minio/stable/minio)
mkdir -p .blob_store/the-extensions-bucket
mkdir -p .blob_store/zed-crash-reports

export MINIO_ROOT_USER=the-blob-store-access-key
export MINIO_ROOT_PASSWORD=the-blob-store-secret-key
exec minio server --quiet .blob_store
