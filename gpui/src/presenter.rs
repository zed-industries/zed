use crate::{
    app::{AppContext, MutableAppContext, WindowInvalidation},
    elements::Element,
    font_cache::FontCache,
    json::{self, ToJson},
    platform::Event,
    text_layout::TextLayoutCache,
    AssetCache, ElementBox, Scene,
};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub struct Presenter {
    window_id: usize,
    rendered_views: HashMap<usize, ElementBox>,
    parents: HashMap<usize, usize>,
    font_cache: Arc<FontCache>,
    text_layout_cache: TextLayoutCache,
    asset_cache: Arc<AssetCache>,
    last_mouse_moved_event: Option<Event>,
}

impl Presenter {
    pub fn new(
        window_id: usize,
        font_cache: Arc<FontCache>,
        text_layout_cache: TextLayoutCache,
        asset_cache: Arc<AssetCache>,
        cx: &MutableAppContext,
    ) -> Self {
        Self {
            window_id,
            rendered_views: cx.render_views(window_id),
            parents: HashMap::new(),
            font_cache,
            text_layout_cache,
            asset_cache,
            last_mouse_moved_event: None,
        }
    }

    pub fn dispatch_path(&self, app: &AppContext) -> Vec<usize> {
        let mut view_id = app.focused_view_id(self.window_id).unwrap();
        let mut path = vec![view_id];
        while let Some(parent_id) = self.parents.get(&view_id).copied() {
            path.push(parent_id);
            view_id = parent_id;
        }
        path.reverse();
        path
    }

    pub fn invalidate(&mut self, mut invalidation: WindowInvalidation, cx: &AppContext) {
        for view_id in invalidation.removed {
            invalidation.updated.remove(&view_id);
            self.rendered_views.remove(&view_id);
            self.parents.remove(&view_id);
        }
        for view_id in invalidation.updated {
            self.rendered_views
                .insert(view_id, cx.render_view(self.window_id, view_id).unwrap());
        }
    }

    pub fn build_scene(
        &mut self,
        window_size: Vector2F,
        scale_factor: f32,
        cx: &mut MutableAppContext,
    ) -> Scene {
        let mut scene = Scene::new(scale_factor);

        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            self.layout(window_size, cx);
            self.after_layout(cx);
            let mut paint_cx = PaintContext {
                scene: &mut scene,
                font_cache: &self.font_cache,
                text_layout_cache: &self.text_layout_cache,
                rendered_views: &mut self.rendered_views,
                app: cx.as_ref(),
            };
            paint_cx.paint(root_view_id, Vector2F::zero());
            self.text_layout_cache.finish_frame();

            if let Some(event) = self.last_mouse_moved_event.clone() {
                self.dispatch_event(event, cx)
            }
        } else {
            log::error!("could not find root_view_id for window {}", self.window_id);
        }

        scene
    }

    fn layout(&mut self, size: Vector2F, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            let mut layout_ctx = LayoutContext {
                rendered_views: &mut self.rendered_views,
                parents: &mut self.parents,
                font_cache: &self.font_cache,
                text_layout_cache: &self.text_layout_cache,
                asset_cache: &self.asset_cache,
                view_stack: Vec::new(),
                app: cx,
            };
            layout_ctx.layout(root_view_id, SizeConstraint::strict(size));
        }
    }

    fn after_layout(&mut self, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            let mut layout_cx = AfterLayoutContext {
                rendered_views: &mut self.rendered_views,
                font_cache: &self.font_cache,
                text_layout_cache: &self.text_layout_cache,
                app: cx,
            };
            layout_cx.after_layout(root_view_id);
        }
    }

    pub fn dispatch_event(&mut self, event: Event, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            if matches!(event, Event::MouseMoved { .. }) {
                self.last_mouse_moved_event = Some(event.clone());
            }

            let mut event_cx = EventContext {
                rendered_views: &mut self.rendered_views,
                actions: Default::default(),
                font_cache: &self.font_cache,
                text_layout_cache: &self.text_layout_cache,
                view_stack: Default::default(),
                invalidated_views: Default::default(),
                app: cx,
            };
            event_cx.dispatch_event(root_view_id, &event);

            let invalidated_views = event_cx.invalidated_views;
            let actions = event_cx.actions;

            for view_id in invalidated_views {
                cx.notify_view(self.window_id, view_id);
            }
            for action in actions {
                cx.dispatch_action_any(
                    self.window_id,
                    &action.path,
                    action.name,
                    action.arg.as_ref(),
                );
            }
        }
    }

    pub fn debug_elements(&self, cx: &AppContext) -> Option<json::Value> {
        cx.root_view_id(self.window_id)
            .and_then(|root_view_id| self.rendered_views.get(&root_view_id))
            .map(|root_element| {
                root_element.debug(&DebugContext {
                    rendered_views: &self.rendered_views,
                    font_cache: &self.font_cache,
                    app: cx,
                })
            })
    }
}

pub struct ActionToDispatch {
    pub path: Vec<usize>,
    pub name: &'static str,
    pub arg: Box<dyn Any>,
}

pub struct LayoutContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    parents: &'a mut HashMap<usize, usize>,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub asset_cache: &'a AssetCache,
    pub app: &'a mut MutableAppContext,
    view_stack: Vec<usize>,
}

impl<'a> LayoutContext<'a> {
    fn layout(&mut self, view_id: usize, constraint: SizeConstraint) -> Vector2F {
        if let Some(parent_id) = self.view_stack.last() {
            self.parents.insert(view_id, *parent_id);
        }
        self.view_stack.push(view_id);
        let mut rendered_view = self.rendered_views.remove(&view_id).unwrap();
        let size = rendered_view.layout(constraint, self);
        self.rendered_views.insert(view_id, rendered_view);
        self.view_stack.pop();
        size
    }
}

