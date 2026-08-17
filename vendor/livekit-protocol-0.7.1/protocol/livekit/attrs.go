package livekit

// Names of participant attributes for SIP.
const (
	// AttrSIPPrefix is shared for all SIP attributes.
	AttrSIPPrefix = "sip."
	// AttrSIPCallID attribute contains LiveKit SIP call ID.
	AttrSIPCallID = AttrSIPPrefix + "callID"
	// AttrSIPTrunkID attribute contains LiveKit SIP Trunk ID used for the call.
	AttrSIPTrunkID = AttrSIPPrefix + "trunkID"
	// AttrSIPDispatchRuleID attribute contains LiveKit SIP DispatchRule ID used for the inbound call.
	AttrSIPDispatchRuleID = AttrSIPPrefix + "ruleID"
	// AttrSIPTrunkNumber attribute contains number associate with LiveKit SIP Trunk.
	// This attribute will be omitted if HidePhoneNumber is set.
	AttrSIPTrunkNumber = AttrSIPPrefix + "trunkPhoneNumber"
	// AttrSIPPhoneNumber attribute contains number external to LiveKit SIP (caller for inbound and called number for outbound).
	// This attribute will be omitted if HidePhoneNumber is set.
	AttrSIPPhoneNumber = AttrSIPPrefix + "phoneNumber"
	// AttrSIPHostName attribute contains host name external to LiveKit SIP (caller for inbound and called number for outbound).
	AttrSIPHostName = AttrSIPPrefix + "hostname"
	// AttrSIPCallStatus attribute contains current call status for a SIP call associated with the participant.
	//
	// SIP participant is ready when it reaches "active" status.
	AttrSIPCallStatus = AttrSIPPrefix + "callStatus"
	// AttrSIPHeaderPrefix is a prefix for automatically mapped SIP header attributes.
	AttrSIPHeaderPrefix = AttrSIPPrefix + "h."

	// AttrIngressPrefix is shared for all Ingress attributes
	AttrIngressPrefix = "ingress."
	// AttrIngressID contains the ingressID associated with the participant
	AttrIngressID = AttrIngressPrefix + "ingressID"
	// AttrIngressResourceID coutains the resourceID of ths ingress session associated with the participant
	AttrIngressResourceID = AttrIngressPrefix + "resourceID"
	// AttrIngressOutOfNetworkPrefix is shared for all ingress out of network (Ads break) related attributes
	AttrIngressOutOfNetworkPrefix = AttrIngressPrefix + "outOfNetwork."
	// AttrIngressOutOfNetworkEventID contains the event ID of the current Out of network splice
	AttrIngressOutOfNetworkEventID = AttrIngressOutOfNetworkPrefix + "eventID"
)
