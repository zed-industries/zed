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

package sip

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io"
	"maps"
	"math"
	"net/netip"
	"sort"
	"strings"

	"github.com/dennwc/iters"
	"github.com/twitchtv/twirp"
	"golang.org/x/exp/slices"

	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/logger"
	"github.com/livekit/protocol/rpc"
	"github.com/livekit/protocol/utils"
	"github.com/livekit/protocol/utils/guid"
)

//go:generate stringer -type TrunkFilteredReason -trimprefix TrunkFiltered
//go:generate stringer -type TrunkConflictReason -trimprefix TrunkConflict
//go:generate stringer -type DispatchRuleConflictReason -trimprefix DispatchRuleConflict

func NewCallID() string {
	return guid.New(utils.SIPCallPrefix)
}

type ErrNoDispatchMatched struct {
	NoRules      bool
	NoTrunks     bool
	CalledNumber string
}

func (e *ErrNoDispatchMatched) Error() string {
	if e.NoRules {
		return "No SIP Dispatch Rules defined"
	}
	if e.NoTrunks {
		return fmt.Sprintf("No SIP Trunk or Dispatch Rules matched for %q", e.CalledNumber)
	}
	return fmt.Sprintf("No SIP Dispatch Rules matched for %q", e.CalledNumber)
}

// DispatchRulePriority returns sorting priority for dispatch rules. Lower value means higher priority.
func DispatchRulePriority(info *livekit.SIPDispatchRuleInfo) int32 {
	// In all these cases, prefer pin-protected rules and rules for specific calling number.
	// Thus, the order will be the following:
	// - 0: Direct or Pin (both pin-protected)
	// - 1: Caller, aka Individual (pin-protected)
	// - 2: Callee (pin-protected)
	// - 100: Direct (open)
	// - 101: Caller, aka Individual (open)
	// - 102: Callee (open)
	// Also, add 1K penalty for not specifying the calling number.
	const (
		last = math.MaxInt32
	)
	// TODO: Maybe allow setting specific priorities for dispatch rules?
	priority := int32(0)
	switch rule := info.GetRule().GetRule().(type) {
	default:
		return last
	case *livekit.SIPDispatchRule_DispatchRuleDirect:
		if rule.DispatchRuleDirect.GetPin() != "" {
			priority = 0
		} else {
			priority = 100
		}
	case *livekit.SIPDispatchRule_DispatchRuleIndividual:
		if rule.DispatchRuleIndividual.GetPin() != "" {
			priority = 1
		} else {
			priority = 101
		}
	case *livekit.SIPDispatchRule_DispatchRuleCallee:
		if rule.DispatchRuleCallee.GetPin() != "" {
			priority = 2
		} else {
			priority = 102
		}
	}
	if len(info.InboundNumbers) == 0 {
		priority += 1000
	}
	if len(info.Numbers) == 0 {
		priority += 1000
	}
	return priority
}

func hasHigherPriority(r1, r2 *livekit.SIPDispatchRuleInfo) bool {
	p1, p2 := DispatchRulePriority(r1), DispatchRulePriority(r2)
	if p1 < p2 {
		return true
	} else if p1 > p2 {
		return false
	}
	// For predictable sorting order.
	room1, _, _ := GetPinAndRoom(r1)
	room2, _, _ := GetPinAndRoom(r2)
	return room1 < room2
}

// SortDispatchRules predictably sorts dispatch rules by priority (first one is highest).
func SortDispatchRules(rules []*livekit.SIPDispatchRuleInfo) {
	sort.Slice(rules, func(i, j int) bool {
		return hasHigherPriority(rules[i], rules[j])
	})
}

func printID(s string) string {
	if s == "" {
		return "<new>"
	}
	return s
}

func printName(s string) string {
	if s == "" {
		return "<blank name>"
	}
	return s
}

// ValidateDispatchRules checks a set of dispatch rules for conflicts.
//
// Deprecated: use ValidateDispatchRulesIter
func ValidateDispatchRules(rules []*livekit.SIPDispatchRuleInfo, opts ...MatchDispatchRuleOpt) error {
	_, err := ValidateDispatchRulesIter(iters.Slice(rules), opts...)
	return err
}

