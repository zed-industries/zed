web: cd ../zed.dev && PORT=3000 npm run dev
collab: cd crates/collab && RUST_LOG=${RUST_LOG:-collab=info} cargo run serve
livekit: livekit-server --dev
postgrest: postgrest crates/collab/admin_api.conf
