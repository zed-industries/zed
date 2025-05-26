use gpui::{Point, point, App};
use crate::{WorkspaceId, DisplaySnapshot, EditorSettings};
use settings::Settings;
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

    // here for completedness, actually useless
    #[allow(dead_code)]
    pub(crate) autoscroll: bool,
    #[allow(dead_code)]
    pub(crate) local: bool,
}

pub(crate) struct Anim {
    start:          f32,
    delta:          f32,
    start_moment:   Instant,
    state:          PersistentState,
}

pub(crate) struct ScrollAnimationManager {
    anim: Option<Anim>,
    scroll_duration: f32,
}

impl ScrollAnimationManager {
    pub(crate) fn new(cx: &App) -> Self {
        ScrollAnimationManager {
            anim: None,
            scroll_duration: EditorSettings::get_global(cx).smooth_scroll_duration
        }
    }

    pub(crate) fn start(
        &mut self, 
        from: Point<f32>, 
        to: Point<f32>, 
        local: bool,
        autoscroll: bool,
        snapshot: &DisplaySnapshot,
        workspace_id: Option<WorkspaceId>
    ) {
        self.anim = Some(Anim {
            start: from.y,
            delta: to.y - from.y,
            start_moment: Instant::now(),
            state: PersistentState {
                snapshot: snapshot.clone(),
                workspace_id,
                autoscroll,
                local
            }
        });
    }

    pub(crate) fn set_duration(&mut self, new_dur: f32) {
        self.scroll_duration = new_dur;
    }

    pub(crate) fn has_anim(&self) -> bool {
        self.anim.is_some()
    }
    
    pub(crate) fn get_state(&self) -> Option<PersistentState> {
        self.anim.as_ref().map(|v| v.state.clone())
    }

    pub(crate) fn update(&mut self) -> UpdateResponse {
        if let Some(anim) = &self.anim {
            let time_since_start = anim.start_moment.elapsed().as_secs_f32();
            if time_since_start >= self.scroll_duration {
                let anim = self.anim.take().unwrap(); 
                UpdateResponse::Finished { 
                    end_position: point(0.0, anim.start + anim.delta), 
                    state: anim.state 
                }
            } else {
                let curr_y = anim.start + (anim.delta * time_since_start / self.scroll_duration);
                UpdateResponse::RequiresAnimationFrame { updated_position: point(0.0, curr_y) }
            }
        } else {
            UpdateResponse::Nothing
        }
    }
}
