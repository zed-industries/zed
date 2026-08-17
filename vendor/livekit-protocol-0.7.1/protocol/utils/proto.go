// Copyright 2023 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package utils

import (
	"github.com/livekit/protocol/livekit/logger"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/reflect/protoreflect"
)

func CloneProto[T proto.Message](m T) T {
	return proto.Clone(m).(T)
}

func CloneProtoSlice[T proto.Message](ms []T) []T {
	cs := make([]T, len(ms))
	for i := range ms {
		cs[i] = CloneProto(ms[i])
	}
	return cs
}

func CloneProtoRedacted[T proto.Message](m T) T {
	clone := proto.Clone(m).(T)

	var redact func(msg proto.Message)
	redact = func(msg proto.Message) {
		if msg == nil {
			return
		}

		reflected := msg.ProtoReflect()
		reflected.Range(func(fd protoreflect.FieldDescriptor, v protoreflect.Value) bool {
			if proto.HasExtension(fd.Options(), logger.E_Redact) {
				reflected.Clear(fd)
			}

			if fd.Kind() == protoreflect.MessageKind {
				switch {
				case fd.IsList():
					src := v.List()
					for i := 0; i < src.Len(); i++ {
						elem := src.Get(i).Message().Interface()
						redact(elem)
					}

				case fd.IsMap():
					src := v.Map()
					src.Range(func(key protoreflect.MapKey, val protoreflect.Value) bool {
						elem := val.Message().Interface()
						redact(elem)
						return true
					})

				default:
					redact(v.Message().Interface())
				}
			}

			return true
		})
	}
	redact(clone)
	return clone
}