// ValidateDispatchRulesIter checks a set of dispatch rules for conflicts.
func ValidateDispatchRulesIter(it iters.Iter[*livekit.SIPDispatchRuleInfo], opts ...MatchDispatchRuleOpt) (best *livekit.SIPDispatchRuleInfo, _ error) {
	it = NewDispatchRuleValidator(opts...).ValidateIter(it)
	defer it.Close()
	for {
		r, err := it.Next()
		if err == io.EOF {
			break
		} else if err != nil {
			return best, err
		}
		if best == nil || hasHigherPriority(r, best) {
			best = r
		}
	}
	return best, nil
}

func NewDispatchRuleValidator(opts ...MatchDispatchRuleOpt) *DispatchRuleValidator {
	var opt matchDispatchRuleOpts
	for _, fnc := range opts {
		fnc(&opt)
	}
	opt.defaults()
	return &DispatchRuleValidator{
		opt:       opt,
		byRuleKey: make(map[dispatchRuleKey]*livekit.SIPDispatchRuleInfo),
	}
}

type dispatchRuleKey struct {
	Pin    string
	Trunk  string
	InboundNumber string
	Number string
}

type DispatchRuleValidator struct {
	opt       matchDispatchRuleOpts
	byRuleKey map[dispatchRuleKey]*livekit.SIPDispatchRuleInfo
}

func (v *DispatchRuleValidator) ValidateIter(it iters.Iter[*livekit.SIPDispatchRuleInfo]) iters.Iter[*livekit.SIPDispatchRuleInfo] {
	return &dispatchRuleValidatorIter{v: v, it: it}
}

func (v *DispatchRuleValidator) Validate(r *livekit.SIPDispatchRuleInfo) error {
	_, pin, err := GetPinAndRoom(r)
	if err != nil {
		return err
	}
	trunks := r.TrunkIds
	if len(trunks) == 0 {
		// This rule matches all trunks, but collides only with other default ones (specific rules take priority).
		trunks = []string{""}
	}
	inboundNumbers := r.InboundNumbers
	if len(inboundNumbers) == 0 {
		// This rule matches all numbers, but collides only with other default ones (specific rules take priority).
		inboundNumbers = []string{""}
	}
	numbers := r.Numbers
	if len(numbers) == 0 {
		// This rule matches all numbers, but collides only with other default ones (specific rules take priority).
		numbers = []string{""}
	}
	for _, trunk := range trunks {
		for _, inboundNumber := range inboundNumbers {
			for _, number := range numbers {
				key := dispatchRuleKey{Pin: pin, Trunk: trunk, Number: NormalizeNumber(number), InboundNumber: NormalizeNumber(inboundNumber)}
				r2 := v.byRuleKey[key]
				if r2 != nil {
					v.opt.Conflict(r, r2, DispatchRuleConflictGeneric)
					if v.opt.AllowConflicts {
						continue
					}
					return twirp.NewErrorf(twirp.InvalidArgument,
						"Dispatch rule for the same trunk, inbound number, number, and PIN combination already exists in dispatch rule %q %q",
						printID(r2.SipDispatchRuleId), printName(r2.Name))
				}
				v.byRuleKey[key] = r
			}
		}
	}
	return nil
}

type dispatchRuleValidatorIter struct {
	v  *DispatchRuleValidator
	it iters.Iter[*livekit.SIPDispatchRuleInfo]
}

func (v *dispatchRuleValidatorIter) Next() (*livekit.SIPDispatchRuleInfo, error) {
	r, err := v.it.Next()
	if err != nil {
		return nil, err
	}
	r = v.v.opt.Replace(r)
	if err = v.v.Validate(r); err != nil {
		return nil, err
	}
	return r, nil
}

func (v *dispatchRuleValidatorIter) Close() {
	v.it.Close()
}

// SelectDispatchRule takes a list of dispatch rules, and takes the decision which one should be selected.
// It returns an error if there are conflicting rules. Returns nil if no rules match.
//
// Deprecated: use MatchDispatchRuleIter
func SelectDispatchRule(rules []*livekit.SIPDispatchRuleInfo, req *rpc.EvaluateSIPDispatchRulesRequest, opts ...MatchDispatchRuleOpt) (*livekit.SIPDispatchRuleInfo, error) {
	// Sorting will do the selection for us. We already filtered out irrelevant ones in MatchDispatchRule and above.
	// Nil is fine here. We will report "no rules matched" later.
	return ValidateDispatchRulesIter(iters.Slice(rules), opts...)
}

