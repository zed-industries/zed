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

package livekit

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"slices"
	"time"

	"buf.build/go/protoyaml"
	"github.com/dennwc/iters"
	"go.opentelemetry.io/otel/attribute"
	proto "google.golang.org/protobuf/proto"
	"gopkg.in/yaml.v3"
)

const (
	TraceKeyPref = "lk."

	TraceKeyRoomPrefix = TraceKeyPref + "room."
	TraceKeyRoomID     = attribute.Key(TraceKeyRoomPrefix + "id")
	TraceKeyRoomName   = attribute.Key(TraceKeyRoomPrefix + "name")

	TraceKeyParticipantPrefix   = TraceKeyPref + "participant."
	TraceKeyParticipantID       = attribute.Key(TraceKeyParticipantPrefix + "id")
	TraceKeyParticipantIdentity = attribute.Key(TraceKeyParticipantPrefix + "identity")

	TraceKeyTrackPrefix = TraceKeyPref + "track."
	TraceKeyTrackID     = attribute.Key(TraceKeyTrackPrefix + "id")

	TraceKeySIPPrefix       = TraceKeyPref + "sip."
	TraceKeySIPHeaderPrefix = TraceKeySIPPrefix + "h."
	TraceKeySIPCallID       = attribute.Key(TraceKeySIPPrefix + "call.id")
	TraceKeySIPCallIDHeader = attribute.Key(TraceKeySIPHeaderPrefix + "CallID")
)

type TrackID string
type ParticipantID string
type ParticipantIdentity string
type ParticipantName string
type RoomID string
type RoomName string
type ConnectionID string
type NodeID string
type RoomKey struct {
	ProjectID string
	RoomName  RoomName
}
type ParticipantKey struct {
	RoomKey
	Identity ParticipantIdentity
}
type JobID string
type DispatchID string
type AgentName string
type SIPCallID string
type SIPCallIDHeader string

func (s TrackID) String() string             { return string(s) }
func (s ParticipantID) String() string       { return string(s) }
func (s ParticipantIdentity) String() string { return string(s) }
func (s ParticipantName) String() string     { return string(s) }
func (s RoomID) String() string              { return string(s) }
func (s RoomName) String() string            { return string(s) }
func (s ConnectionID) String() string        { return string(s) }
func (s NodeID) String() string              { return string(s) }
func (s JobID) String() string               { return string(s) }
func (s DispatchID) String() string          { return string(s) }
func (s AgentName) String() string           { return string(s) }
func (s SIPCallID) String() string           { return string(s) }
func (s SIPCallIDHeader) String() string     { return string(s) }
func (s ParticipantKey) String() string {
	return fmt.Sprintf("%s_%s_%s", s.ProjectID, s.RoomName, s.Identity)
}

func (s ParticipantID) Trace() attribute.KeyValue {
	return TraceKeyParticipantID.String(string(s))
}
func (s ParticipantIdentity) Trace() attribute.KeyValue {
	return TraceKeyParticipantIdentity.String(string(s))
}
func (s RoomID) Trace() attribute.KeyValue {
	return TraceKeyRoomID.String(string(s))
}
func (s RoomName) Trace() attribute.KeyValue {
	return TraceKeyRoomName.String(string(s))
}
func (s TrackID) Trace() attribute.KeyValue {
	return TraceKeyTrackID.String(string(s))
}
func (s SIPCallID) Trace() attribute.KeyValue {
	return TraceKeySIPCallID.String(string(s))
}
func (s SIPCallIDHeader) Trace() attribute.KeyValue {
	return TraceKeySIPCallIDHeader.String(string(s))
}

type stringTypes interface {
	ParticipantID | RoomID | TrackID | ParticipantIdentity | ParticipantName | RoomName | ConnectionID | NodeID
}

func IDsAsStrings[T stringTypes](ids []T) []string {
	strs := make([]string, 0, len(ids))
	for _, id := range ids {
		strs = append(strs, string(id))
	}
	return strs
}

func StringsAsIDs[T stringTypes](ids []string) []T {
	asID := make([]T, 0, len(ids))
	for _, id := range ids {
		asID = append(asID, T(id))
	}

	return asID
}

type Guid interface {
	TrackID | ParticipantID | RoomID
}

type GuidBlock [9]byte