pub struct AfterLayoutContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a mut MutableAppContext,
}

impl<'a> AfterLayoutContext<'a> {
    fn after_layout(&mut self, view_id: usize) {
        if let Some(mut view) = self.rendered_views.remove(&view_id) {
            view.after_layout(self);
            self.rendered_views.insert(view_id, view);
        }
    }
}

pub struct PaintContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    pub scene: &'a mut Scene,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a AppContext,
}

impl<'a> PaintContext<'a> {
    fn paint(&mut self, view_id: usize, origin: Vector2F) {
        if let Some(mut tree) = self.rendered_views.remove(&view_id) {
            tree.paint(origin, self);
            self.rendered_views.insert(view_id, tree);
        }
    }
}

pub struct EventContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    actions: Vec<ActionToDispatch>,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a mut MutableAppContext,
    view_stack: Vec<usize>,
    invalidated_views: HashSet<usize>,
}

impl<'a> EventContext<'a> {
    fn dispatch_event(&mut self, view_id: usize, event: &Event) -> bool {
        if let Some(mut element) = self.rendered_views.remove(&view_id) {
            self.view_stack.push(view_id);
            let result = element.dispatch_event(event, self);
            self.view_stack.pop();
            self.rendered_views.insert(view_id, element);
            result
        } else {
            false
        }
    }

    pub fn dispatch_action<A: 'static + Any>(&mut self, name: &'static str, arg: A) {
        self.actions.push(ActionToDispatch {
            path: self.view_stack.clone(),
            name,
            arg: Box::new(arg),
        });
    }

    pub fn notify(&mut self) {
        self.invalidated_views
            .insert(*self.view_stack.last().unwrap());
    }
}

pub struct DebugContext<'a> {
    rendered_views: &'a HashMap<usize, ElementBox>,
    pub font_cache: &'a FontCache,
    pub app: &'a AppContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn invert(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

impl ToJson for Axis {
    fn to_json(&self) -> serde_json::Value {
        match self {
            Axis::Horizontal => json!("horizontal"),
            Axis::Vertical => json!("vertical"),
        }
    }
}

pub trait Vector2FExt {
    fn along(self, axis: Axis) -> f32;
}

impl Vector2FExt for Vector2F {
    fn along(self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.x(),
            Axis::Vertical => self.y(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SizeConstraint {
    pub min: Vector2F,
    pub max: Vector2F,
}

impl SizeConstraint {
    pub fn new(min: Vector2F, max: Vector2F) -> Self {
        Self { min, max }
    }

    pub fn strict(size: Vector2F) -> Self {
        Self {
            min: size,
            max: size,
        }
    }

    pub fn strict_along(axis: Axis, max: f32) -> Self {
        match axis {
            Axis::Horizontal => Self {
                min: vec2f(max, 0.0),
                max: vec2f(max, f32::INFINITY),
            },
            Axis::Vertical => Self {
                min: vec2f(0.0, max),
                max: vec2f(f32::INFINITY, max),
            },
        }
    }

    pub fn max_along(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.max.x(),
            Axis::Vertical => self.max.y(),
        }
    }
}

impl ToJson for SizeConstraint {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "min": self.min.to_json(),
            "max": self.max.to_json(),
        })
    }
}

pub struct ChildView {
    view_id: usize,
}

impl ChildView {
    pub fn new(view_id: usize) -> Self {
        Self { view_id }
    }
}

impl Element for ChildView {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = cx.layout(self.view_id, constraint);
        (size, ())
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        cx: &mut AfterLayoutContext,
    ) {
        cx.after_layout(self.view_id);
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        cx.paint(self.view_id, bounds.origin());
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        cx.dispatch_event(self.view_id, event)
    }

    fn debug(
        &self,
        bounds: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "ChildView",
            "view_id": self.view_id,
            "bounds": bounds.to_json(),
            "child": if let Some(view) = cx.rendered_views.get(&self.view_id) {
                view.debug(cx)
            } else {
                json!(null)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_responder_chain() {
    //     let settings = settings_rx(None);
    //     let mut app = App::new().unwrap();
    //     let workspace = app.add_model(|cx| Workspace::new(Vec::new(), cx));
    //     let (window_id, workspace_view) =
    //         app.add_window(|cx| WorkspaceView::new(workspace.clone(), settings, cx));

    //     let invalidations = Rc::new(RefCell::new(Vec::new()));
    //     let invalidations_ = invalidations.clone();
    //     app.on_window_invalidated(window_id, move |invalidation, _| {
    //         invalidations_.borrow_mut().push(invalidation)
    //     });

    //     let active_pane_id = workspace_view.update(&mut app, |view, cx| {
    //         cx.focus(view.active_pane());
    //         view.active_pane().id()
    //     });

    //     app.update(|app| {
    //         let mut presenter = Presenter::new(
    //             window_id,
    //             Rc::new(FontCache::new()),
    //             Rc::new(AssetCache::new()),
    //             app,
    //         );
    //         for invalidation in invalidations.borrow().iter().cloned() {
    //             presenter.update(vec2f(1024.0, 768.0), 2.0, Some(invalidation), app);
    //         }

    //         assert_eq!(
    //             presenter.responder_chain(app.cx()).unwrap(),
    //             vec![workspace_view.id(), active_pane_id]
    //         );
    //     });
    // }
}