// GetPinAndRoom returns a room name/prefix and the pin for a dispatch rule. Just a convenience wrapper.
func GetPinAndRoom(info *livekit.SIPDispatchRuleInfo) (room, pin string, err error) {
	// TODO: Could probably add methods on SIPDispatchRuleInfo struct instead.
	switch rule := info.GetRule().GetRule().(type) {
	default:
		return "", "", fmt.Errorf("Unsupported SIP Dispatch Rule: %T", rule)
	case *livekit.SIPDispatchRule_DispatchRuleDirect:
		pin = rule.DispatchRuleDirect.GetPin()
		room = rule.DispatchRuleDirect.GetRoomName()
	case *livekit.SIPDispatchRule_DispatchRuleIndividual:
		pin = rule.DispatchRuleIndividual.GetPin()
		room = rule.DispatchRuleIndividual.GetRoomPrefix()
	case *livekit.SIPDispatchRule_DispatchRuleCallee:
		pin = rule.DispatchRuleCallee.GetPin()
		room = rule.DispatchRuleCallee.GetRoomPrefix()
	}
	return room, pin, nil
}

func printNumbers(numbers []string) string {
	if len(numbers) == 0 {
		return "<any>"
	}
	return fmt.Sprintf("%q", numbers)
}

// NormalizeNumber normalizes a phone number by removing formatting characters and ensuring it starts with a "+".
// This function delegates to livekit.NormalizeNumber for the actual implementation.
func NormalizeNumber(num string) string {
	return livekit.NormalizeNumber(num)
}

func validateTrunkInbound(byInbound map[string]*livekit.SIPInboundTrunkInfo, t *livekit.SIPInboundTrunkInfo, opt *matchTrunkOpts) error {
	if len(t.AllowedNumbers) == 0 {
		if t2 := byInbound[""]; t2 != nil {
			opt.Conflict(t, t2, TrunkConflictCalledNumber)
			if opt.AllowConflicts {
				return nil
			}
			return twirp.NewErrorf(twirp.InvalidArgument, "Conflicting inbound SIP Trunks: %q and %q, using the same number(s) %s without AllowedNumbers set",
				printID(t.SipTrunkId), printID(t2.SipTrunkId), printNumbers(t.Numbers))
		}
		byInbound[""] = t
	} else {
		for _, num := range t.AllowedNumbers {
			inboundKey := NormalizeNumber(num)
			t2 := byInbound[inboundKey]
			if t2 != nil {
				opt.Conflict(t, t2, TrunkConflictCallingNumber)
				if opt.AllowConflicts {
					continue
				}
				return twirp.NewErrorf(twirp.InvalidArgument, "Conflicting inbound SIP Trunks: %q and %q, using the same number(s) %s and AllowedNumber %q",
					printID(t.SipTrunkId), printID(t2.SipTrunkId), printNumbers(t.Numbers), num)
			}
			byInbound[inboundKey] = t
		}
	}
	return nil
}

// ValidateTrunks checks a set of trunks for conflicts.
//
// Deprecated: use ValidateTrunksIter
func ValidateTrunks(trunks []*livekit.SIPInboundTrunkInfo, opts ...MatchTrunkOpt) error {
	return ValidateTrunksIter(iters.Slice(trunks), opts...)
}

// ValidateTrunksIter checks a set of trunks for conflicts.
func ValidateTrunksIter(it iters.Iter[*livekit.SIPInboundTrunkInfo], opts ...MatchTrunkOpt) error {
	defer it.Close()
	var opt matchTrunkOpts
	for _, fnc := range opts {
		fnc(&opt)
	}
	opt.defaults()
	byOutboundAndInbound := make(map[string]map[string]*livekit.SIPInboundTrunkInfo)
	for {
		t, err := it.Next()
		if err == io.EOF {
			break
		} else if err != nil {
			return err
		}
		t = opt.Replace(t)
		if len(t.Numbers) == 0 {
			byInbound := byOutboundAndInbound[""]
			if byInbound == nil {
				byInbound = make(map[string]*livekit.SIPInboundTrunkInfo)
				byOutboundAndInbound[""] = byInbound
			}
			if err := validateTrunkInbound(byInbound, t, &opt); err != nil {
				return err
			}
		} else {
			for _, num := range t.Numbers {
				byInbound := byOutboundAndInbound[num]
				if byInbound == nil {
					byInbound = make(map[string]*livekit.SIPInboundTrunkInfo)
					byOutboundAndInbound[num] = byInbound
				}
				if err := validateTrunkInbound(byInbound, t, &opt); err != nil {
					return err
				}
			}
		}
	}
	return nil
}

