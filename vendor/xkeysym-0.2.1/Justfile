# SPDX-License-Identifier: MIT OR Apache-2.0 OR Zlib
# Copyright 2022-2023 John Nunley
#
# Licensed under the Apache License, Version 2.0, the MIT License, and
# the Zlib license ("the Licenses"), you may not use this file except in
# compliance with one of the the Licenses, at your option. You may obtain
#  a copy of the Licenses at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#     http://opensource.org/licenses/MIT
#     http://opensource.org/licenses/Zlib
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the Licenses is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the Licenses for the specific language governing permissions and
# limitations under the Licenses.

keysyms:
    docker container run --rm \
        --name keysym_generator \
        --mount type=bind,source="$(pwd)",target=/xkeysym \
        archlinux:base \
        sh -c "pacman -Syu rust xorgproto --noconfirm && \
        cargo run --manifest-path /xkeysym/keysym-generator/Cargo.toml \
           /xkeysym/src/automatically_generated.rs"

