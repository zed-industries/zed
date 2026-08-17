package rpc

import (
	"errors"
	"maps"
	"math/rand/v2"
	"net"
	"strings"

	"github.com/livekit/protocol/livekit"
)

func (p *GetSIPTrunkAuthenticationRequest) SIPCall() *SIPCall {
	if p == nil {
		return nil
	}
	if p.Call != nil {
		return p.Call
	}
	ip := p.SrcAddress
	if addr, _, err := net.SplitHostPort(ip); err == nil {
		ip = addr
	}
	c := &SIPCall{
		LkCallId: p.SipCallId,
		SourceIp: ip,
		From: &livekit.SIPUri{
			User: p.From,
			Host: p.FromHost,
		},
		To: &livekit.SIPUri{
			User: p.To,
			Host: p.ToHost,
		},
	}
	c.Address = c.To
	return c
}

func (p *EvaluateSIPDispatchRulesRequest) SIPCall() *SIPCall {
	if p == nil {
		return nil
	}
	if p.Call != nil {
		return p.Call
	}
	ip := p.SrcAddress
	if addr, _, err := net.SplitHostPort(ip); err == nil {
		ip = addr
	}
	c := &SIPCall{
		LkCallId: p.SipCallId,
		SourceIp: ip,
		From: &livekit.SIPUri{
			User: p.CallingNumber,
			Host: p.CallingHost,
		},
		To: &livekit.SIPUri{
			User: p.CalledNumber,
			Host: p.CalledHost,
		},
	}
	c.Address = c.To
	return c
}

// NewCreateSIPParticipantRequest fills InternalCreateSIPParticipantRequest from
// livekit.CreateSIPParticipantRequest and livekit.SIPTrunkInfo.
func NewCreateSIPParticipantRequest(
	projectID, callID, ownHostname, wsUrl, token string,
	req *livekit.CreateSIPParticipantRequest,
	trunk *livekit.SIPOutboundTrunkInfo,
) (*InternalCreateSIPParticipantRequest, error) {
	if err := req.Validate(); err != nil {
		return nil, err
	}
	var (
		hostname           string
		enc                livekit.SIPMediaEncryption
		headers            map[string]string
		includeHeaders     livekit.SIPHeaderOptions
		transport          livekit.SIPTransport
		destinationCountry string
		authUser           string
		authPass           string
		hdrToAttr          map[string]string
		attrToHdr          map[string]string
	)
	if trunk != nil {
		hostname = trunk.Address
		enc = trunk.MediaEncryption
		headers = trunk.Headers
		includeHeaders = trunk.IncludeHeaders
		transport = trunk.Transport
		destinationCountry = trunk.DestinationCountry
		authUser = trunk.AuthUsername
		authPass = trunk.AuthPassword
		hdrToAttr = trunk.HeadersToAttributes
		attrToHdr = trunk.AttributesToHeaders
	} else if t := req.Trunk; t != nil {
		hostname = t.Hostname
		transport = t.Transport
		destinationCountry = t.DestinationCountry
		authUser = t.AuthUsername
		authPass = t.AuthPassword
		hdrToAttr = t.HeadersToAttributes
		attrToHdr = t.AttributesToHeaders
	}

	outboundNumber := req.SipNumber
	if outboundNumber == "" {
		if trunk == nil || len(trunk.Numbers) == 0 {
			return nil, errors.New("no numbers on outbound trunk")
		}
		outboundNumber = trunk.Numbers[rand.IntN(len(trunk.Numbers))]
	}
	// A sanity check for the number format for well-known providers.
	switch {
	case strings.HasSuffix(hostname, "twilio.com"):
		// Twilio requires leading '+'.
		if !strings.HasPrefix(outboundNumber, "+") {
			outboundNumber = "+" + outboundNumber
		}
	}
	attrs := maps.Clone(req.ParticipantAttributes)
	if attrs == nil {
		attrs = make(map[string]string)
	}
	attrs[livekit.AttrSIPCallID] = callID
	trunkID := req.SipTrunkId
	if trunkID == "" && trunk != nil {
		trunkID = trunk.SipTrunkId
	}
	attrs[livekit.AttrSIPTrunkID] = trunkID
	if !req.HidePhoneNumber {
		attrs[livekit.AttrSIPPhoneNumber] = req.SipCallTo
		attrs[livekit.AttrSIPHostName] = hostname
		attrs[livekit.AttrSIPTrunkNumber] = outboundNumber
	}

	var features []livekit.SIPFeature
	if req.KrispEnabled {
		features = append(features, livekit.SIPFeature_KRISP_ENABLED)
	}
	if req.MediaEncryption != 0 {
		enc = req.MediaEncryption
	}

	if len(req.Headers) != 0 {
		headers = maps.Clone(headers)
		if headers == nil {
			headers = make(map[string]string)
		}
		for k, v := range req.Headers {
			headers[k] = v
		}
	}
	if req.IncludeHeaders != 0 {
		includeHeaders = req.IncludeHeaders
	}
	participantIdentity := req.ParticipantIdentity
	if participantIdentity == "" {
		participantIdentity = "sip_" + req.SipCallTo
	}

	return &InternalCreateSIPParticipantRequest{
		ProjectId:             projectID,
		SipCallId:             callID,
		SipTrunkId:            trunkID,
		DestinationCountry:    destinationCountry,
		Address:               hostname,
		Hostname:              ownHostname,
		Transport:             transport,
		Number:                outboundNumber,
		Username:              authUser,
		Password:              authPass,
		CallTo:                req.SipCallTo,
		WsUrl:                 wsUrl,
		Token:                 token,
		RoomName:              req.RoomName,
		ParticipantIdentity:   participantIdentity,
		ParticipantName:       req.ParticipantName,
		ParticipantMetadata:   req.ParticipantMetadata,
		ParticipantAttributes: attrs,
		Dtmf:                  req.Dtmf,
		PlayDialtone:          req.PlayRingtone || req.PlayDialtone,
		Headers:               headers,
		HeadersToAttributes:   hdrToAttr,
		AttributesToHeaders:   attrToHdr,
		IncludeHeaders:        includeHeaders,
		EnabledFeatures:       features,
		RingingTimeout:        req.RingingTimeout,
		MaxCallDuration:       req.MaxCallDuration,
		MediaEncryption:       enc,
		WaitUntilAnswered:     req.WaitUntilAnswered,
		DisplayName:           req.DisplayName,
		Destination:           req.Destination,
	}, nil
}

// NewTransferSIPParticipantRequest fills InternalTransferSIPParticipantRequest from
// livekit.TransferSIPParticipantRequest.
func NewTransferSIPParticipantRequest(
	callID string,
	req *livekit.TransferSIPParticipantRequest,
) (*InternalTransferSIPParticipantRequest, error) {
	return &InternalTransferSIPParticipantRequest{
		SipCallId:      callID,
		TransferTo:     req.TransferTo,
		PlayDialtone:   req.PlayDialtone,
		Headers:        req.Headers,
		RingingTimeout: req.RingingTimeout,
	}, nil
}