func isValidMask(mask string) bool {
	// Allowed formats:
	// - 1.2.3.4
	// - 1.2.3.4/8
	// - [::]
	// - [::]/8
	// - some.host.name
	if strings.ContainsAny(mask, "()+*;, \t\n\r") {
		return false
	}
	if strings.Contains(mask, "://") {
		return false
	}
	return true
}

func filterInvalidAddrMasks(masks []string) []string {
	if len(masks) == 0 {
		return nil
	}
	out := make([]string, 0, len(masks))
	for _, m := range masks {
		if isValidMask(m) {
			out = append(out, m)
		}
	}
	return out
}

func matchAddrMask(ip netip.Addr, mask string) bool {
	if !strings.Contains(mask, "/") {
		expIP, err := netip.ParseAddr(mask)
		if err != nil {
			return false
		}
		return ip == expIP
	}
	pref, err := netip.ParsePrefix(mask)
	if err != nil {
		return false
	}
	return pref.Contains(ip)
}

func matchAddrMasks(addr string, host string, masks []string) bool {
	ip, err := netip.ParseAddr(addr)
	if err != nil {
		return true
	}
	masks = filterInvalidAddrMasks(masks)
	if len(masks) == 0 {
		return true
	}
	for _, mask := range masks {
		if mask == host || matchAddrMask(ip, mask) {
			return true
		}
	}
	return false
}

func matchNumbers(num string, allowed []string) bool {
	if len(allowed) == 0 {
		return true
	}
	norm := NormalizeNumber(num)
	for _, allow := range allowed {
		if num == allow || norm == NormalizeNumber(allow) {
			return true
		}
	}
	return false
}

// TrunkMatchType indicates how a trunk was matched
type TrunkMatchType int

const (
	// TrunkMatchEmpty indicates no trunks were defined
	TrunkMatchEmpty TrunkMatchType = iota
	// TrunkMatchNone indicates trunks exist but none matched
	TrunkMatchNone
	// TrunkMatchDefault indicates only a default trunk (with no specific numbers) matched
	TrunkMatchDefault
	// TrunkMatchSpecific indicates a trunk with specific numbers matched
	TrunkMatchSpecific
)

// TrunkMatchResult provides detailed information about the trunk matching process
type TrunkMatchResult struct {
	// The matched trunk, if any
	Trunk *livekit.SIPInboundTrunkInfo
	// How the trunk was matched
	MatchType TrunkMatchType
	// Number of default trunks found
	DefaultTrunkCount int
}

// MatchTrunk finds a SIP Trunk definition matching the request.
// Returns nil if no rules matched or an error if there are conflicting definitions.
//
// Deprecated: use MatchTrunkIter
func MatchTrunk(trunks []*livekit.SIPInboundTrunkInfo, call *rpc.SIPCall, opts ...MatchTrunkOpt) (*livekit.SIPInboundTrunkInfo, error) {
	return MatchTrunkIter(iters.Slice(trunks), call, opts...)
}

