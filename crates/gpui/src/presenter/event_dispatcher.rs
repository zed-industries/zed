use std::sync::Arc;

use collections::{HashMap, HashSet};
use pathfinder_geometry::vector::Vector2F;

use crate::{
    scene::{
        ClickRegionEvent, DownOutRegionEvent, DownRegionEvent, DragRegionEvent, HoverRegionEvent,
        MouseRegionEvent, MoveRegionEvent, UpOutRegionEvent, UpRegionEvent,
    },
    CursorRegion, CursorStyle, ElementBox, Event, EventContext, FontCache, MouseButton,
    MouseMovedEvent, MouseRegion, MouseRegionId, MutableAppContext, Scene, TextLayoutCache,
};

pub struct EventDispatcher {
    window_id: usize,
    font_cache: Arc<FontCache>,

    last_mouse_moved_event: Option<Event>,
    cursor_regions: Vec<CursorRegion>,
    mouse_regions: Vec<(MouseRegion, usize)>,
    clicked_regions: Vec<MouseRegion>,
    clicked_button: Option<MouseButton>,
    mouse_position: Vector2F,
    hovered_region_ids: HashSet<MouseRegionId>,
}

impl EventDispatcher {
    pub fn new(window_id: usize, font_cache: Arc<FontCache>) -> Self {
        Self {
            window_id,
            font_cache,

            last_mouse_moved_event: Default::default(),
            cursor_regions: Default::default(),
            mouse_regions: Default::default(),
            clicked_regions: Default::default(),
            clicked_button: Default::default(),
            mouse_position: Default::default(),
            hovered_region_ids: Default::default(),
        }
    }

    pub fn clicked_region_ids(&self) -> Option<(Vec<MouseRegionId>, MouseButton)> {
        self.clicked_button.map(|button| {
            (
                self.clicked_regions
                    .iter()
                    .filter_map(MouseRegion::id)
                    .collect(),
                button,
            )
        })
    }

    pub fn hovered_region_ids(&self) -> HashSet<MouseRegionId> {
        self.hovered_region_ids.clone()
    }

    pub fn update_mouse_regions(&mut self, scene: &Scene) {
        self.cursor_regions = scene.cursor_regions();
        self.mouse_regions = scene.mouse_regions();
    }

