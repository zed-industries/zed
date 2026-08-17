package utils

import (
	"math"
	"regexp"
	"strconv"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestRaw_DefaultFirstGroup(t *testing.T) {
	src := `num-pushed=(guint64)123 num-gap=(uint64)456`
	kv := NewKVRegexScanner(src)

	rePushed := regexp.MustCompile(`\bnum-pushed=\(g?uint64\)(\d+)`)
	reGap := regexp.MustCompile(`\bnum-gap=\(g?uint64\)(\d+)`)

	rawPushed, ok := kv.Raw(rePushed)
	require.True(t, ok, "expected match for num-pushed")
	require.Equal(t, "123", rawPushed)

	rawGap, ok := kv.Raw(reGap)
	require.True(t, ok, "expected match for num-gap")
	require.Equal(t, "456", rawGap)
}

func TestRaw_NamedGroup(t *testing.T) {
	src := `plc-duration=(guint64)20000000`
	kv := NewKVRegexScanner(src)

	reDur := regexp.MustCompile(`\bplc-duration=\(g?uint64\)(?P<ns>\d+)`)

	raw, ok := kv.Raw(reDur, "ns")
	require.True(t, ok, "expected match for plc-duration ns group")
	require.Equal(t, "20000000", raw)
}

func TestUint64_Int64_Uint_Int(t *testing.T) {
	src := `u64=(guint64)184 r64=(int64)-42 u=(uint)48000 i=(int)7`
	kv := NewKVRegexScanner(src)

	reU64 := regexp.MustCompile(`\bu64=\(g?uint64\)(\d+)`)
	reI64 := regexp.MustCompile(`\br64=\(g?int64\)(-?\d+)`)
	reU := regexp.MustCompile(`\bu=\(g?uint\)(\d+)`)
	reI := regexp.MustCompile(`\bi=\(g?int\)(-?\d+)`)

	vU64, ok := kv.Uint64(reU64)
	require.True(t, ok)
	require.Equal(t, uint64(184), vU64)

	vI64, ok := kv.Int64(reI64)
	require.True(t, ok)
	require.Equal(t, int64(-42), vI64)

	vU, ok := kv.Uint(reU)
	require.True(t, ok)
	require.Equal(t, uint(48000), vU)

	vI, ok := kv.Int(reI)
	require.True(t, ok)
	require.Equal(t, 7, vI)
}

func TestFloat64_AndRejectNaNInf(t *testing.T) {
	kv := NewKVRegexScanner(`rms=(double)-12.25 bad1=(double)nan bad2=(double)inf`)
	reOk := regexp.MustCompile(`\brms=\(g?double\)(-?[0-9.]+)`)
	reNaN := regexp.MustCompile(`\bbad1=\(g?double\)([^,}\s]+)`)
	reInf := regexp.MustCompile(`\bbad2=\(g?double\)([^,}\s]+)`)

	v, ok := kv.Float64(reOk)
	require.True(t, ok)
	require.Equal(t, -12.25, v)

	_, ok = kv.Float64(reNaN)
	require.False(t, ok, "Float64 should reject NaN")

	_, ok = kv.Float64(reInf)
	require.False(t, ok, "Float64 should reject Inf")
}

func TestBool_CaseAndAliases(t *testing.T) {
	kv := NewKVRegexScanner(`b1=(gboolean)TRUE b2=(boolean)false b3=(boolean)Yes b4=(boolean)0 other=x`)
	reB1 := regexp.MustCompile(`\bb1=\(g?boolean\)([^,}\s]+)`)
	reB2 := regexp.MustCompile(`\bb2=\(g?boolean\)([^,}\s]+)`)
	reB3 := regexp.MustCompile(`\bb3=\(g?boolean\)([^,}\s]+)`)
	reB4 := regexp.MustCompile(`\bb4=\(g?boolean\)([^,}\s]+)`)

	v1, ok := kv.Bool(reB1)
	require.True(t, ok)
	require.True(t, v1)

	v2, ok := kv.Bool(reB2)
	require.True(t, ok)
	require.False(t, v2)

	v3, ok := kv.Bool(reB3)
	require.True(t, ok)
	require.True(t, v3)

	v4, ok := kv.Bool(reB4)
	require.True(t, ok)
	require.False(t, v4)
}

func TestString_Unquote(t *testing.T) {
	kv := NewKVRegexScanner(`msg=(string)"hello \"world\"" raw=(string)plain`)
	reQ := regexp.MustCompile(`\bmsg=\(string\)("(?:[^"\\]|\\.)*")`)
	reRaw := regexp.MustCompile(`\braw=\(string\)([^,}\s]+)`)

	s, ok := kv.String(reQ)
	require.True(t, ok)
	require.Equal(t, `hello "world"`, s)

	s, ok = kv.String(reRaw)
	require.True(t, ok)
	require.Equal(t, "plain", s)
}

func TestDurationNs(t *testing.T) {
	kv := NewKVRegexScanner(`d=(guint64)20000000`) // 20ms
	re := regexp.MustCompile(`\bd=\(g?uint64\)(\d+)`)

	d, ok := kv.DurationNs(re)
	require.True(t, ok)
	require.Equal(t, 20*time.Millisecond, d)
}

func TestDurationNs_OverflowGuard(t *testing.T) {
	overflow := strconv.FormatUint(math.MaxUint64, 10)
	kv := NewKVRegexScanner(`d=(guint64)` + overflow)
	re := regexp.MustCompile(`\bd=\(g?uint64\)(\d+)`)

	_, ok := kv.DurationNs(re)
	require.False(t, ok, "DurationNs should fail on overflow")
}

func TestNoCapturingGroup_ReturnsFalse(t *testing.T) {
	kv := NewKVRegexScanner(`nogrp=(uint)7`)
	re := regexp.MustCompile(`\bnogrp=\(g?uint\)\d+`) // no capturing group

	_, ok := kv.Uint(re)
	require.False(t, ok, "expected false when regex has no capturing group")
}

func TestMissingNamedGroup_ReturnsFalse(t *testing.T) {
	kv := NewKVRegexScanner(`v=(int)42`)
	re := regexp.MustCompile(`\bv=\(g?int\)(?P<val>\d+)`)

	_, ok := kv.Int(re, "nope") // named group doesn't exist
	require.False(t, ok, "expected false when named group is missing")
}