// MatchTrunkDetailed is like MatchTrunkIter but returns detailed match information
func MatchTrunkDetailed(it iters.Iter[*livekit.SIPInboundTrunkInfo], call *rpc.SIPCall, opts ...MatchTrunkOpt) (*TrunkMatchResult, error) {
	defer it.Close()
	var opt matchTrunkOpts
	for _, fnc := range opts {
		fnc(&opt)
	}
	opt.defaults()

	result := &TrunkMatchResult{
		MatchType: TrunkMatchEmpty, // Start with assumption it's empty
	}

	var (
		selectedTrunk    *livekit.SIPInboundTrunkInfo
		defaultTrunk     *livekit.SIPInboundTrunkInfo
		defaultTrunkPrev *livekit.SIPInboundTrunkInfo
		sawAnyTrunk      bool
	)
	calledNorm := NormalizeNumber(call.To.User)
	for {
		tr, err := it.Next()
		if err == io.EOF {
			break
		} else if err != nil {
			return nil, err
		}
		if !sawAnyTrunk {
			sawAnyTrunk = true
			result.MatchType = TrunkMatchNone // We have trunks but haven't matched any yet
		}
		tr = opt.Replace(tr)
		// Do not consider it if number doesn't match.
		if !matchNumbers(call.From.User, tr.AllowedNumbers) {
			if !opt.Filtered(tr, TrunkFilteredCallingNumberDisallowed) {
				continue
			}
		}
		if !matchAddrMasks(call.SourceIp, call.From.Host, tr.AllowedAddresses) {
			if !opt.Filtered(tr, TrunkFilteredSourceAddressDisallowed) {
				continue
			}
		}
		if len(tr.Numbers) == 0 {
			// Default/wildcard trunk.
			defaultTrunkPrev = defaultTrunk
			defaultTrunk = tr
			result.DefaultTrunkCount++
		} else {
			for _, num := range tr.Numbers {
				if num == call.To.User || NormalizeNumber(num) == calledNorm {
					// Trunk specific to the number.
					if selectedTrunk != nil {
						opt.Conflict(selectedTrunk, tr, TrunkConflictCalledNumber)
						if opt.AllowConflicts {
							// This path is unreachable, since we pick the first trunk. Kept for completeness.
							continue
						}
						return nil, twirp.NewErrorf(twirp.FailedPrecondition, "Multiple SIP Trunks matched for %q", call.To.User)
					}
					selectedTrunk = tr
					if opt.AllowConflicts {
						// Pick the first match as soon as it's found. We don't care about conflicts.
						result.Trunk = selectedTrunk
						result.MatchType = TrunkMatchSpecific
						return result, nil
					}
					// Keep searching! We want to know if there are any conflicting Trunk definitions.
				} else {
					opt.Filtered(tr, TrunkFilteredCalledNumberDisallowed)
				}
			}
		}
	}

	if selectedTrunk != nil {
		result.Trunk = selectedTrunk
		result.MatchType = TrunkMatchSpecific
		return result, nil
	}
	if result.DefaultTrunkCount > 1 {
		opt.Conflict(defaultTrunk, defaultTrunkPrev, TrunkConflictDefault)
		if !opt.AllowConflicts {
			return nil, twirp.NewErrorf(twirp.FailedPrecondition, "Multiple default SIP Trunks matched for %q", call.To.User)
		}
	}
	if defaultTrunk != nil {
		result.Trunk = defaultTrunk
		result.MatchType = TrunkMatchDefault
	}
	return result, nil
}

type matchTrunkOpts struct {
	AllowConflicts bool
	Filtered       TrunkFilteredFunc
	Conflict       TrunkConflictFunc
	Replace        TrunkReplaceFunc
}

func (opt *matchTrunkOpts) defaults() {
	if opt.Filtered == nil {
		opt.Filtered = func(_ *livekit.SIPInboundTrunkInfo, _ TrunkFilteredReason) bool {
			return false
		}
	}
	if opt.Conflict == nil {
		opt.Conflict = func(_, _ *livekit.SIPInboundTrunkInfo, _ TrunkConflictReason) {}
	}
	if opt.Replace == nil {
		opt.Replace = func(t *livekit.SIPInboundTrunkInfo) *livekit.SIPInboundTrunkInfo {
			return t
		}
	}
}

type MatchTrunkOpt func(opt *matchTrunkOpts)

type TrunkFilteredReason int

const (
	TrunkFilteredInvalid = TrunkFilteredReason(iota)
	TrunkFilteredCallingNumberDisallowed
	TrunkFilteredCalledNumberDisallowed
	TrunkFilteredSourceAddressDisallowed
)

type TrunkFilteredFunc func(tr *livekit.SIPInboundTrunkInfo, reason TrunkFilteredReason) bool

// WithTrunkFiltered sets a callback that is called when selected Trunk(s) doesn't match the call.
// If the callback returns true, trunk will not be filtered.
func WithTrunkFiltered(fnc TrunkFilteredFunc) MatchTrunkOpt {
	return func(opt *matchTrunkOpts) {
		opt.Filtered = fnc
	}
}

type TrunkConflictReason int

const (
	TrunkConflictDefault = TrunkConflictReason(iota)
	TrunkConflictCalledNumber
	TrunkConflictCallingNumber
)

type TrunkConflictFunc func(t1, t2 *livekit.SIPInboundTrunkInfo, reason TrunkConflictReason)