func (r *RoomConfiguration) UnmarshalYAML(value *yaml.Node) error {
	// Marshall the Node back to yaml to pass it to the protobuf specific unmarshaller
	str, err := yaml.Marshal(value)
	if err != nil {
		return err
	}

	return protoyaml.Unmarshal(str, r)
}

func (r *RoomConfiguration) MarshalYAML() (interface{}, error) {
	return marshalProto(r)
}

func (r *RoomEgress) UnmarshalYAML(value *yaml.Node) error {
	// Marshall the Node back to yaml to pass it to the protobuf specific unmarshaller
	str, err := yaml.Marshal(value)
	if err != nil {
		return err
	}

	return protoyaml.Unmarshal(str, r)
}

func (r *RoomEgress) MarshalYAML() (interface{}, error) {
	return marshalProto(r)
}

func (r *RoomAgent) UnmarshalYAML(value *yaml.Node) error {
	// Marshall the Node back to yaml to pass it to the protobuf specific unmarshaller
	str, err := yaml.Marshal(value)
	if err != nil {
		return err
	}

	return protoyaml.Unmarshal(str, r)
}

func (r *RoomAgent) MarshalYAML() (interface{}, error) {
	return marshalProto(r)
}

func marshalProto(o proto.Message) (map[string]interface{}, error) {
	// Marshall the Node to yaml using the protobuf specific marshaller to ensure the proper field names are used
	str, err := protoyaml.MarshalOptions{UseProtoNames: true}.Marshal(o)
	if err != nil {
		return nil, err
	}

	m := make(map[string]interface{})

	err = yaml.Unmarshal(str, &m)
	if err != nil {
		return nil, err
	}

	return m, nil
}

func cloneProto[T proto.Message](m T) T {
	return proto.Clone(m).(T)
}

func IsJobType(jobType JobType) bool {
	_, ok := JobType_name[int32(jobType)]
	return ok
}

func (p *Pagination) Filter(v PageItem) bool {
	if p == nil {
		return true
	}
	if p.AfterId != "" {
		if id := v.ID(); id != "" && id <= p.AfterId {
			return false
		}
	}
	return true
}

// TokenPaginationData represents the data encoded in a TokenPagination token
type TokenPaginationData struct {
	Offset int32 `json:"offset"`
	Limit  int32 `json:"limit"`
}

// EncodeTokenPagination encodes offset and limit into a TokenPagination.
// The token is a base64-encoded JSON object containing the offset and limit values.
func EncodeTokenPagination(offset, limit int32) (*TokenPagination, error) {
	data := TokenPaginationData{
		Offset: offset,
		Limit:  limit,
	}

	jsonData, err := json.Marshal(data)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal token pagination data: %w", err)
	}

	token := base64.URLEncoding.EncodeToString(jsonData)
	return &TokenPagination{Token: token}, nil
}

// DecodeTokenPagination decodes a TokenPagination into offset and limit.
// Returns an error if the token is invalid or cannot be decoded.
// If the TokenPagination is nil or has an empty token, returns zero values without error.
func DecodeTokenPagination(tp *TokenPagination) (offset, limit int32, err error) {
	if tp == nil || tp.Token == "" {
		return 0, 0, nil
	}

	decoded, err := base64.URLEncoding.DecodeString(tp.Token)
	if err != nil {
		return 0, 0, fmt.Errorf("failed to decode token: %w", err)
	}

	var data TokenPaginationData
	if err := json.Unmarshal(decoded, &data); err != nil {
		return 0, 0, fmt.Errorf("failed to unmarshal token pagination data: %w", err)
	}

	return data.Offset, data.Limit, nil
}

// CursorTokenData holds the encoded pieces used for cursor pagination.
//
// Instead of paginating with an offset, cursor pagination represents
// a position in an ordered list. The server returns a token for the last item in
// a page, and the client sends it back to request the next page.
//
// A cursor token contains:
//   - sort_key: the value of the primary ordering column (e.g. created_at)
//   - tie_breaker: a secondary stable key (usually a unique ID) to make ordering
//     deterministic when multiple rows share the same sort_key value
//
// This matches common SQL ordering like:
//
//	ORDER BY created_at DESC, id DESC
//
// and next-page predicate like:
//
//	WHERE (created_at, id) < (:created_at, :id)
//
// (for descending order).
type CursorTokenData struct {
	SortKey    string `json:"sort_key"`
	TieBreaker string `json:"tie_breaker"`
}

// ErrNoCursor indicates that no cursor was provided (e.g. first page request).
var ErrNoCursor = errors.New("no cursor")

