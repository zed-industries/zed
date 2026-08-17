package livekit

import (
	"errors"
	"fmt"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"github.com/livekit/protocol/utils/xtwirp"
	"golang.org/x/text/language"
)

var (
	_ xtwirp.ErrorMeta = (*SIPStatus)(nil)
	_ error            = (*SIPStatus)(nil)
)

// SIPStatusFrom unwraps an error and returns associated SIP call status, if any.
func SIPStatusFrom(err error) *SIPStatus {
	st, ok := status.FromError(err)
	if !ok {
		return nil
	}
	for _, d := range st.Details() {
		if e, ok := d.(*SIPStatus); ok {
			return e
		}
	}
	return nil
}

func (p SIPStatusCode) ShortName() string {
	return strings.TrimPrefix(p.String(), "SIP_STATUS_")
}

func (p *SIPStatus) Error() string {
	if p.Status != "" {
		return fmt.Sprintf("sip status: %d: %s", p.Code, p.Status)
	}
	return fmt.Sprintf("sip status: %d (%s)", p.Code, p.Code.ShortName())
}

// Maps SIP response codes received from remote SIP servers to GRPC error codes.
var sipCodeToGRPCCode = map[SIPStatusCode]codes.Code{
	// 3xx - Redirection Responses
	SIPStatusCode_SIP_STATUS_MULTIPLE_CHOICES:    codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_MOVED_PERMANENTLY:   codes.NotFound,
	SIPStatusCode_SIP_STATUS_MOVED_TEMPORARILY:   codes.NotFound,
	SIPStatusCode_SIP_STATUS_USE_PROXY:           codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_ALTERNATIVE_SERVICE: codes.InvalidArgument,

	// 4xx - Client Failure Responses
	SIPStatusCode_SIP_STATUS_BAD_REQUEST:                      codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_UNAUTHORIZED:                     codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_PAYMENT_REQUIRED:                 codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_FORBIDDEN:                        codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_NOTFOUND:                         codes.NotFound,
	SIPStatusCode_SIP_STATUS_METHOD_NOT_ALLOWED:               codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_NOT_ACCEPTABLE:                   codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_PROXY_AUTH_REQUIRED:              codes.Unauthenticated,
	SIPStatusCode_SIP_STATUS_REQUEST_TIMEOUT:                  codes.DeadlineExceeded,
	SIPStatusCode_SIP_STATUS_CONFLICT:                         codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_GONE:                             codes.NotFound,
	SIPStatusCode_SIP_STATUS_LENGTH_REQUIRED:                  codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_CONDITIONAL_REQUEST_FAILED:       codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_REQUEST_ENTITY_TOO_LARGE:         codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_REQUEST_URI_TOO_LONG:             codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_UNSUPPORTED_MEDIA_TYPE:           codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_REQUESTED_RANGE_NOT_SATISFIABLE:  codes.OutOfRange,
	SIPStatusCode_SIP_STATUS_UNKNOWN_RESOURCE_PRIORITY:        codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_BAD_EXTENSION:                    codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_EXTENSION_REQUIRED:               codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_SESSION_INTERVAL_TOO_SMALL:       codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_INTERVAL_TOO_BRIEF:               codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_BAD_LOCATION_INFORMATION:         codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_BAD_ALERT_MESSAGE:                codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_USE_IDENTITY_HEADER:              codes.Unauthenticated,
	SIPStatusCode_SIP_STATUS_PROVIDE_REFERRER_IDENTITY:        codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_FLOW_FAILED:                      codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_ANONYMITY_DISALLOWED:             codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_BAD_IDENTITY_INFO:                codes.Unauthenticated,
	SIPStatusCode_SIP_STATUS_UNSUPPORTED_CERTIFICATE:          codes.Unauthenticated,
	SIPStatusCode_SIP_STATUS_INVALID_IDENTITY_HEADER:          codes.Unauthenticated,
	SIPStatusCode_SIP_STATUS_FIRST_HOP_LACKS_OUTBOUND_SUPPORT: codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_MAX_BREADTH_EXCEEDED:             codes.ResourceExhausted,
	SIPStatusCode_SIP_STATUS_BAD_INFO_PACKAGE:                 codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_CONSENT_NEEDED:                   codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_TEMPORARILY_UNAVAILABLE:          codes.ResourceExhausted,
	SIPStatusCode_SIP_STATUS_CALL_TRANSACTION_DOES_NOT_EXISTS: codes.NotFound,
	SIPStatusCode_SIP_STATUS_LOOP_DETECTED:                    codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_TOO_MANY_HOPS:                    codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_ADDRESS_INCOMPLETE:               codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_AMBIGUOUS:                        codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_BUSY_HERE:                        codes.ResourceExhausted,
	SIPStatusCode_SIP_STATUS_REQUEST_TERMINATED:               codes.Aborted,
	SIPStatusCode_SIP_STATUS_NOT_ACCEPTABLE_HERE:              codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_BAD_EVENT:                        codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_REQUEST_PENDING:                  codes.Aborted,
	SIPStatusCode_SIP_STATUS_UNDECIPHERABLE:                   codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_SECURITY_AGREEMENT_REQUIRED:      codes.Unauthenticated,

	// 5xx - Server Failure Responses
	SIPStatusCode_SIP_STATUS_INTERNAL_SERVER_ERROR: codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_NOT_IMPLEMENTED:       codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_BAD_GATEWAY:           codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_SERVICE_UNAVAILABLE:   codes.FailedPrecondition,
	SIPStatusCode_SIP_STATUS_GATEWAY_TIMEOUT:       codes.DeadlineExceeded,
	SIPStatusCode_SIP_STATUS_VERSION_NOT_SUPPORTED: codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_MESSAGE_TOO_LARGE:     codes.InvalidArgument,

	// 6xx - Global Failure Responses
	SIPStatusCode_SIP_STATUS_GLOBAL_BUSY_EVERYWHERE:         codes.ResourceExhausted,
	SIPStatusCode_SIP_STATUS_GLOBAL_DECLINE:                 codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: codes.NotFound,
	SIPStatusCode_SIP_STATUS_GLOBAL_NOT_ACCEPTABLE:          codes.InvalidArgument,
	SIPStatusCode_SIP_STATUS_GLOBAL_UNWANTED:                codes.PermissionDenied,
	SIPStatusCode_SIP_STATUS_GLOBAL_REJECTED:                codes.PermissionDenied,
}

