package roomobs

import (
	"github.com/livekit/protocol/livekit"
)

type ParticipantReporterResolver interface {
	Resolve(roomName livekit.RoomName, roomID livekit.RoomID, participant livekit.ParticipantIdentity, pID livekit.ParticipantID)
	Reset()
}

type deferredParticipantResolver struct {
	room               KeyResolver
	roomSession        KeyResolver
	participant        KeyResolver
	participantSession KeyResolver
}

func DeferredParticipantReporter(p ProjectReporter) (ParticipantSessionReporter, ParticipantReporterResolver) {
	room, roomResolver := p.WithDeferredRoom()
	roomSession, roomSessionResolver := room.WithDeferredRoomSession()
	participant, participantResolver := roomSession.WithDeferredParticipant()
	participantSession, participantSessionResolver := participant.WithDeferredParticipantSession()

	return participantSession, deferredParticipantResolver{roomResolver, roomSessionResolver, participantResolver, participantSessionResolver}
}

func (r deferredParticipantResolver) Resolve(roomName livekit.RoomName, roomID livekit.RoomID, participant livekit.ParticipantIdentity, pID livekit.ParticipantID) {
	r.room.Resolve(string(roomName))
	r.roomSession.Resolve(string(roomID))
	r.participant.Resolve(string(participant))
	r.participantSession.Resolve(string(pID))
}

func (r deferredParticipantResolver) Reset() {
	r.room.Reset()
	r.roomSession.Reset()
	r.participant.Reset()
	r.participantSession.Reset()
}