// EncodeTokenPaginationWithCursor encodes cursor token data into a TokenPagination.
func EncodeTokenPaginationWithCursor(data CursorTokenData) (*TokenPagination, error) {
	token, err := EncodeCursorToken(data)
	if err != nil {
		return nil, err
	}
	return &TokenPagination{Token: token}, nil
}

// DecodeTokenPaginationWithCursor decodes a TokenPagination into CursorTokenData.
// Returns ErrNoCursor if the TokenPagination is nil or has an empty token.
func DecodeTokenPaginationWithCursor(tp *TokenPagination) (data CursorTokenData, err error) {
	if tp == nil || tp.Token == "" {
		return CursorTokenData{}, ErrNoCursor
	}
	data, err = DecodeCursorToken(tp.Token)
	if err != nil {
		return CursorTokenData{}, err
	}
	return data, nil
}

func EncodeCursorToken(data CursorTokenData) (string, error) {
	jsonData, err := json.Marshal(data)
	if err != nil {
		return "", fmt.Errorf("failed to marshal cursor token data: %w", err)
	}
	return base64.URLEncoding.EncodeToString(jsonData), nil
}

func DecodeCursorToken(token string) (CursorTokenData, error) {
	decoded, err := base64.URLEncoding.DecodeString(token)
	if err != nil {
		return CursorTokenData{}, fmt.Errorf("failed to decode cursor token: %w", err)
	}
	var data CursorTokenData
	if err := json.Unmarshal(decoded, &data); err != nil {
		return CursorTokenData{}, fmt.Errorf("failed to unmarshal cursor token data: %w", err)
	}
	return data, nil
}

type ScalarCursorCodec[T any] struct {
	Encode func(T) (string, error)
	Decode func(string) (T, error)
}

type CursorCodec[S, T any] struct {
	SortKey    ScalarCursorCodec[S]
	TieBreaker ScalarCursorCodec[T]
}

func (c CursorCodec[P, T]) Decode(tp *TokenPagination) (primary P, tie T, err error) {
	data, err := DecodeTokenPaginationWithCursor(tp)
	if err != nil {
		return *new(P), *new(T), err
	}
	if data.SortKey == "" && data.TieBreaker == "" {
		return *new(P), *new(T), ErrNoCursor
	}

	primary, err = c.SortKey.Decode(data.SortKey)
	if err != nil {
		return *new(P), *new(T), fmt.Errorf("decode primary: %w", err)
	}
	tie, err = c.TieBreaker.Decode(data.TieBreaker)
	if err != nil {
		return *new(P), *new(T), fmt.Errorf("decode tie: %w", err)
	}
	return primary, tie, nil
}

func (c CursorCodec[P, T]) Encode(primary P, tie T) (*TokenPagination, error) {
	sortKey, err := c.SortKey.Encode(primary)
	if err != nil {
		return nil, fmt.Errorf("encode primary: %w", err)
	}
	tieKey, err := c.TieBreaker.Encode(tie)
	if err != nil {
		return nil, fmt.Errorf("encode tie: %w", err)
	}
	if sortKey == "" && tieKey == "" {
		return nil, nil
	}
	return EncodeTokenPaginationWithCursor(CursorTokenData{
		SortKey:    sortKey,
		TieBreaker: tieKey,
	})
}

var StringCursorCodec = ScalarCursorCodec[string]{
	Encode: func(s string) (string, error) { return s, nil },
	Decode: func(s string) (string, error) { return s, nil },
}

var TimeRFC3339NanoCursorCodec = ScalarCursorCodec[time.Time]{
	Encode: func(t time.Time) (string, error) { return t.UTC().Format(time.RFC3339Nano), nil },
	Decode: func(s string) (time.Time, error) { return time.Parse(time.RFC3339Nano, s) },
}

type pageIterReq[T any] interface {
	GetPage() *Pagination
	Filter(v T) bool
}

type pageIterResp[T any] interface {
	GetItems() []T
}

type PageItem interface {
	ID() string
}

func ListPageIter[T PageItem, Req pageIterReq[T], Resp pageIterResp[T]](fnc func(ctx context.Context, req Req) (Resp, error), req Req) iters.PageIter[T] {
	it := &listPageIter[T, Req, Resp]{fnc: fnc, req: req}
	return iters.FilterPage(it, func(v T) bool {
		return req.Filter(v)
	})
}

