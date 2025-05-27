use gpui::{App, Point, point};
use crate::{
    WorkspaceId, EditorSettings, ScrollAnchor, RowExt, DisplayRow, DisplayPoint,
    display_map::{DisplaySnapshot, ToDisplayPoint},
};
use language::Bias;
use settings::Settings;
use std::time::Instant;

pub(crate) enum UpdateResponse {
    Finished {
        destination_anchor:    ScrollAnchor,
        destination_top_row:   u32,
        state:                 PersistentState, 
    },
    Nothing,
    RequiresAnimationFrame {
        intermediate_anchor:    ScrollAnchor,
        intermediate_top_row:   u32,
    }
}

#[derive(Clone)]
pub(crate) struct PersistentState {
    pub(crate) map:             DisplaySnapshot,
    pub(crate) workspace_id:    Option<WorkspaceId>,
    pub(crate) local:           bool,
    pub(crate) autoscroll:      bool,
}

pub(crate) struct Anim {
    start:                  f32,
    delta:                  f32,
    destination_top_row:    u32,
    destination_anchor:     ScrollAnchor,
    start_moment:           Instant,
    state:                  PersistentState,
}

impl Anim {
    pub(crate) fn new(
        from: Point<f32>, 
        destination_top_row: u32,
        destination_anchor: ScrollAnchor, 
        map: DisplaySnapshot,
        workspace_id: Option<WorkspaceId>,
        local: bool,
        autoscroll: bool,
    ) -> Anim {
        let start = from.y;
        let end = destination_anchor.offset.y + destination_anchor.anchor.to_display_point(&map).row().as_f32();
        let delta = end - start;
        Anim {
            start,
            delta,
            destination_top_row,
            destination_anchor,
            start_moment: Instant::now(),
            state: PersistentState {
                map,
                workspace_id,
                local,
                autoscroll,
            }
        }
    }
}

pub(crate) struct ScrollAnimationManager {
    anim: Option<Anim>,
    scroll_duration: f32,
}

impl ScrollAnimationManager {
    pub(crate) fn new(cx: &App) -> Self {
        ScrollAnimationManager {
            anim: None,
            scroll_duration: EditorSettings::get_global(cx).smooth_scroll_duration.max(0.)
        }
    }

    pub(crate) fn start(&mut self, anim: Anim) {
        self.anim = Some(anim);
    }

    pub(crate) fn set_duration(&mut self, new_dur: f32) {
        self.scroll_duration = new_dur.max(0.);
    }

    pub(crate) fn has_anim(&self) -> bool {
        self.anim.is_some()
    }
    
    pub(crate) fn get_state(&self) -> Option<PersistentState> {
        self.anim.as_ref().map(|v| v.state.clone())
    }

    fn make_final_results(&self, intermediate_scroll_top: f32, map: &DisplaySnapshot) -> (ScrollAnchor, u32) {
        // the logic here is roughly the same as what you'd find in
        // [ScrollManager::set_scroll_position()]
        // the idea is to build objects that [ScrollManager::set_anchor()] can exploit
        // using our calculated intermediate_scroll_top
        let scroll_top_buffer_point =
            DisplayPoint::new(DisplayRow(intermediate_scroll_top as u32), 0).to_point(map);
        let new_top_anchor = map
            .buffer_snapshot
            .anchor_at(scroll_top_buffer_point, Bias::Right);

        (
            ScrollAnchor {
                anchor: new_top_anchor,
                offset: point(
                    // mamamia we don't support horizontal scrolling yet ...
                    self.anim.as_ref().unwrap().destination_anchor.offset.x, 
                    intermediate_scroll_top - new_top_anchor.to_display_point(map).row().as_f32(),
                ),
            },
            scroll_top_buffer_point.row 
        )
    }

    pub(crate) fn update(&mut self) -> UpdateResponse {
        if let Some(anim) = &self.anim {
            let time_since_start = anim.start_moment.elapsed().as_secs_f32();
            if time_since_start >= self.scroll_duration {
                let anim = self.anim.take().unwrap(); 
                UpdateResponse::Finished { 
                    destination_top_row: anim.destination_top_row,
                    destination_anchor: anim.destination_anchor,
                    state: anim.state,
                }
            } else {
                let new_scroll_top = 
                    anim.start + (anim.delta * time_since_start / self.scroll_duration);

                let (intermediate_anchor, intermediate_top_row) = 
                    self.make_final_results(new_scroll_top, &anim.state.map);

                UpdateResponse::RequiresAnimationFrame { 
                    intermediate_anchor,
                    intermediate_top_row,
                }
            }
        } else {
            UpdateResponse::Nothing
        }
    }
}