// WithAllowTrunkConflicts allows conflicting Trunk definitions by picking the first match.
//
// Using this option will prevent TrunkConflictFunc from firing, since the first match will be returned immediately.
func WithAllowTrunkConflicts() MatchTrunkOpt {
	return func(opt *matchTrunkOpts) {
		opt.AllowConflicts = true
	}
}

// WithTrunkConflict sets a callback that is called when two Trunks conflict.
func WithTrunkConflict(fnc TrunkConflictFunc) MatchTrunkOpt {
	return func(opt *matchTrunkOpts) {
		opt.Conflict = fnc
	}
}

type TrunkReplaceFunc func(t *livekit.SIPInboundTrunkInfo) *livekit.SIPInboundTrunkInfo

// WithTrunkReplace sets a callback that is called to potentially replace trunks before matching runs.
func WithTrunkReplace(fnc TrunkReplaceFunc) MatchTrunkOpt {
	return func(opt *matchTrunkOpts) {
		opt.Replace = fnc
	}
}

// MatchTrunkIter finds a SIP Trunk definition matching the request.
// Returns nil if no rules matched or an error if there are conflicting definitions.
func MatchTrunkIter(it iters.Iter[*livekit.SIPInboundTrunkInfo], call *rpc.SIPCall, opts ...MatchTrunkOpt) (*livekit.SIPInboundTrunkInfo, error) {
	result, err := MatchTrunkDetailed(it, call, opts...)
	if err != nil {
		return nil, err
	}
	return result.Trunk, nil
}

// MatchDispatchRule finds the best dispatch rule matching the request parameters. Returns an error if no rule matched.
// Trunk parameter can be nil, in which case only wildcard dispatch rules will be effective (ones without Trunk IDs).
//
// Deprecated: use MatchDispatchRuleIter
func MatchDispatchRule(trunk *livekit.SIPInboundTrunkInfo, rules []*livekit.SIPDispatchRuleInfo, req *rpc.EvaluateSIPDispatchRulesRequest, opts ...MatchDispatchRuleOpt) (*livekit.SIPDispatchRuleInfo, error) {
	return MatchDispatchRuleIter(trunk, iters.Slice(rules), req, opts...)
}

type matchDispatchRuleOpts struct {
	AllowConflicts bool
	Conflict       DispatchRuleConflictFunc
	Replace        DispatchRuleReplaceFunc
}

func (opt *matchDispatchRuleOpts) defaults() {
	if opt.Conflict == nil {
		opt.Conflict = func(_, _ *livekit.SIPDispatchRuleInfo, _ DispatchRuleConflictReason) {}
	}
	if opt.Replace == nil {
		opt.Replace = func(r *livekit.SIPDispatchRuleInfo) *livekit.SIPDispatchRuleInfo {
			return r
		}
	}
}

type MatchDispatchRuleOpt func(opt *matchDispatchRuleOpts)

type DispatchRuleConflictReason int

const (
	DispatchRuleConflictGeneric = DispatchRuleConflictReason(iota)
)

type DispatchRuleConflictFunc func(r1, r2 *livekit.SIPDispatchRuleInfo, reason DispatchRuleConflictReason)

// WithAllowDispatchRuleConflicts allows conflicting DispatchRule definitions.
func WithAllowDispatchRuleConflicts() MatchDispatchRuleOpt {
	return func(opt *matchDispatchRuleOpts) {
		opt.AllowConflicts = true
	}
}

// WithDispatchRuleConflict sets a callback that is called when two DispatchRules conflict.
func WithDispatchRuleConflict(fnc DispatchRuleConflictFunc) MatchDispatchRuleOpt {
	return func(opt *matchDispatchRuleOpts) {
		opt.Conflict = fnc
	}
}

type DispatchRuleReplaceFunc func(r *livekit.SIPDispatchRuleInfo) *livekit.SIPDispatchRuleInfo

// WithDispatchRuleReplace sets a callback that is called to potentially replace dispatch rules before matching runs.
func WithDispatchRuleReplace(fnc DispatchRuleReplaceFunc) MatchDispatchRuleOpt {
	return func(opt *matchDispatchRuleOpts) {
		opt.Replace = fnc
	}
}

