#!/bin/bash
set -e

# publish `sea-query-attr`
cd sea-query-attr
cargo publish
cd ..

# publish `sea-query-derive`
cd sea-query-derive
cargo publish
cd ..

# publish `sea-query`
cargo publish

# publish `sea-query-binder`
cd sea-query-binder
cargo publish
cd ..

# publish `sea-query-rusqlite`
cd sea-query-rusqlite
cargo publish
cd ..

# publish `sea-query-postgres`
cd sea-query-postgres
cargo publish
cd ..