    pub fn redispatch_mouse_moved_event<'a>(&'a mut self, cx: &mut EventContext<'a>) {
        if let Some(event) = self.last_mouse_moved_event.clone() {
            self.dispatch_event(event, true, cx);
        }
    }

    pub fn dispatch_event<'a>(
        &'a mut self,
        event: Event,
        event_reused: bool,
        cx: &mut EventContext<'a>,
    ) -> bool {
        let root_view_id = cx.root_view_id(self.window_id);
        if root_view_id.is_none() {
            return false;
        }

        let root_view_id = root_view_id.unwrap();
        //1. Allocate the correct set of GPUI events generated from the platform events
        // -> These are usually small: [Mouse Down] or [Mouse up, Click] or [Mouse Moved, Mouse Dragged?]
        // -> Also moves around mouse related state
        let events_to_send = self.select_region_events(&event, cx, event_reused);

        // For a given platform event, potentially multiple mouse region events can be created. For a given
        // region event, dispatch continues until a mouse region callback fails to propogate (handled is set to true)
        // If no region handles any of the produced platform events, we fallback to the old dispatch event style.
        let mut invalidated_views: HashSet<usize> = Default::default();
        let mut any_event_handled = false;
        for mut region_event in events_to_send {
            //2. Find mouse regions relevant to each region_event. For example, if the event is click, select
            // the clicked_regions that overlap with the mouse position
            let valid_regions = self.select_relevant_mouse_regions(&region_event);
            let hovered_region_ids = self.hovered_region_ids.clone();

            //3. Dispatch region event ot each valid mouse region
            for valid_region in valid_regions.into_iter() {
                region_event.set_region(valid_region.bounds);
                if let MouseRegionEvent::Hover(e) = &mut region_event {
                    e.started = valid_region
                        .id()
                        .map(|region_id| hovered_region_ids.contains(&region_id))
                        .unwrap_or(false)
                }

                if let Some(callback) = valid_region.handlers.get(&region_event.handler_key()) {
                    if !event_reused {
                        invalidated_views.insert(valid_region.view_id);
                    }

                    cx.handled = true;
                    cx.with_current_view(valid_region.view_id, {
                        let region_event = region_event.clone();
                        |cx| {
                            callback(region_event, cx);
                        }
                    });

                    // For bubbling events, if the event was handled, don't continue dispatching
                    // This only makes sense for local events.
                    if cx.handled && region_event.is_local() {
                        break;
                    }
                }
            }

            // Keep track if any platform event was handled
            any_event_handled = any_event_handled && cx.handled;
        }

        if !any_event_handled {
            // No platform event was handled, so fall back to old mouse event dispatch style
            any_event_handled = cx.dispatch_event(root_view_id, &event);
        }

        // Notify any views which have been validated from event callbacks
        for view_id in invalidated_views {
            cx.notify_view(self.window_id, view_id);
        }

        any_event_handled
    }

    fn select_region_events(
        &mut self,
        event: &Event,
        cx: &mut MutableAppContext,
        event_reused: bool,
    ) -> Vec<MouseRegionEvent> {
        let mut events_to_send = Vec::new();
        match event {
            Event::MouseDown(e) => {
                //Click events are weird because they can be fired after a drag event.
                //MDN says that browsers handle this by starting from 'the most
                //specific ancestor element that contained both [positions]'
                //So we need to store the overlapping regions on mouse down.
                self.clicked_regions = self
                    .mouse_regions
                    .iter()
                    .filter_map(|(region, _)| {
                        region
                            .bounds
                            .contains_point(e.position)
                            .then(|| region.clone())
                    })
                    .collect();
                self.clicked_button = Some(e.button);

                events_to_send.push(MouseRegionEvent::Down(DownRegionEvent {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
                events_to_send.push(MouseRegionEvent::DownOut(DownOutRegionEvent {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
            }
            Event::MouseUp(e) => {
                //NOTE: The order of event pushes is important! MouseUp events MUST be fired
                //before click events, and so the UpRegionEvent events need to be pushed before
                //ClickRegionEvents
                events_to_send.push(MouseRegionEvent::Up(UpRegionEvent {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
                events_to_send.push(MouseRegionEvent::UpOut(UpOutRegionEvent {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
                events_to_send.push(MouseRegionEvent::Click(ClickRegionEvent {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
            }
            Event::MouseMoved(
                e @ MouseMovedEvent {
                    position,
                    pressed_button,
                    ..
                },
            ) => {
                let mut style_to_assign = CursorStyle::Arrow;
                for region in self.cursor_regions.iter().rev() {
                    if region.bounds.contains_point(*position) {
                        style_to_assign = region.style;
                        break;
                    }
                }
                cx.platform().set_cursor_style(style_to_assign);

                if !event_reused {
                    if pressed_button.is_some() {
                        events_to_send.push(MouseRegionEvent::Drag(DragRegionEvent {
                            region: Default::default(),
                            prev_mouse_position: self.mouse_position,
                            platform_event: e.clone(),
                        }));
                    }
                    events_to_send.push(MouseRegionEvent::Move(MoveRegionEvent {
                        region: Default::default(),
                        platform_event: e.clone(),
                    }));
                }

                events_to_send.push(MouseRegionEvent::Hover(HoverRegionEvent {
                    region: Default::default(),
                    platform_event: e.clone(),
                    started: false,
                }));

                self.last_mouse_moved_event = Some(event.clone());
            }
            _ => {}
        }
        if let Some(position) = event.position() {
            self.mouse_position = position;
        }
        events_to_send
    }

    fn select_relevant_mouse_regions(
        &mut self,
        region_event: &MouseRegionEvent,
    ) -> Vec<MouseRegion> {
        let mut valid_regions = Vec::new();
        //GPUI elements are arranged by depth but sibling elements can register overlapping
        //mouse regions. As such, hover events are only fired on overlapping elements which
        //are at the same depth as the deepest element which overlaps with the mouse.
        if let MouseRegionEvent::Hover(_) = *region_event {
            let mut top_most_depth = None;
            let mouse_position = self.mouse_position.clone();
            for (region, depth) in self.mouse_regions.iter().rev() {
                let contains_mouse = region.bounds.contains_point(mouse_position);

                if contains_mouse && top_most_depth.is_none() {
                    top_most_depth = Some(depth);
                }

                if let Some(region_id) = region.id() {
                    //This unwrap relies on short circuiting boolean expressions
                    //The right side of the && is only executed when contains_mouse
                    //is true, and we know above that when contains_mouse is true
                    //top_most_depth is set
                    if contains_mouse && depth == top_most_depth.unwrap() {
                        //Ensure that hover entrance events aren't sent twice
                        if self.hovered_region_ids.insert(region_id) {
                            valid_regions.push(region.clone());
                        }
                    } else {
                        //Ensure that hover exit events aren't sent twice
                        if self.hovered_region_ids.remove(&region_id) {
                            valid_regions.push(region.clone());
                        }
                    }
                }
            }
        } else if let MouseRegionEvent::Click(e) = region_event {
            //Clear stored clicked_regions
            let clicked_regions = std::mem::replace(&mut self.clicked_regions, Vec::new());
            self.clicked_button = None;

            //Find regions which still overlap with the mouse since the last MouseDown happened
            for clicked_region in clicked_regions.into_iter().rev() {
                if clicked_region.bounds.contains_point(e.position) {
                    valid_regions.push(clicked_region);
                }
            }
        } else if region_event.is_local() {
            for (mouse_region, _) in self.mouse_regions.iter().rev() {
                //Contains
                if mouse_region.bounds.contains_point(self.mouse_position) {
                    valid_regions.push(mouse_region.clone());
                }
            }
        } else {
            for (mouse_region, _) in self.mouse_regions.iter().rev() {
                //NOT contains
                if !mouse_region.bounds.contains_point(self.mouse_position) {
                    valid_regions.push(mouse_region.clone());
                }
            }
        }
        valid_regions
    }
}
