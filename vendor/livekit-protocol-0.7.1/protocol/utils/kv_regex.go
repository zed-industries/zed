package utils

import (
	"math"
	"regexp"
	"strconv"
	"strings"
	"time"
)

// Scanner holds the source string to be searched.
type Scanner struct {
	src string
}

func NewKVRegexScanner(s string) Scanner { return Scanner{src: s} }

// Raw returns the first capture group by default, or a named group if provided.
// Example:
//
//	re := regexp.MustCompile(`key=(\d+)`)         // group 1
//	kv.Raw(re)  -> "123", true
//	re2 := regexp.MustCompile(`key=(?P<val>\d+)`) // named group "val"
//	kv.Raw(re2, "val") -> "123", true
func (kv Scanner) Raw(re *regexp.Regexp, group ...string) (string, bool) {
	idx, ok := resolveGroupIndex(re, group...)
	if !ok {
		return "", false
	}
	m := re.FindStringSubmatch(kv.src)
	if len(m) == 0 || idx >= len(m) {
		return "", false
	}
	return m[idx], true
}

func (kv Scanner) Uint64(re *regexp.Regexp, group ...string) (uint64, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return 0, false
	}
	u, err := strconv.ParseUint(raw, 10, 64)
	return u, err == nil
}

func (kv Scanner) Int64(re *regexp.Regexp, group ...string) (int64, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return 0, false
	}
	i, err := strconv.ParseInt(raw, 10, 64)
	return i, err == nil
}

func (kv Scanner) Uint(re *regexp.Regexp, group ...string) (uint, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return 0, false
	}
	u, err := strconv.ParseUint(raw, 10, 0)
	return uint(u), err == nil
}

func (kv Scanner) Int(re *regexp.Regexp, group ...string) (int, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return 0, false
	}
	i, err := strconv.ParseInt(raw, 10, 0)
	return int(i), err == nil
}

func (kv Scanner) Float64(re *regexp.Regexp, group ...string) (float64, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return 0, false
	}
	f, err := strconv.ParseFloat(raw, 64)
	if err != nil || math.IsNaN(f) || math.IsInf(f, 0) {
		return 0, false
	}
	return f, true
}

func (kv Scanner) Bool(re *regexp.Regexp, group ...string) (bool, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return false, false
	}
	switch strings.ToLower(raw) {
	case "true", "1", "yes":
		return true, true
	case "false", "0", "no":
		return false, true
	default:
		return false, false
	}
}

// DurationNs treats the captured value as nanoseconds and returns time.Duration.
func (kv Scanner) DurationNs(re *regexp.Regexp, group ...string) (time.Duration, bool) {
	u, ok := kv.Uint64(re, group...)
	if !ok || u > math.MaxInt64 {
		return 0, false
	}
	return time.Duration(u), true
}

// String returns the captured string, unquoting if it looks like "..." (handles \" escapes).
func (kv Scanner) String(re *regexp.Regexp, group ...string) (string, bool) {
	raw, ok := kv.Raw(re, group...)
	if !ok {
		return "", false
	}
	if len(raw) >= 2 && raw[0] == '"' && raw[len(raw)-1] == '"' {
		if unq, err := strconv.Unquote(raw); err == nil {
			return unq, true
		}
	}
	return raw, true
}

func indexOfSubexpName(re *regexp.Regexp, name string) int {
	for i, n := range re.SubexpNames() {
		if n == name {
			return i
		}
	}
	return -1
}

func resolveGroupIndex(re *regexp.Regexp, group ...string) (int, bool) {
	if len(group) == 0 || group[0] == "" {
		if re.NumSubexp() < 1 {
			return -1, false // no capturing groups
		}
		return 1, true // default to first group
	}
	idx := indexOfSubexpName(re, group[0])
	if idx <= 0 {
		return -1, false // named group missing
	}
	return idx, true
}
