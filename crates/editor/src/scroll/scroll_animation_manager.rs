use gpui::{Point, point};
use crate::{WorkspaceId, DisplaySnapshot};
use std::time::Instant;

pub(crate) enum UpdateResponse {
    Finished {
        end_position: Point<f32>,
        state: PersistentState
    },
    Nothing,
    RequiresAnimationFrame {
        updated_position: Point<f32>
    }
}

#[derive(Clone)]
pub(crate) struct PersistentState {
    pub(crate) snapshot: DisplaySnapshot,
    pub(crate) workspace_id: Option<WorkspaceId>,
}

pub(crate) struct Anim {
    start:          f32,
    delta:          f32,
    start_moment:   Instant,
    state:          PersistentState,
}

pub(crate) struct ScrollAnimationManager {
    anim: Option<Anim>,
}

impl ScrollAnimationManager {
    pub(crate) fn new() -> Self {
        ScrollAnimationManager {
            anim: None,
        }
    }

    pub(crate) fn start(
        &mut self, 
        from: Point<f32>, 
        to: Point<f32>, 
        snapshot: &DisplaySnapshot,
        workspace_id: Option<WorkspaceId>
    ) {
        self.anim = Some(Anim {
            start: from.y,
            delta: to.y - from.y,
            start_moment: Instant::now(),
            state: PersistentState {
                snapshot: snapshot.clone(),
                workspace_id
            }
        });
    }

    pub(crate) fn has_anim(&self) -> bool {
        self.anim.is_some()
    }
    
    pub(crate) fn get_state(&self) -> Option<PersistentState> {
        self.anim.as_ref().map(|v| v.state.clone())
    }

    pub(crate) fn update(&mut self) -> UpdateResponse {
        const MAX_DUR: f32 = 0.5;

        if let Some(anim) = &self.anim {
            let time_since_start = anim.start_moment.elapsed().as_secs_f32();
            if time_since_start >= MAX_DUR {
                let anim = self.anim.take().unwrap(); 
                UpdateResponse::Finished { 
                    end_position: point(0.0, anim.start + anim.delta), 
                    state: anim.state 
                }
            } else {
                let curr_y = anim.start + (time_since_start * anim.delta / MAX_DUR);
                UpdateResponse::RequiresAnimationFrame { updated_position: point(0.0, curr_y) }
            }
        } else {
            UpdateResponse::Nothing
        }
    }
}