// MatchDispatchRuleIter finds the best dispatch rule matching the request parameters. Returns an error if no rule matched.
// Trunk parameter can be nil, in which case only wildcard dispatch rules will be effective (ones without Trunk IDs).
func MatchDispatchRuleIter(trunk *livekit.SIPInboundTrunkInfo, rules iters.Iter[*livekit.SIPDispatchRuleInfo], req *rpc.EvaluateSIPDispatchRulesRequest, opts ...MatchDispatchRuleOpt) (*livekit.SIPDispatchRuleInfo, error) {
	rules = NewDispatchRuleValidator(opts...).ValidateIter(rules)
	defer rules.Close()
	// Trunk can still be nil here in case none matched or were defined.
	// This is still fine, but only in case we'll match exactly one wildcard dispatch rule.

	// We split the matched dispatch rules into two sets in relation to Trunks: specific and default (aka wildcard).
	// First, attempt to match any of the specific rules, where we did match the Trunk ID.
	// If nothing matches there - fallback to default/wildcard rules, where no Trunk IDs were mentioned.
	var (
		specificRule    *livekit.SIPDispatchRuleInfo
		specificRuleCnt int
		defaultRule     *livekit.SIPDispatchRuleInfo
		defaultRuleCnt  int
	)
	noPin := req.NoPin
	sentPin := req.GetPin()
	for {
		info, err := rules.Next()
		if err == io.EOF {
			break
		} else if err != nil {
			return nil, err
		}
		if len(info.InboundNumbers) != 0 && !slices.Contains(info.InboundNumbers, req.CallingNumber) {
			continue
		}
		if len(info.Numbers) != 0 && !slices.Contains(info.Numbers, req.CalledNumber) {
			continue
		}
		_, rulePin, err := GetPinAndRoom(info)
		if err != nil {
			logger.Errorw("Invalid SIP Dispatch Rule", err, "dispatchRuleID", info.SipDispatchRuleId)
			continue
		}
		// Filter heavily on the Pin, so that only relevant rules remain.
		if noPin {
			if rulePin != "" {
				// Skip pin-protected rules if no pin mode requested.
				continue
			}
		} else if sentPin != "" {
			if rulePin == "" {
				// Pin already sent, skip non-pin-protected rules.
				continue
			}
			if sentPin != rulePin {
				// Pin doesn't match. Don't return an error here, just wait for other rule to match (or none at all).
				// Note that we will NOT match non-pin-protected rules, thus it will not fallback to open rules.
				continue
			}
		}
		if len(info.TrunkIds) == 0 {
			// Default/wildcard dispatch rule.
			defaultRuleCnt++
			if defaultRule == nil || hasHigherPriority(info, defaultRule) {
				defaultRule = info
			}
			continue
		}
		// Specific dispatch rules. Require a Trunk associated with the number.
		if trunk == nil {
			continue
		}
		if !slices.Contains(info.TrunkIds, trunk.SipTrunkId) {
			continue
		}
		specificRuleCnt++
		if specificRule == nil || hasHigherPriority(info, specificRule) {
			specificRule = info
		}
	}
	if specificRuleCnt == 0 && defaultRuleCnt == 0 {
		err := &ErrNoDispatchMatched{NoRules: true, NoTrunks: trunk == nil, CalledNumber: req.CalledNumber}
		return nil, twirp.WrapError(twirp.NewErrorf(twirp.FailedPrecondition, "%s", err.Error()), err)
	}
	if specificRule != nil {
		return specificRule, nil
	}
	if defaultRule != nil {
		return defaultRule, nil
	}
	err := &ErrNoDispatchMatched{NoRules: false, NoTrunks: trunk == nil, CalledNumber: req.CalledNumber}
	return nil, twirp.WrapError(twirp.NewErrorf(twirp.FailedPrecondition, "%s", err.Error()), err)
}