func (p *SIPStatus) GRPCStatus() *status.Status {
	code, ok := sipCodeToGRPCCode[p.Code]
	if !ok {
		code = codes.Unknown // 1xx and 2xx codes should never emit an error, something is wrong.
		if p.Code < 200 {
			code = codes.Unknown // 1xx are not final responses, something is wrong.
		} else if p.Code < 300 {
			return status.New(codes.OK, "OK") // Preserving previous behavior
		} else if code < 500 {
			code = codes.InvalidArgument
		} else if code < 600 {
			code = codes.FailedPrecondition // 5xx from remote server, per guideline (c) in gRPC docs
		} else if code < 700 {
			code = codes.InvalidArgument // Same as 4xx ,but authoritative
		}
	}
	msg := p.Status
	if msg == "" {
		msg = p.Code.ShortName()
	}
	st := status.New(code, fmt.Sprintf("sip status %d: %s", p.Code, msg))
	if st2, err := st.WithDetails(p); err == nil {
		return st2
	}
	return st
}

func (p *SIPStatus) TwirpErrorMeta() map[string]string {
	status := p.Status
	if status == "" {
		status = p.Code.String()
	}
	return map[string]string{
		"sip_status_code": strconv.Itoa(int(p.Code)),
		"sip_status":      status,
	}
}

// Name returns a lower-case short name for the transport.
// It returns an empty string if transport is not specified.
func (p SIPTransport) Name() string {
	switch p {
	case SIPTransport_SIP_TRANSPORT_AUTO:
		return ""
	case SIPTransport_SIP_TRANSPORT_UDP:
		return "udp"
	case SIPTransport_SIP_TRANSPORT_TCP:
		return "tcp"
	case SIPTransport_SIP_TRANSPORT_TLS:
		return "tls"
	default:
		return strings.TrimPrefix(p.String(), "SIP_TRANSPORT_")
	}
}

// ToProto implements DataPacket in Go SDK.
func (p *SipDTMF) ToProto() *DataPacket {
	return &DataPacket{
		Value: &DataPacket_SipDtmf{
			SipDtmf: p,
		},
	}
}

func (p *SIPTrunkInfo) ID() string {
	if p == nil {
		return ""
	}
	return p.SipTrunkId
}

func (p *SIPInboundTrunkInfo) ID() string {
	if p == nil {
		return ""
	}
	return p.SipTrunkId
}

func (p *SIPOutboundTrunkInfo) ID() string {
	if p == nil {
		return ""
	}
	return p.SipTrunkId
}

