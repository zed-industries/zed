#!/usr/bin/env bash

set -e

echo "Generating SchemeMut from Scheme"
sed 's/trait Scheme/trait SchemeMut/' scheme.rs \
| sed 's/\&self/\&mut self/g' \
> scheme_mut.rs

echo "Generating SchemeBlock from Scheme"
sed 's/trait Scheme/trait SchemeBlock/' scheme.rs \
| sed 's/fn handle(\&self, packet: \&mut Packet)/fn handle(\&self, packet: \&Packet) -> Option<usize>/' \
| sed 's/packet.a = Error::mux(res);/res.transpose().map(Error::mux)/' \
| sed 's/\.map(|f| f\.bits())/\.map(|f| f.map(|f| f.bits()))/' \
| sed 's/\.map(|o| o as usize)/.map(|o| o.map(|o| o as usize))/' \
| sed 's/Ok(0)/Ok(Some(0))/g' \
| sed 's/Result<\([^>]\+\)>/Result<Option<\1>>/g' \
| sed 's/convert_to_this_scheme/convert_to_this_scheme_block/g' \
| sed 's/convert_in_scheme_handle/convert_in_scheme_handle_block/g' \
> scheme_block.rs

echo "Generating SchemeBlockMut from SchemeBlock"
sed 's/trait SchemeBlock/trait SchemeBlockMut/' scheme_block.rs \
| sed 's/\&self/\&mut self/g' \
> scheme_block_mut.rs