// EvaluateDispatchRule checks a selected Dispatch Rule against the provided request.
func EvaluateDispatchRule(projectID string, trunk *livekit.SIPInboundTrunkInfo, rule *livekit.SIPDispatchRuleInfo, req *rpc.EvaluateSIPDispatchRulesRequest) (*rpc.EvaluateSIPDispatchRulesResponse, error) {
	call := req.SIPCall()
	sentPin := req.GetPin()

	trunkID := req.SipTrunkId
	if trunk != nil {
		trunkID = trunk.SipTrunkId
	}
	enc := livekit.SIPMediaEncryption_SIP_MEDIA_ENCRYPT_DISABLE
	if trunk != nil {
		enc = trunk.MediaEncryption
	}
	attrs := maps.Clone(rule.Attributes)
	if attrs == nil {
		attrs = make(map[string]string)
	}
	for k, v := range req.ExtraAttributes {
		attrs[k] = v
	}
	attrs[livekit.AttrSIPCallID] = call.LkCallId
	attrs[livekit.AttrSIPTrunkID] = trunkID

	to := call.To.User
	from := call.From.User
	fromName := "Phone " + from
	fromID := "sip_" + from
	if rule.HidePhoneNumber {
		// Mask the phone number, hash identity. Omit number in attrs.
		h := sha256.Sum256([]byte(call.From.User))
		fromID = "sip_" + hex.EncodeToString(h[:8])
		// TODO: Maybe keep regional code, but mask all but 4 last digits?
		n := 4
		if len(from) <= 4 {
			n = 1
		}
		from = from[len(from)-n:]
		fromName = "Phone " + from
	} else {
		attrs[livekit.AttrSIPPhoneNumber] = call.From.User
		attrs[livekit.AttrSIPHostName] = call.From.Host
		attrs[livekit.AttrSIPTrunkNumber] = call.To.User
	}

	room, rulePin, err := GetPinAndRoom(rule)
	if err != nil {
		return nil, err
	}
	if rulePin != "" {
		if sentPin == "" {
			return &rpc.EvaluateSIPDispatchRulesResponse{
				ProjectId:         projectID,
				SipTrunkId:        trunkID,
				SipDispatchRuleId: rule.SipDispatchRuleId,
				Result:            rpc.SIPDispatchResult_REQUEST_PIN,
				MediaEncryption:   enc,
				RequestPin:        true,
			}, nil
		}
		if rulePin != sentPin {
			// This should never happen in practice, because matchSIPDispatchRule should remove rules with the wrong pin.
			return nil, twirp.NewError(twirp.PermissionDenied, "Incorrect PIN for SIP room")
		}
	} else {
		// Pin was sent, but room doesn't require one. Assume user accidentally pressed phone button.
	}
	switch rule := rule.GetRule().GetRule().(type) {
	case *livekit.SIPDispatchRule_DispatchRuleIndividual:
		// TODO: Remove "_" if the prefix is empty for consistency with Callee dispatch rule.
		// TODO: Do we need to escape specific characters in the number?
		// TODO: Include actual SIP call ID in the room name?
		room = fmt.Sprintf("%s_%s_%s", rule.DispatchRuleIndividual.GetRoomPrefix(), from, guid.New(""))
	case *livekit.SIPDispatchRule_DispatchRuleCallee:
		room = to
		if pref := rule.DispatchRuleCallee.GetRoomPrefix(); pref != "" {
			room = pref + "_" + to
		}
		if rule.DispatchRuleCallee.Randomize {
			room += "_" + guid.New("")
		}
	}
	attrs[livekit.AttrSIPDispatchRuleID] = rule.SipDispatchRuleId
	resp := &rpc.EvaluateSIPDispatchRulesResponse{
		ProjectId:             projectID,
		SipTrunkId:            trunkID,
		SipDispatchRuleId:     rule.SipDispatchRuleId,
		Result:                rpc.SIPDispatchResult_ACCEPT,
		RoomName:              room,
		ParticipantIdentity:   fromID,
		ParticipantName:       fromName,
		ParticipantMetadata:   rule.Metadata,
		ParticipantAttributes: attrs,
		RoomPreset:            rule.RoomPreset,
		RoomConfig:            rule.RoomConfig,
		MediaEncryption:       enc,
	}
	krispEnabled := false
	if trunk != nil {
		resp.Headers = trunk.Headers
		resp.HeadersToAttributes = trunk.HeadersToAttributes
		resp.AttributesToHeaders = trunk.AttributesToHeaders
		resp.IncludeHeaders = trunk.IncludeHeaders
		resp.RingingTimeout = trunk.RingingTimeout
		resp.MaxCallDuration = trunk.MaxCallDuration
		krispEnabled = krispEnabled || trunk.KrispEnabled
	}
	if rule != nil {
		krispEnabled = krispEnabled || rule.KrispEnabled
		if rule.MediaEncryption != 0 {
			resp.MediaEncryption = rule.MediaEncryption
		}
	}
	if krispEnabled {
		resp.EnabledFeatures = append(resp.EnabledFeatures, livekit.SIPFeature_KRISP_ENABLED)
	}
	return resp, nil
}
