# Contributing Guide

## Regenerate Descriptors

`pbjson-types` is generated from a vended set of descriptors, this:

* Avoids requiring downstream crates to have an installed version of `protoc`
* Ensures consistent code generation, regardless of local version of `protoc`

To regenerate the `descriptors.bin` run

```
docker run --rm -v $PWD:/src -w /src archlinux bash -c "pacman -Sy --noconfirm protobuf && protoc --version && protoc -o descriptors.bin --include_imports --include_source_info protos/google/protobuf/types.proto"
```
