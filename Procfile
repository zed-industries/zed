collab: RUST_LOG=${RUST_LOG:-info} cargo run --package=collab serve all
cloud: cd ../cloud; cargo make dev
livekit: livekit-server --dev
blob_store: ./script/run-local-minio
