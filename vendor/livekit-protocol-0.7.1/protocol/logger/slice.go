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

package logger

import (
	"time"

	"go.uber.org/multierr"
	"go.uber.org/zap/zapcore"
	"google.golang.org/protobuf/proto"
)

type protoSlice[T proto.Message] []T

func ProtoSlice[T proto.Message](s []T) zapcore.ArrayMarshaler {
	return protoSlice[T](s)
}

func (s protoSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	var err error
	for _, v := range s {
		err = multierr.Append(err, e.AppendObject(Proto(v)))
	}
	return err
}

type objectSlice[T zapcore.ObjectMarshaler] []T

func ObjectSlice[T zapcore.ObjectMarshaler](s []T) zapcore.ArrayMarshaler {
	return objectSlice[T](s)
}

func (s objectSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	var err error
	for _, v := range s {
		err = multierr.Append(err, e.AppendObject(v))
	}
	return err
}

type timeSlice []time.Time

func TimeSlice(s []time.Time) zapcore.ArrayMarshaler {
	return timeSlice(s)
}

func (s timeSlice) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendTime(v)
	}
	return nil
}

type durationSlice []time.Duration

func DurationSlice(s []time.Duration) zapcore.ArrayMarshaler {
	return durationSlice(s)
}

func (s durationSlice) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendDuration(v)
	}
	return nil
}

type boolSlice[T ~bool] []T

func BoolSlice[T ~bool](s []T) zapcore.ArrayMarshaler {
	return boolSlice[T](s)
}

func (s boolSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendBool(bool(v))
	}
	return nil
}

type byteStringSlice[T ~[]byte] []T

func ByteStringSlice[T ~[]byte](s []T) zapcore.ArrayMarshaler {
	return byteStringSlice[T](s)
}

func (s byteStringSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendByteString([]byte(v))
	}
	return nil
}

type complex128Slice[T ~complex128] []T

func Complex128Slice[T ~complex128](s []T) zapcore.ArrayMarshaler {
	return complex128Slice[T](s)
}

func (s complex128Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendComplex128(complex128(v))
	}
	return nil
}

type complex64Slice[T ~complex64] []T

func Complex64Slice[T ~complex64](s []T) zapcore.ArrayMarshaler {
	return complex64Slice[T](s)
}

func (s complex64Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendComplex64(complex64(v))
	}
	return nil
}

type float64Slice[T ~float64] []T

func Float64Slice[T ~float64](s []T) zapcore.ArrayMarshaler {
	return float64Slice[T](s)
}

func (s float64Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendFloat64(float64(v))
	}
	return nil
}

type float32Slice[T ~float32] []T

func Float32Slice[T ~float32](s []T) zapcore.ArrayMarshaler {
	return float32Slice[T](s)
}

func (s float32Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendFloat32(float32(v))
	}
	return nil
}

type intSlice[T ~int] []T

func IntSlice[T ~int](s []T) zapcore.ArrayMarshaler {
	return intSlice[T](s)
}

func (s intSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendInt(int(v))
	}
	return nil
}

type int64Slice[T ~int64] []T

func Int64Slice[T ~int64](s []T) zapcore.ArrayMarshaler {
	return int64Slice[T](s)
}

func (s int64Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendInt64(int64(v))
	}
	return nil
}

type int32Slice[T ~int32] []T

func Int32Slice[T ~int32](s []T) zapcore.ArrayMarshaler {
	return int32Slice[T](s)
}

func (s int32Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendInt32(int32(v))
	}
	return nil
}

type int16Slice[T ~int16] []T

func Int16Slice[T ~int16](s []T) zapcore.ArrayMarshaler {
	return int16Slice[T](s)
}

func (s int16Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendInt16(int16(v))
	}
	return nil
}

type int8Slice[T ~int8] []T

func Int8Slice[T ~int8](s []T) zapcore.ArrayMarshaler {
	return int8Slice[T](s)
}

func (s int8Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendInt8(int8(v))
	}
	return nil
}

type stringSlice[T ~string] []T

func StringSlice[T ~string](s []T) zapcore.ArrayMarshaler {
	return stringSlice[T](s)
}

func (s stringSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendString(string(v))
	}
	return nil
}

type uintSlice[T ~uint] []T

func UintSlice[T ~uint](s []T) zapcore.ArrayMarshaler {
	return uintSlice[T](s)
}

func (s uintSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendUint(uint(v))
	}
	return nil
}

type uint64Slice[T ~uint64] []T

func Uint64Slice[T ~uint64](s []T) zapcore.ArrayMarshaler {
	return uint64Slice[T](s)
}

func (s uint64Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendUint64(uint64(v))
	}
	return nil
}

type uint32Slice[T ~uint32] []T

func Uint32Slice[T ~uint32](s []T) zapcore.ArrayMarshaler {
	return uint32Slice[T](s)
}

func (s uint32Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendUint32(uint32(v))
	}
	return nil
}

type uint16Slice[T ~uint16] []T

func Uint16Slice[T ~uint16](s []T) zapcore.ArrayMarshaler {
	return uint16Slice[T](s)
}

func (s uint16Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendUint16(uint16(v))
	}
	return nil
}

type uint8Slice[T ~uint8] []T

func Uint8Slice[T ~uint8](s []T) zapcore.ArrayMarshaler {
	return uint8Slice[T](s)
}

func (s uint8Slice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendUint8(uint8(v))
	}
	return nil
}

type uintptrSlice[T ~uintptr] []T

func UintptrSlice[T ~uintptr](s []T) zapcore.ArrayMarshaler {
	return uintptrSlice[T](s)
}

func (s uintptrSlice[T]) MarshalLogArray(e zapcore.ArrayEncoder) error {
	for _, v := range s {
		e.AppendUintptr(uintptr(v))
	}
	return nil
}