func (p *SIPDispatchRuleInfo) ID() string {
	if p == nil {
		return ""
	}
	return p.SipDispatchRuleId
}

// AsInbound converts legacy SIPTrunkInfo to SIPInboundTrunkInfo.
func (p *SIPTrunkInfo) AsInbound() *SIPInboundTrunkInfo {
	if p == nil || p.Kind == SIPTrunkInfo_TRUNK_OUTBOUND {
		return nil
	}
	var nums []string
	if p.OutboundNumber != "" {
		nums = []string{p.OutboundNumber}
	}
	return &SIPInboundTrunkInfo{
		SipTrunkId:       p.SipTrunkId,
		Name:             p.Name,
		Metadata:         p.Metadata,
		Numbers:          nums,
		AllowedAddresses: p.InboundAddresses,
		AllowedNumbers:   p.InboundNumbers,
		AuthUsername:     p.InboundUsername,
		AuthPassword:     p.InboundPassword,
	}
}

// AsTrunkInfo converts SIPInboundTrunkInfo to legacy SIPTrunkInfo.
func (p *SIPInboundTrunkInfo) AsTrunkInfo() *SIPTrunkInfo {
	if p == nil {
		return nil
	}
	var num string
	if len(p.Numbers) != 0 {
		num = p.Numbers[0]
	}
	return &SIPTrunkInfo{
		SipTrunkId:       p.SipTrunkId,
		Kind:             SIPTrunkInfo_TRUNK_INBOUND,
		Name:             p.Name,
		Metadata:         p.Metadata,
		OutboundNumber:   num,
		InboundAddresses: p.AllowedAddresses,
		InboundNumbers:   p.AllowedNumbers,
		InboundUsername:  p.AuthUsername,
		InboundPassword:  p.AuthPassword,
	}
}

// AsOutbound converts legacy SIPTrunkInfo to SIPOutboundTrunkInfo.
func (p *SIPTrunkInfo) AsOutbound() *SIPOutboundTrunkInfo {
	if p == nil || p.Kind == SIPTrunkInfo_TRUNK_INBOUND {
		return nil
	}
	var nums []string
	if p.OutboundNumber != "" {
		nums = []string{p.OutboundNumber}
	}
	return &SIPOutboundTrunkInfo{
		SipTrunkId:   p.SipTrunkId,
		Name:         p.Name,
		Metadata:     p.Metadata,
		Address:      p.OutboundAddress,
		Transport:    p.Transport,
		Numbers:      nums,
		AuthUsername: p.OutboundUsername,
		AuthPassword: p.OutboundPassword,
	}
}

// AsTrunkInfo converts SIPOutboundTrunkInfo to legacy SIPTrunkInfo.
func (p *SIPOutboundTrunkInfo) AsTrunkInfo() *SIPTrunkInfo {
	if p == nil {
		return nil
	}
	var num string
	if len(p.Numbers) != 0 {
		num = p.Numbers[0]
	}
	return &SIPTrunkInfo{
		SipTrunkId:       p.SipTrunkId,
		Kind:             SIPTrunkInfo_TRUNK_OUTBOUND,
		Name:             p.Name,
		Metadata:         p.Metadata,
		OutboundAddress:  p.Address,
		Transport:        p.Transport,
		OutboundNumber:   num,
		OutboundUsername: p.AuthUsername,
		OutboundPassword: p.AuthPassword,
	}
}

// validateHeaders makes sure header names/keys and values are per SIP specifications
func validateHeaders(headers map[string]string) error {
	for headerName, headerValue := range headers {
		if err := ValidateHeaderName(headerName, true); err != nil {
			return fmt.Errorf("invalid header name: %w", err)
		}
		if err := ValidateHeaderValue(headerName, headerValue); err != nil {
			return fmt.Errorf("invalid header value for %s: %w", headerName, err)
		}
	}
	return nil
}

// validateHeaderNames Makes sure the values of the given map correspond to valid SIP header names
func validateAttributesToHeaders(attributesToHeaders map[string]string) error {
	for _, headerName := range attributesToHeaders {
		if err := ValidateHeaderName(headerName, false); err != nil {
			return fmt.Errorf("invalid header name: %w", err)
		}
	}
	return nil
}

// validateHeaderToAttributes Makes sure the keys of the given map correspond to valid SIP header names
func validateHeaderToAttributes(headerToAttributes map[string]string) error {
	for headerName, _ := range headerToAttributes {
		if err := ValidateHeaderName(headerName, false); err != nil {
			return fmt.Errorf("invalid header name: %w", err)
		}
	}
	return nil
}