type listPageIter[T PageItem, Req pageIterReq[T], Resp pageIterResp[T]] struct {
	fnc  func(ctx context.Context, opts Req) (Resp, error)
	req  Req
	done bool
}

func (it *listPageIter[T, Req, Resp]) NextPage(ctx context.Context) ([]T, error) {
	if it.done {
		return nil, io.EOF
	}
	opts := it.req.GetPage()
	resp, err := it.fnc(ctx, it.req)
	page := resp.GetItems()
	if opts == nil {
		// No pagination set - returns all items.
		// We have to do this to support legacy implementations.
		it.done = true
		return page, err
	}
	// Advance pagination cursor.
	hasID := false
	for i := len(page) - 1; i >= 0; i-- {
		if id := page[i].ID(); id > opts.AfterId {
			opts.AfterId = id
			hasID = true
		}
	}
	if err == nil && (len(page) == 0 || !hasID) {
		err = io.EOF
		it.done = true
	}
	return page, err
}

func (it *listPageIter[_, _, _]) Close() {
	it.done = true
}

func (p *ListUpdate) Validate() error {
	if p == nil {
		return nil
	}
	change := len(p.Set)+len(p.Add)+len(p.Remove) > 0
	if !p.Clear && !change {
		return fmt.Errorf("unsupported list update operation")
	}
	if p.Clear && change {
		return fmt.Errorf("cannot clear and change the list at the same time")
	}
	if len(p.Set) > 0 && len(p.Add)+len(p.Remove) > 0 {
		return fmt.Errorf("cannot set and change the list at the same time")
	}
	for _, v := range p.Set {
		if v == "" {
			return fmt.Errorf("empty element in the list")
		}
	}
	for _, v := range p.Add {
		if v == "" {
			return fmt.Errorf("empty element in the list")
		}
	}
	for _, v := range p.Remove {
		if v == "" {
			return fmt.Errorf("empty element in the list")
		}
	}
	return nil
}

func (p *ListUpdate) Apply(arr []string) ([]string, error) {
	if err := p.Validate(); err != nil {
		return arr, err
	}
	applyListUpdate(&arr, p)
	return arr, nil
}

func applyUpdate[T any](dst *T, set *T) {
	if set != nil {
		*dst = *set
	}
}

func applyUpdatePtr[T any](dst **T, set *T) {
	if set != nil {
		*dst = set
	}
}

func applyListUpdate[T ~string](dst *[]T, u *ListUpdate) {
	if u == nil {
		return
	}
	if u.Clear {
		*dst = nil
		return
	}
	if len(u.Set) != 0 {
		arr := make([]T, 0, len(u.Set))
		for _, v := range u.Set {
			arr = append(arr, T(v))
		}
		*dst = arr
		return
	}
	arr := slices.Clone(*dst)
	for _, v := range u.Remove {
		if i := slices.Index(arr, T(v)); i >= 0 {
			arr = slices.Delete(arr, i, i+1)
		}
	}
	for _, v := range u.Add {
		if i := slices.Index(arr, T(v)); i < 0 {
			arr = append(arr, T(v))
		}
	}
	*dst = arr
}

func applyMapDiff(dst *map[string]string, diff map[string]string) {
	m := *dst
	for k, v := range diff {
		if v != "" {
			if m == nil {
				m = make(map[string]string)
			}
			m[k] = v
		} else {
			delete(m, k)
		}
	}
	*dst = m
}

// ToProto implements DataPacket in Go SDK.
func (p *ChatMessage) ToProto() *DataPacket {
	return &DataPacket{
		Value: &DataPacket_ChatMessage{
			ChatMessage: p,
		},
	}
}

// ToProto implements DataPacket in Go SDK.
func (p *DataStream_Header) ToProto() *DataPacket {
	return &DataPacket{
		Value: &DataPacket_StreamHeader{
			StreamHeader: p,
		},
	}
}

// ToProto implements DataPacket in Go SDK.
func (p *DataStream_Chunk) ToProto() *DataPacket {
	return &DataPacket{
		Value: &DataPacket_StreamChunk{
			StreamChunk: p,
		},
	}
}

// ToProto implements DataPacket in Go SDK.
func (p *DataStream_Trailer) ToProto() *DataPacket {
	return &DataPacket{
		Value: &DataPacket_StreamTrailer{
			StreamTrailer: p,
		},
	}
}
