use gpui::*;
use gpui::prelude::*;

struct Clicked;

struct DraggableRectangle {
    position: Point<Pixels>,
    drag_origin: Option<(Point<Pixels>, Point<Pixels>)>,
    color: Rgba,
    blend_mode: BlendMode,
    width: Pixels,
    height: Pixels,
}

impl DraggableRectangle {
    fn new(position: Point<Pixels>, width: Pixels, height: Pixels, color: Rgba, blend_mode: BlendMode) -> Self {
        Self { position, drag_origin: None, color, blend_mode, width, height }
    }

    fn mode_name(&self) -> &'static str {
        match self.blend_mode {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Add => "Add",
            BlendMode::Subtract => "Subtract",
            BlendMode::Invert => "Invert",
            BlendMode::Screen => "Screen",
        }
    }
}

impl EventEmitter<Clicked> for DraggableRectangle {}

impl Render for DraggableRectangle {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut bg_color = self.color;
        if self.blend_mode == BlendMode::Normal {
            bg_color.a = 0.6;
        };
        div()
            .absolute()
            .top(self.position.y)
            .left(self.position.x)
            .w(self.width)
            .h(self.height)
            .bg(bg_color)
            .blend_mode(self.blend_mode)
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .absolute()
                    .top_2()
                    .left_2()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .line_height(px(18.0))
                            .text_color(black())
                            .font_family("Helvetica")
                            .child(self.mode_name())
                    )
                    .when(bg_color.a < 1.0, |this| {
                        this.child(
                            div()
                                .text_size(px(20.0))
                                .text_color(black())
                                .font_family("Helvetica")
                                .child(format!("Opacity {:.0}%", bg_color.a * 100.0))
                        )
                    })
            )
            .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                this.drag_origin = Some((event.position, this.position));
                cx.stop_propagation();
                cx.emit(Clicked);
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if let Some((mouse_start, rect_start)) = this.drag_origin {
                    this.position = rect_start + (event.position - mouse_start);
                    cx.notify();
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.drag_origin = None;
                cx.notify();
            }))
            .on_mouse_up_out(MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.drag_origin = None;
                cx.notify();
            }))
    }
}

struct DraggableRectangles {
    rectangles: Vec<Entity<DraggableRectangle>>,
}

impl DraggableRectangles {
    fn new(rectangles: Vec<Entity<DraggableRectangle>>, cx: &mut Context<Self>) -> Self {
        for rect in &rectangles {
            let handle = rect.downgrade();
            cx.subscribe(rect, move |this, _subscription, _: &Clicked, cx| {
                if let Some(handle) = handle.upgrade() {
                    this.rectangles.retain(|r| r != &handle);
                    this.rectangles.push(handle);
                    cx.notify();
                }
            }).detach();
        }
        Self { rectangles }
    }
}

impl Render for DraggableRectangles {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x888888))
            .relative()
            .child(
                div()
                    .absolute()
                    .top(px(150.0))
                    .left(px(150.0))
                    .w(px(1250.0))
                    .h(px(200.0))
                    .bg(rgb(0x5555ff))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .child(div().text_color(white()).text_size(px(128.0)).font_family("Helvetica").child("Blend "))
                            .child(div().text_color(black()).text_size(px(128.0)).font_family("Helvetica").child("Modes"))
                    )
            )
            .children(self.rectangles.clone())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size(px(1550.0), px(400.0)), cx))),
                is_resizable: false,
                ..Default::default()
            },
            |_, cx| {
                let rectangles = vec![
                    cx.new(|_| DraggableRectangle::new(Point { x: px(50.0), y: px(50.0) }, px(200.0), px(200.0), rgb(0xff5555), BlendMode::Normal)),
                    cx.new(|_| DraggableRectangle::new(Point { x: px(300.0), y: px(50.0) }, px(200.0), px(200.0), rgb(0xff5555), BlendMode::Multiply)),
                    cx.new(|_| DraggableRectangle::new(Point { x: px(550.0), y: px(50.0) }, px(200.0), px(200.0), rgb(0xff5555), BlendMode::Add)),
                    cx.new(|_| DraggableRectangle::new(Point { x: px(800.0), y: px(50.0) }, px(200.0), px(200.0), rgb(0xff5555), BlendMode::Subtract)),
                    cx.new(|_| DraggableRectangle::new(Point { x: px(1050.0), y: px(50.0) }, px(200.0), px(200.0), rgb(0xff5555), BlendMode::Invert)),
                    cx.new(|_| DraggableRectangle::new(Point { x: px(1300.0), y: px(50.0) }, px(200.0), px(200.0), rgb(0xff5555), BlendMode::Screen)),
                ];
                cx.new(|cx| DraggableRectangles::new(rectangles, cx))
            },
        ).unwrap();
    });
}