func (p *SIPTrunkInfo) Validate() error {
	if len(p.InboundNumbersRegex) != 0 {
		return fmt.Errorf("trunks with InboundNumbersRegex are deprecated")
	}
	return nil
}

func (p *CreateSIPOutboundTrunkRequest) Validate() error {
	if p.Trunk == nil {
		return errors.New("missing trunk")
	}
	if p.Trunk.SipTrunkId != "" {
		return errors.New("trunk id must not be set")
	}
	if err := p.Trunk.Validate(); err != nil {
		return err
	}
	return nil
}

func (p *CreateSIPInboundTrunkRequest) Validate() error {
	if p.Trunk == nil {
		return errors.New("missing trunk")
	}
	if p.Trunk.SipTrunkId != "" {
		return errors.New("trunk id must not be set")
	}
	if err := p.Trunk.Validate(); err != nil {
		return err
	}
	return nil
}

func (p *UpdateSIPOutboundTrunkRequest) Validate() error {
	if p.SipTrunkId == "" {
		return errors.New("trunk id must be set")
	}
	if p.Action == nil {
		return errors.New("missing or unsupported update action")
	}
	switch a := p.Action.(type) {
	default:
		return nil
	case *UpdateSIPOutboundTrunkRequest_Replace:
		info := a.Replace
		if info == nil {
			return errors.New("missing trunk")
		}
		if info.SipTrunkId != "" && info.SipTrunkId != p.SipTrunkId {
			return errors.New("trunk id in the info must be empty or match the id in the update")
		}
		return info.Validate()
	case *UpdateSIPOutboundTrunkRequest_Update:
		diff := a.Update
		if diff == nil {
			return errors.New("missing trunk update")
		}
		return diff.Validate()
	}
}

func (p *UpdateSIPInboundTrunkRequest) Validate() error {
	if p.SipTrunkId == "" {
		return errors.New("trunk id must be set")
	}
	if p.Action == nil {
		return errors.New("missing or unsupported update action")
	}
	switch a := p.Action.(type) {
	default:
		return nil
	case *UpdateSIPInboundTrunkRequest_Replace:
		info := a.Replace
		if info == nil {
			return errors.New("missing trunk")
		}
		if info.SipTrunkId != "" && info.SipTrunkId != p.SipTrunkId {
			return errors.New("trunk id in the info must be empty or match the id in the update")
		}
		return info.Validate()
	case *UpdateSIPInboundTrunkRequest_Update:
		diff := a.Update
		if diff == nil {
			return errors.New("missing trunk update")
		}
		return diff.Validate()
	}
}

func (p *SIPInboundTrunkInfo) Validate() error {
	hasAuth := p.AuthUsername != "" || p.AuthPassword != ""
	hasCIDR := len(p.AllowedAddresses) != 0
	hasNumbers := len(p.Numbers) != 0 // TODO: remove this condition, it doesn't really help with security
	if !hasAuth && !hasCIDR && !hasNumbers {
		return errors.New("for security, one of the fields must be set: AuthUsername+AuthPassword, AllowedAddresses or Numbers")
	}
	if err := validateHeaders(p.Headers); err != nil {
		return err
	}
	if err := validateAttributesToHeaders(p.AttributesToHeaders); err != nil {
		return err
	}
	if err := validateHeaderToAttributes(p.HeadersToAttributes); err != nil {
		return err
	}
	return nil
}

func (p *SIPInboundTrunkUpdate) Validate() error {
	if err := p.Numbers.Validate(); err != nil {
		return err
	}
	if err := p.AllowedAddresses.Validate(); err != nil {
		return err
	}
	if err := p.AllowedNumbers.Validate(); err != nil {
		return err
	}
	return nil
}

func (p *SIPInboundTrunkUpdate) Apply(info *SIPInboundTrunkInfo) error {
	if err := p.Validate(); err != nil {
		return err
	}
	applyListUpdate(&info.Numbers, p.Numbers)
	applyListUpdate(&info.AllowedAddresses, p.AllowedAddresses)
	applyListUpdate(&info.AllowedNumbers, p.AllowedNumbers)
	applyUpdate(&info.AuthUsername, p.AuthUsername)
	applyUpdate(&info.AuthPassword, p.AuthPassword)
	applyUpdate(&info.Name, p.Name)
	applyUpdate(&info.Metadata, p.Metadata)
	applyUpdate(&info.MediaEncryption, p.MediaEncryption)
	return info.Validate()
}

