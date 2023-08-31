use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct InteractionMap(Arc<[Option<InteractionType>]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InteractionId {
    event_type: InteractionType,
    capture: u32,
}

const DEFAULT_INTERACTION_ID: InteractionId = InteractionId {
    event_type: InteractionType::Click,
    capture: u32::MAX,
};

impl super::buffer::QueryFeatureMap for InteractionMap {
    type Id = InteractionId;

    fn get(&self, capture_id: u32) -> Self::Id {
        match self.0.get(capture_id as usize) {
            Some(Some(event)) => InteractionId {
                event_type: *event,
                capture: capture_id,
            },
            _ => DEFAULT_INTERACTION_ID,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InteractionType {
    Click,
}

impl InteractionMap {
    pub fn new(capture_names: &[String]) -> Self {
        InteractionMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    let mut capture_parts = capture_name.split(".").peekable();
                    if let Some(str) = capture_parts.next() {
                        if str == "click" && capture_parts.peek().is_some() {
                            return Some(InteractionType::Click);
                        }
                    }
                    None
                })
                .collect(),
        )
    }
}

impl Default for InteractionMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}
