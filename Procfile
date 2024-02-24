collab: RUST_LOG=${RUST_LOG:-warn,tower_http=info,collab=info} cargo run --package=collab serve
livekit: livekit-server --dev
blob_store: MINIO_ROOT_USER=the-blob-store-access-key MINIO_ROOT_PASSWORD=the-blob-store-secret-key minio server .blob_store