type UpdateSIPOutboundTrunkRequestAction interface {
	isUpdateSIPOutboundTrunkRequest_Action
	Apply(info *SIPOutboundTrunkInfo) (*SIPOutboundTrunkInfo, error)
}

var (
	_ UpdateSIPOutboundTrunkRequestAction = (*UpdateSIPOutboundTrunkRequest_Replace)(nil)
	_ UpdateSIPOutboundTrunkRequestAction = (*UpdateSIPOutboundTrunkRequest_Update)(nil)
)

func (p *UpdateSIPOutboundTrunkRequest_Replace) Apply(info *SIPOutboundTrunkInfo) (*SIPOutboundTrunkInfo, error) {
	val := cloneProto(p.Replace)
	if val == nil {
		return nil, errors.New("missing trunk")
	}
	if info.SipTrunkId != "" {
		val.SipTrunkId = info.SipTrunkId
	}
	if err := val.Validate(); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *UpdateSIPOutboundTrunkRequest_Update) Apply(info *SIPOutboundTrunkInfo) (*SIPOutboundTrunkInfo, error) {
	diff := p.Update
	if diff == nil {
		return nil, errors.New("missing trunk update")
	}
	val := cloneProto(info)
	if err := diff.Apply(val); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *SIPOutboundTrunkInfo) Validate() error {
	if len(p.Numbers) == 0 {
		return errors.New("no trunk numbers specified")
	}
	if p.Address == "" {
		return errors.New("no outbound address specified")
	} else if strings.Contains(p.Address, "transport=") {
		return errors.New("trunk transport should be set as a field, not a URI parameter")
	} else if strings.ContainsAny(p.Address, "@;") || strings.HasPrefix(p.Address, "sip:") || strings.HasPrefix(p.Address, "sips:") {
		return errors.New("trunk address should be a hostname or IP, not SIP URI")
	}
	if err := validateHeaders(p.Headers); err != nil {
		return err
	}
	if err := validateAttributesToHeaders(p.AttributesToHeaders); err != nil {
		return err
	}
	if err := validateHeaderToAttributes(p.HeadersToAttributes); err != nil {
		return err
	}
	return nil
}

func (p *SIPOutboundConfig) Validate() error {
	if p.Hostname == "" {
		return errors.New("no outbound hostname specified")
	} else if strings.Contains(p.Hostname, "transport=") {
		return errors.New("trunk transport should be set as a field, not a URI parameter")
	} else if strings.ContainsAny(p.Hostname, "@;") || strings.HasPrefix(p.Hostname, "sip:") || strings.HasPrefix(p.Hostname, "sips:") {
		return errors.New("trunk hostname should be a domain name or IP, not SIP URI")
	}
	if err := validateAttributesToHeaders(p.AttributesToHeaders); err != nil {
		return err
	}
	if err := validateHeaderToAttributes(p.HeadersToAttributes); err != nil {
		return err
	}
	return nil
}

func (p *SIPOutboundTrunkUpdate) Validate() error {
	if err := p.Numbers.Validate(); err != nil {
		return err
	}
	return nil
}

func (p *SIPOutboundTrunkUpdate) Apply(info *SIPOutboundTrunkInfo) error {
	if err := p.Validate(); err != nil {
		return err
	}
	applyUpdate(&info.Address, p.Address)
	applyUpdate(&info.Transport, p.Transport)
	applyUpdate(&info.DestinationCountry, p.DestinationCountry)
	applyListUpdate(&info.Numbers, p.Numbers)
	applyUpdate(&info.AuthUsername, p.AuthUsername)
	applyUpdate(&info.AuthPassword, p.AuthPassword)
	applyUpdate(&info.Name, p.Name)
	applyUpdate(&info.Metadata, p.Metadata)
	applyUpdate(&info.MediaEncryption, p.MediaEncryption)
	return info.Validate()
}

type UpdateSIPInboundTrunkRequestAction interface {
	isUpdateSIPInboundTrunkRequest_Action
	Apply(info *SIPInboundTrunkInfo) (*SIPInboundTrunkInfo, error)
}

var (
	_ UpdateSIPInboundTrunkRequestAction = (*UpdateSIPInboundTrunkRequest_Replace)(nil)
	_ UpdateSIPInboundTrunkRequestAction = (*UpdateSIPInboundTrunkRequest_Update)(nil)
)

