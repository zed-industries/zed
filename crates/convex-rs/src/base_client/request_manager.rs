use std::collections::{
    BTreeMap,
    BTreeSet,
    VecDeque,
};

use convex_sync_types::{
    ClientMessage,
    Timestamp,
};
use tokio::sync::oneshot;

use crate::{
    sync::ReconnectProtocolReason,
    FunctionResult,
};

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct RequestId(u32);
impl RequestId {
    pub fn new(id: u32) -> Self {
        RequestId(id)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum RequestType {
    Mutation,
    Action,
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum RequestStatus {
    Requested,
    Completed,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Request {
    pub id: RequestId,
    pub typ: RequestType,
    pub status: RequestStatus,
    pub ts: Option<Timestamp>,
    pub value: Option<FunctionResult>,
    pub message: ClientMessage,
}

impl Request {
    pub fn new(id: RequestId, typ: RequestType, message: ClientMessage) -> Self {
        Request {
            id,
            typ,
            status: RequestStatus::Requested,
            ts: None,
            value: None,
            message,
        }
    }

    pub fn update_value(&mut self, value: FunctionResult) {
        self.value = Some(value);
    }

    pub fn update_timestamp(&mut self, ts: Option<Timestamp>) {
        self.ts = ts;
    }
}

pub struct RequestManager {
    ongoing_requests: BTreeMap<RequestId, (Request, oneshot::Sender<FunctionResult>)>,
}

impl RequestManager {
    pub fn new() -> Self {
        RequestManager {
            ongoing_requests: BTreeMap::new(),
        }
    }

    pub fn update_request(
        &mut self,
        request_id: &RequestId,
        request_type: RequestType,
        value: FunctionResult,
        ts: Option<Timestamp>,
    ) -> Result<(), ReconnectProtocolReason> {
        let Some((request, _)) = self.ongoing_requests.get_mut(request_id) else {
            return Err("Invalid request id from server".to_string());
        };
        if request.typ != request_type {
            return Err("Mismatched request type from server".to_string());
        };
        let errored = matches!(value, FunctionResult::ErrorMessage(_));
        request.update_value(value);
        request.update_timestamp(ts);
        request.status = RequestStatus::Completed;

        // Actions and errored mutations are ok to complete immediately
        if request_type == RequestType::Action || errored {
            self._remove_and_notify_completed(request_id);
        }
        Ok(())
    }

    pub fn remove_and_notify_completed(&mut self, ts: Timestamp) -> BTreeSet<RequestId> {
        let mut completed_requests = BTreeSet::new();
        for (id, (request, _)) in self.ongoing_requests.iter() {
            let mut is_completed = false;
            if request.status == RequestStatus::Completed {
                is_completed = true;
            }
            if let Some(request_ts) = request.ts {
                if request_ts <= ts {
                    is_completed = true;
                }
            }
            if is_completed {
                completed_requests.insert(*id);
            }
        }
        for id in completed_requests.iter() {
            self._remove_and_notify_completed(id);
        }
        completed_requests
    }

    fn _remove_and_notify_completed(&mut self, request_id: &RequestId) {
        let (request, sender) = self
            .ongoing_requests
            .remove(request_id)
            .expect("INTERNAL BUG: request_id must be present");
        if let Err(value) = sender.send(
            request
                .value
                .expect("INTERNAL BUG: Value missing on completed request"),
        ) {
            tracing::info!(
                "Request {request_id:?} completed with result {value:?}, but result receiver was \
                 dropped"
            );
        }
    }

    pub fn track_request(
        &mut self,
        message: &ClientMessage,
        request_id: RequestId,
        request_type: RequestType,
    ) -> oneshot::Receiver<FunctionResult> {
        let (tx, rx) = oneshot::channel();
        let request = Request::new(request_id, request_type, message.clone());
        self.ongoing_requests.insert(request_id, (request, tx));
        rx
    }

    pub fn restart(&self) -> VecDeque<ClientMessage> {
        // Sort ongoing requests by timestamp
        let mut ordered_requests = Vec::from_iter(self.ongoing_requests.values());
        ordered_requests.sort_by(|(req_a, _), (req_b, _)| req_b.ts.cmp(&req_a.ts));

        let mut messages = VecDeque::new();
        for (request, _) in ordered_requests {
            messages.push_back(request.message.clone());
        }
        messages
    }
}
