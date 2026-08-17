package observability

import (
	"github.com/livekit/protocol/observability/agentsobs"
	"github.com/livekit/protocol/observability/egressobs"
	"github.com/livekit/protocol/observability/gatewayobs"
	"github.com/livekit/protocol/observability/ingressobs"
	"github.com/livekit/protocol/observability/roomobs"
	"github.com/livekit/protocol/observability/storageobs"
	"github.com/livekit/protocol/observability/telephonycallobs"
	"github.com/livekit/protocol/observability/telephonyobs"
)

const Project = "livekit"

type Reporter interface {
	Room() roomobs.Reporter
	Agent() agentsobs.Reporter
	Gateway() gatewayobs.Reporter
	Telephony() telephonyobs.Reporter
	Connector() any // any is a placeholder for the connector type
	Egress() egressobs.Reporter
	Ingress() ingressobs.Reporter
	GatewayMetrics() any // any is a placeholder for the gateway metrics type
	TelephonyCall() telephonycallobs.Reporter
	Storage() storageobs.Reporter
	Close()
}

func NewReporter() Reporter {
	return reporter{}
}

type reporter struct{}

func (reporter) Room() roomobs.Reporter {
	return roomobs.NewNoopReporter()
}

func (reporter) Agent() agentsobs.Reporter {
	return agentsobs.NewNoopReporter()
}

func (reporter) Gateway() gatewayobs.Reporter {
	return gatewayobs.NewNoopReporter()
}

func (reporter) Telephony() telephonyobs.Reporter {
	return telephonyobs.NewNoopReporter()
}

func (reporter) Connector() any {
	return nil
}

func (reporter) Egress() egressobs.Reporter {
	return egressobs.NewNoopReporter()
}

func (reporter) Ingress() ingressobs.Reporter {
	return ingressobs.NewNoopReporter()
}

func (reporter) GatewayMetrics() any {
	return nil
}

func (reporter) TelephonyCall() telephonycallobs.Reporter {
	return telephonycallobs.NewNoopReporter()
}

func (reporter) Storage() storageobs.Reporter { return storageobs.NewNoopReporter() }

func (reporter) Close() {
}
