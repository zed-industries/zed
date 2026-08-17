package rpc

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/livekit"
)

func TestNewCreateSIPParticipantRequest(t *testing.T) {
	r := &livekit.CreateSIPParticipantRequest{
		SipTrunkId:          "trunk",
		SipCallTo:           "+3333",
		RoomName:            "room",
		ParticipantIdentity: "",
		ParticipantName:     "",
		ParticipantMetadata: "meta",
		ParticipantAttributes: map[string]string{
			"extra": "1",
		},
		Headers: map[string]string{
			"X-B": "B2",
			"X-C": "C",
		},
		Dtmf:              "1234#",
		PlayDialtone:      true,
		WaitUntilAnswered: true,
	}
	tr := &livekit.SIPOutboundTrunkInfo{
		SipTrunkId:         "trunk",
		Address:            "sip.example.com",
		Numbers:            []string{"+1111"},
		DestinationCountry: "us",
		AuthUsername:       "user",
		AuthPassword:       "pass",
		Headers: map[string]string{
			"X-A": "A",
			"X-B": "B1",
		},
	}
	exp := &InternalCreateSIPParticipantRequest{
		ProjectId:           "p_123",
		SipCallId:           "call-id",
		SipTrunkId:          "trunk",
		Address:             "sip.example.com",
		Hostname:            "xyz.sip.livekit.cloud",
		DestinationCountry:  "us",
		Number:              "+1111",
		CallTo:              "+3333",
		Username:            "user",
		Password:            "pass",
		RoomName:            "room",
		ParticipantIdentity: "sip_+3333",
		ParticipantMetadata: "meta",
		Token:               "token",
		WsUrl:               "url",
		Dtmf:                "1234#",
		PlayDialtone:        true,
		ParticipantAttributes: map[string]string{
			"extra":                    "1",
			livekit.AttrSIPCallID:      "call-id",
			livekit.AttrSIPTrunkID:     "trunk",
			livekit.AttrSIPTrunkNumber: "+1111",
			livekit.AttrSIPPhoneNumber: "+3333",
			livekit.AttrSIPHostName:    "sip.example.com",
		},
		Headers: map[string]string{
			"X-A": "A",
			"X-B": "B2",
			"X-C": "C",
		},
		WaitUntilAnswered: true,
	}
	res, err := NewCreateSIPParticipantRequest("p_123", "call-id", "xyz.sip.livekit.cloud", "url", "token", r, tr)
	require.NoError(t, err)
	require.Equal(t, exp, res)

	r.HidePhoneNumber = true
	res, err = NewCreateSIPParticipantRequest("p_123", "call-id", "xyz.sip.livekit.cloud", "url", "token", r, tr)
	require.NoError(t, err)
	require.Equal(t, &InternalCreateSIPParticipantRequest{
		ProjectId:           "p_123",
		SipCallId:           "call-id",
		SipTrunkId:          "trunk",
		Address:             "sip.example.com",
		Hostname:            "xyz.sip.livekit.cloud",
		DestinationCountry:  "us",
		Number:              "+1111",
		CallTo:              "+3333",
		Username:            "user",
		Password:            "pass",
		RoomName:            "room",
		Token:               "token",
		WsUrl:               "url",
		Dtmf:                "1234#",
		PlayDialtone:        true,
		ParticipantIdentity: "sip_+3333",
		ParticipantAttributes: map[string]string{
			"extra":                "1",
			livekit.AttrSIPCallID:  "call-id",
			livekit.AttrSIPTrunkID: "trunk",
		},
		ParticipantMetadata: "meta",
		Headers: map[string]string{
			"X-A": "A",
			"X-B": "B2",
			"X-C": "C",
		},
		WaitUntilAnswered: true,
	}, res)

	r.HidePhoneNumber = false
	r.SipNumber = tr.Numbers[0]
	r.Trunk = &livekit.SIPOutboundConfig{
		Hostname:            tr.Address,
		Transport:           tr.Transport,
		DestinationCountry:  "us",
		AuthUsername:        tr.AuthUsername,
		AuthPassword:        tr.AuthPassword,
		HeadersToAttributes: tr.HeadersToAttributes,
		AttributesToHeaders: tr.AttributesToHeaders,
	}
	r.SipTrunkId = ""
	exp.SipTrunkId = ""
	for k, v := range tr.Headers {
		if _, ok := r.Headers[k]; !ok {
			r.Headers[k] = v
		}
	}
	exp.ParticipantAttributes[livekit.AttrSIPTrunkID] = ""
	res, err = NewCreateSIPParticipantRequest("p_123", "call-id", "xyz.sip.livekit.cloud", "url", "token", r, nil)
	require.NoError(t, err)
	require.Equal(t, exp, res)
}
