#!/bin/bash
# Copyright 2023 LiveKit, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


if ! command -v protoc &> /dev/null
then
  echo "protoc is required and not found. please install"
  exit 1
fi

if ! command -v mage &> /dev/null
then
  pushd /tmp
  git clone https://github.com/magefile/mage
  cd mage
  go run bootstrap.go
  rm -rf /tmp/mage
  popd
fi

if ! command -v mage &> /dev/null
then
  echo "Ensure `go env GOPATH`/bin is in your \$PATH"
  exit 1
fi

go mod download

GO_VERSION=`go version | { read _ _ v _; echo ${v#go}; }`
GO_TARGET_VERSION=1.17

function version { echo "$@" | awk -F. '{ printf("%d%03d%03d%03d\n", $1,$2,$3,$4); }'; }

if [ $(version $GO_VERSION) -ge $(version $GO_TARGET_VERSION) ];
  then
    go install github.com/twitchtv/twirp/protoc-gen-twirp@v8.1.3
    go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.31.0
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.3
    go install github.com/livekit/psrpc/protoc-gen-psrpc@v0.5.1
  else
    go get -u github.com/twitchtv/twirp/protoc-gen-twirp@v8.1.3
    go get -u google.golang.org/protobuf/cmd/protoc-gen-go@v1.30.0
    go get -u google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.2
    go get -u github.com/livekit/psrpc/protoc-gen-psrpc@v0.3.1
fi