func (p *UpdateSIPInboundTrunkRequest_Replace) Apply(info *SIPInboundTrunkInfo) (*SIPInboundTrunkInfo, error) {
	val := cloneProto(p.Replace)
	if val == nil {
		return nil, errors.New("missing trunk")
	}
	if info.SipTrunkId != "" {
		val.SipTrunkId = info.SipTrunkId
	}
	if err := val.Validate(); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *UpdateSIPInboundTrunkRequest_Update) Apply(info *SIPInboundTrunkInfo) (*SIPInboundTrunkInfo, error) {
	diff := p.Update
	if diff == nil {
		return nil, errors.New("missing trunk update")
	}
	val := cloneProto(info)
	if err := diff.Apply(val); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *CreateSIPDispatchRuleRequest) DispatchRuleInfo() *SIPDispatchRuleInfo {
	if p == nil {
		return nil
	}
	if p.DispatchRule != nil {
		return p.DispatchRule
	}
	return &SIPDispatchRuleInfo{
		Rule:            p.Rule,
		TrunkIds:        p.TrunkIds,
		InboundNumbers:  p.InboundNumbers,
		HidePhoneNumber: p.HidePhoneNumber,
		Name:            p.Name,
		Metadata:        p.Metadata,
		Attributes:      p.Attributes,
		RoomPreset:      p.RoomPreset,
		RoomConfig:      p.RoomConfig,
	}
}

func (p *CreateSIPDispatchRuleRequest) Validate() error {
	if p.DispatchRule == nil {
		// legacy
		return p.DispatchRuleInfo().Validate()
	}
	if p.DispatchRule.SipDispatchRuleId != "" {
		return errors.New("rule id must not be set")
	}
	return p.DispatchRule.Validate()
}

func (p *UpdateSIPDispatchRuleRequest) Validate() error {
	if p.SipDispatchRuleId == "" {
		return errors.New("rule id must be set")
	}
	if p.Action == nil {
		return errors.New("missing or unsupported update action")
	}
	switch a := p.Action.(type) {
	default:
		return nil
	case *UpdateSIPDispatchRuleRequest_Replace:
		info := a.Replace
		if info == nil {
			return errors.New("missing dispatch rule")
		}
		if info.SipDispatchRuleId != "" && info.SipDispatchRuleId != p.SipDispatchRuleId {
			return errors.New("rule id in the info must be empty or match the id in the update")
		}
		return info.Validate()
	case *UpdateSIPDispatchRuleRequest_Update:
		diff := a.Update
		if diff == nil {
			return errors.New("missing dispatch rule update")
		}
		return diff.Validate()
	}
}

func (p *SIPDispatchRuleInfo) Validate() error {
	if p.Rule == nil {
		return errors.New("missing rule")
	}
	return nil
}

func (p *SIPDispatchRuleUpdate) Validate() error {
	if err := p.TrunkIds.Validate(); err != nil {
		return err
	}
	return nil
}

func (p *SIPDispatchRuleUpdate) Apply(info *SIPDispatchRuleInfo) error {
	if err := p.Validate(); err != nil {
		return err
	}
	applyListUpdate(&info.TrunkIds, p.TrunkIds)
	applyUpdatePtr(&info.Rule, p.Rule)
	applyUpdate(&info.Name, p.Name)
	applyUpdate(&info.Metadata, p.Metadata)
	applyUpdate(&info.MediaEncryption, p.MediaEncryption)
	applyMapDiff(&info.Attributes, p.Attributes)
	return info.Validate()
}

type UpdateSIPDispatchRuleRequestAction interface {
	isUpdateSIPDispatchRuleRequest_Action
	Apply(info *SIPDispatchRuleInfo) (*SIPDispatchRuleInfo, error)
}

var (
	_ UpdateSIPDispatchRuleRequestAction = (*UpdateSIPDispatchRuleRequest_Replace)(nil)
	_ UpdateSIPDispatchRuleRequestAction = (*UpdateSIPDispatchRuleRequest_Update)(nil)
)

