#!/bin/sh -ex

cd "$(dirname "$0")"

die() {
    echo "$@" >&2
    exit 1
}

cargo build --manifest-path=../protobuf-codegen/Cargo.toml
cargo build --manifest-path=../protoc-bin/Cargo.toml --bin protoc-bin-print-paths

eval "$(cargo run --manifest-path=../protoc-bin/Cargo.toml --bin protoc-bin-print-paths)"

test -n "$PROTOC"

where_am_i=$(
    cd ..
    pwd
)

rm -rf tmp-generated
mkdir tmp-generated

case $(uname) in
Linux)
    exe_suffix=""
    ;;
MSYS_NT*)
    exe_suffix=".exe"
    ;;
esac

"$PROTOC" \
    --plugin=protoc-gen-rs="$where_am_i/target/debug/protoc-gen-rs$exe_suffix" \
    --rs_out tmp-generated \
    --rs_opt 'inside_protobuf=true gen_mod_rs=false' \
    -I../proto \
    ../proto/google/protobuf/*.proto \
    ../proto/google/protobuf/compiler/*.proto \
    ../proto/rustproto.proto \
    ../proto/doctest_pb.proto

mv \
    tmp-generated/descriptor.rs \
    tmp-generated/plugin.rs \
    tmp-generated/rustproto.rs \
    tmp-generated/doctest_pb.rs \
    src/
mv tmp-generated/well_known_types_mod.rs src/well_known_types/mod.rs
mv tmp-generated/*.rs src/well_known_types/

# vim: set ts=4 sw=4 et:
