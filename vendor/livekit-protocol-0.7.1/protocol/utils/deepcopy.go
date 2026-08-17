// Copyright 2024 LiveKit, Inc.
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
	"reflect"
)

func DeepCopy[T any](v T) T {
	return deepCopy(reflect.ValueOf(v)).Interface().(T)
}

func deepCopy(v reflect.Value) reflect.Value {
	switch v.Type().Kind() {
	case reflect.Array:
		c := reflect.New(v.Type()).Elem()
		for i := range v.Len() {
			c.Index(i).Set(deepCopy(v.Index(i)))
		}
		return c

	case reflect.Map:
		if v.IsNil() {
			return v
		}
		c := reflect.MakeMap(v.Type())
		for mr := v.MapRange(); mr.Next(); {
			c.SetMapIndex(deepCopy(mr.Key()), deepCopy(mr.Value()))
		}
		return c

	case reflect.Pointer:
		if v.IsNil() {
			return v
		}
		c := reflect.New(v.Type().Elem())
		c.Elem().Set(deepCopy(v.Elem()))
		return c

	case reflect.Slice:
		if v.IsNil() {
			return v
		}
		c := reflect.MakeSlice(v.Type(), v.Len(), v.Cap())
		for i := range v.Len() {
			c.Index(i).Set(deepCopy(v.Index(i)))
		}
		return c

	case reflect.Struct:
		c := reflect.New(v.Type()).Elem()
		for i := range v.NumField() {
			if c.Field(i).CanSet() {
				c.Field(i).Set(deepCopy(v.Field(i)))
			}
		}
		return c

	default: // Bool, Chan, Complex128, Complex64, Float32, Float64, Func, Int, Int16, Int32, Int64, Int8, Interface, String, Uint, Uint16, Uint32, Uint64, Uint8, Uintptr, UnsafePointer
		return v
	}
}