func (p *UpdateSIPDispatchRuleRequest_Replace) Apply(info *SIPDispatchRuleInfo) (*SIPDispatchRuleInfo, error) {
	val := cloneProto(p.Replace)
	if val == nil {
		return nil, errors.New("missing dispatch rule")
	}
	if info.SipDispatchRuleId != "" {
		val.SipDispatchRuleId = info.SipDispatchRuleId
	}
	if err := val.Validate(); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *UpdateSIPDispatchRuleRequest_Update) Apply(info *SIPDispatchRuleInfo) (*SIPDispatchRuleInfo, error) {
	diff := p.Update
	if diff == nil {
		return nil, errors.New("missing dispatch rule update")
	}
	val := cloneProto(info)
	if err := diff.Apply(val); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *CreateSIPParticipantRequest) Validate() error {
	if p.SipTrunkId == "" && p.Trunk == nil {
		return errors.New("missing sip trunk id")
	}
	if p.Trunk != nil {
		if err := p.Trunk.Validate(); err != nil {
			return err
		}
	}
	if p.SipCallTo == "" {
		return errors.New("missing sip callee number")
	} else if strings.Contains(p.SipCallTo, "@") {
		return errors.New("SipCallTo should be a phone number or SIP user, not a full SIP URI")
	}
	if p.RoomName == "" {
		return errors.New("missing room name")
	}

	// Validate display_name if provided
	if p.DisplayName != nil {
		if len(*p.DisplayName) > 128 {
			return errors.New("DisplayName too long (max 128 characters)")
		}
		if _, err := strconv.Unquote("\"" + *p.DisplayName + "\""); err != nil {
			return fmt.Errorf("DisplayName must be a valid quoted string: %w", err)
		}
	}

	// Validate destination if provided
	if err := p.Destination.Validate(); err != nil {
		return err
	}

	if err := validateHeaders(p.Headers); err != nil {
		return err
	}

	return nil
}

func (d *Destination) Validate() error {
	if d == nil {
		return nil
	}

	// Rule 1: If city is specified, country must be specified
	if d.City != "" && d.Country == "" && d.Region == "" {
		return errors.New("if city is specified, country or region must also be specified")
	}

	// Rule 2: If country is specified, it must be a valid ISO 3166-1 alpha-2 code (2-letter only)
	if d.Country != "" {
		// First check: must be exactly 2 characters
		if len(d.Country) != 2 {
			return errors.New("country must be a valid ISO 3166-1 alpha-2 code (2-letter like 'US', 'IN', 'UK')")
		}

		// Use golang.org/x/text/language to validate 2-letter country codes
		region, err := language.ParseRegion(d.Country)
		if err != nil {
			return errors.New("country must be a valid ISO 3166-1 alpha-2 code (2-letter like 'US', 'IN', 'UK')")
		}

		// Check if the parsed region is actually a valid country
		// This is the most direct way to validate - region.IsCountry() returns true
		// only for actual valid countries, false for invalid codes like "XX"
		if !region.IsCountry() {
			return errors.New("country must be a valid ISO 3166-1 alpha-2 code (2-letter like 'US', 'IN', 'UK')")
		}

		// Additional check: ensure the parsed region matches our input
		// This prevents auto corrections by the library
		if region.String() != d.Country {
			return errors.New("country must be a valid ISO 3166-1 alpha-2 code (2-letter like 'US', 'IN', 'UK')")
		}
	}

	return nil
}

func (p *TransferSIPParticipantRequest) Validate() error {
	if p.RoomName == "" {
		return errors.New("missing room name")
	}
	if p.ParticipantIdentity == "" {
		return errors.New("missing participant identity")
	}
	if p.TransferTo == "" {
		return errors.New("missing transfer to")
	}

	// Validate TransferTo URI format and ensure RFC compliance
	var innerURI string
	if strings.HasPrefix(p.TransferTo, "<") && strings.HasSuffix(p.TransferTo, ">") {
		// Extract inner URI for validation
		innerURI = p.TransferTo[1 : len(p.TransferTo)-1]
	} else {
		innerURI = p.TransferTo
	}

	if !strings.HasPrefix(innerURI, "sip:") && !strings.HasPrefix(innerURI, "tel:") {
		// In theory the Refer-To header can receive the full name-addr.
		// This can make this check inaccurate, but we want to limit to just SIP and TEL URIs.
		return errors.New("transfer_to must be a valid SIP or TEL URI (sip: or tel:)")
	}

	if strings.HasPrefix(innerURI, "sip:") {
		// addr-spec = sip:...
		// name-addr = [ display-name ] <addr-spec>
		// Both name-addr and addr-spec are allowed in RFC3515 (section-2.1).
		// However, name-addr is more premissive and widely-supported, so we convert.
		p.TransferTo = fmt.Sprintf("<%s>", innerURI)
	} else {
		// tel: URIs are not explicitly allowed in spec, but are generally supported.
		p.TransferTo = innerURI
	}

	if err := validateHeaders(p.Headers); err != nil {
		return err
	}

	return nil
}

func filterSlice[T any](arr []T, fnc func(v T) bool) []T {
	var out []T
	for _, v := range arr {
		if fnc(v) {
			out = append(out, v)
		}
	}
	return out
}

func filterIDs[T any, ID comparable](arr []T, ids []ID, get func(v T) ID) []T {
	if len(ids) == 0 {
		return arr
	}
	out := make([]T, len(ids))
	for i, id := range ids {
		j := slices.IndexFunc(arr, func(v T) bool {
			return get(v) == id
		})
		if j >= 0 {
			out[i] = arr[j]
		}
	}
	return out
}

func (p *ListSIPTrunkRequest) Filter(info *SIPTrunkInfo) bool {
	if info == nil {
		return true // for FilterSlice to work correctly with missing IDs
	}
	return true
}

func (p *ListSIPInboundTrunkRequest) Filter(info *SIPInboundTrunkInfo) bool {
	if info == nil {
		return true // for FilterSlice to work correctly with missing IDs
	}
	if len(p.TrunkIds) != 0 && !slices.Contains(p.TrunkIds, info.SipTrunkId) {
		return false
	}
	if len(p.Numbers) != 0 && len(info.Numbers) != 0 {
		ok := false
		for _, num := range info.Numbers {
			if slices.Contains(p.Numbers, num) {
				ok = true
				break
			}
			normalizedNum := NormalizeNumber(num)
			for _, reqNum := range p.Numbers {
				if NormalizeNumber(reqNum) == normalizedNum {
					ok = true
					break
				}
			}
			if ok {
				break
			}
		}
		if !ok {
			return false
		}
	}
	return true
}

func (p *ListSIPInboundTrunkRequest) FilterSlice(arr []*SIPInboundTrunkInfo) []*SIPInboundTrunkInfo {
	arr = filterIDs(arr, p.TrunkIds, func(v *SIPInboundTrunkInfo) string {
		return v.SipTrunkId
	})
	return filterSlice(arr, p.Filter)
}

func (p *ListSIPOutboundTrunkRequest) Filter(info *SIPOutboundTrunkInfo) bool {
	if info == nil {
		return true // for FilterSlice to work correctly with missing IDs
	}
	if len(p.TrunkIds) != 0 && !slices.Contains(p.TrunkIds, info.SipTrunkId) {
		return false
	}
	if len(p.Numbers) != 0 && len(info.Numbers) != 0 {
		ok := false
		for _, num := range info.Numbers {
			if slices.Contains(p.Numbers, num) {
				ok = true
				break
			}
		}
		if !ok {
			return false
		}
	}
	return true
}

func (p *ListSIPOutboundTrunkRequest) FilterSlice(arr []*SIPOutboundTrunkInfo) []*SIPOutboundTrunkInfo {
	arr = filterIDs(arr, p.TrunkIds, func(v *SIPOutboundTrunkInfo) string {
		return v.SipTrunkId
	})
	return filterSlice(arr, p.Filter)
}

func (p *ListSIPDispatchRuleRequest) Filter(info *SIPDispatchRuleInfo) bool {
	if info == nil {
		return true // for FilterSlice to work correctly with missing IDs
	}
	if len(p.DispatchRuleIds) != 0 && !slices.Contains(p.DispatchRuleIds, info.SipDispatchRuleId) {
		return false
	}
	if len(p.TrunkIds) != 0 && len(info.TrunkIds) != 0 {
		ok := false
		for _, id := range info.TrunkIds {
			if slices.Contains(p.TrunkIds, id) {
				ok = true
				break
			}
		}
		if !ok {
			return false
		}
	}
	return true
}

func (p *ListSIPDispatchRuleRequest) FilterSlice(arr []*SIPDispatchRuleInfo) []*SIPDispatchRuleInfo {
	arr = filterIDs(arr, p.DispatchRuleIds, func(v *SIPDispatchRuleInfo) string {
		return v.SipDispatchRuleId
	})
	return filterSlice(arr, p.Filter)
}

// NormalizeNumber normalizes a phone number by removing formatting characters and ensuring it starts with a "+".
// If the input is empty, it returns an empty string.
// If the input doesn't match the expected number pattern, it returns the original input unchanged.
func NormalizeNumber(num string) string {
	if num == "" {
		return ""
	}
	if !reNumber.MatchString(num) {
		return num
	}
	num = reNumberRepl.Replace(num)
	if !strings.HasPrefix(num, "+") {
		return "+" + num
	}
	return num
}

var (
	reNumber     = regexp.MustCompile(`^\+?[\d\- ()]+$`)
	reNumberRepl = strings.NewReplacer(
		" ", "",
		"-", "",
		"(", "",
		")", "",
	)
)
