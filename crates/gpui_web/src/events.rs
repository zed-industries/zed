use std::cell::RefCell;
use std::rc::Rc;

use gpui::{
    Capslock, ExternalPaths, FileDropEvent, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, NavigationDirection, Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent,
    TouchPhase, point, px,
};
use smallvec::smallvec;
use wasm_bindgen::prelude::*;

use crate::window::{WebWindowCallbacks, WebWindowMutableState};

pub struct WebEventListeners {
    #[allow(dead_code)]
    closures: Vec<Closure<dyn FnMut(JsValue)>>,
}

struct ClickState {
    last_position: Point<Pixels>,
    last_time: f64,
    count: usize,
}

impl Default for ClickState {
    fn default() -> Self {
        Self {
            last_position: Point::default(),
            last_time: 0.0,
            count: 0,
        }
    }
}

impl ClickState {
    fn register_click(&mut self, position: Point<Pixels>, time: f64) -> usize {
        let distance = ((f32::from(position.x) - f32::from(self.last_position.x)).powi(2)
            + (f32::from(position.y) - f32::from(self.last_position.y)).powi(2))
        .sqrt();

        if (time - self.last_time) < 400.0 && distance < 5.0 {
            self.count += 1;
        } else {
            self.count = 1;
        }

        self.last_position = position;
        self.last_time = time;
        self.count
    }
}

pub fn register_event_listeners(
    canvas: &web_sys::HtmlCanvasElement,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
) -> WebEventListeners {
    let mut closures: Vec<Closure<dyn FnMut(JsValue)>> = Vec::new();
    let click_state = Rc::new(RefCell::new(ClickState::default()));
    let pressed_button: Rc<RefCell<Option<MouseButton>>> = Rc::new(RefCell::new(None));
    let is_mac = is_mac_platform();

    register_pointer_down(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        Rc::clone(&click_state),
        Rc::clone(&pressed_button),
        is_mac,
    );
    register_pointer_up(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        Rc::clone(&click_state),
        Rc::clone(&pressed_button),
        is_mac,
    );
    register_pointer_move(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        Rc::clone(&pressed_button),
        is_mac,
    );
    register_pointer_leave(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        Rc::clone(&pressed_button),
        is_mac,
    );
    register_wheel(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        is_mac,
    );
    register_context_menu(canvas, &mut closures);
    register_drag_events(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
    );

    register_key_down(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        is_mac,
    );
    register_key_up(
        canvas,
        &mut closures,
        Rc::clone(&callbacks),
        Rc::clone(&state),
        is_mac,
    );

    WebEventListeners { closures }
}

fn register_pointer_down(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    click_state: Rc<RefCell<ClickState>>,
    pressed_button: Rc<RefCell<Option<MouseButton>>>,
    is_mac: bool,
) {
    let canvas_clone = canvas.clone();
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::PointerEvent = event.unchecked_into();
        event.prevent_default();
        canvas_clone.focus().ok();

        let button = dom_mouse_button_to_gpui(event.button());
        let position = pointer_position_in_element(&event);
        let modifiers = modifiers_from_mouse_event(&event, is_mac);
        let time = js_sys::Date::now();

        *pressed_button.borrow_mut() = Some(button);

        let click_count = click_state.borrow_mut().register_click(position, time);

        {
            let mut current_state = state.borrow_mut();
            current_state.mouse_position = position;
            current_state.modifiers = modifiers;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::MouseDown(MouseDownEvent {
                button,
                position,
                modifiers,
                click_count,
                first_mouse: false,
            }),
        );
    });
    canvas
        .add_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_pointer_up(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    click_state: Rc<RefCell<ClickState>>,
    pressed_button: Rc<RefCell<Option<MouseButton>>>,
    is_mac: bool,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::PointerEvent = event.unchecked_into();
        event.prevent_default();

        let button = dom_mouse_button_to_gpui(event.button());
        let position = pointer_position_in_element(&event);
        let modifiers = modifiers_from_mouse_event(&event, is_mac);

        *pressed_button.borrow_mut() = None;

        let click_count = click_state.borrow().count;

        {
            let mut current_state = state.borrow_mut();
            current_state.mouse_position = position;
            current_state.modifiers = modifiers;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::MouseUp(MouseUpEvent {
                button,
                position,
                modifiers,
                click_count,
            }),
        );
    });
    canvas
        .add_event_listener_with_callback("pointerup", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_pointer_move(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    pressed_button: Rc<RefCell<Option<MouseButton>>>,
    is_mac: bool,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::PointerEvent = event.unchecked_into();
        event.prevent_default();

        let position = pointer_position_in_element(&event);
        let modifiers = modifiers_from_mouse_event(&event, is_mac);
        let current_pressed = *pressed_button.borrow();

        {
            let mut current_state = state.borrow_mut();
            current_state.mouse_position = position;
            current_state.modifiers = modifiers;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::MouseMove(MouseMoveEvent {
                position,
                pressed_button: current_pressed,
                modifiers,
            }),
        );
    });
    canvas
        .add_event_listener_with_callback("pointermove", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_pointer_leave(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    pressed_button: Rc<RefCell<Option<MouseButton>>>,
    is_mac: bool,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::PointerEvent = event.unchecked_into();

        let position = pointer_position_in_element(&event);
        let modifiers = modifiers_from_mouse_event(&event, is_mac);
        let current_pressed = *pressed_button.borrow();

        {
            let mut current_state = state.borrow_mut();
            current_state.mouse_position = position;
            current_state.modifiers = modifiers;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::MouseExited(MouseExitEvent {
                position,
                pressed_button: current_pressed,
                modifiers,
            }),
        );
    });
    canvas
        .add_event_listener_with_callback("pointerleave", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_wheel(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    is_mac: bool,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::WheelEvent = event.unchecked_into();
        event.prevent_default();

        let mouse_event: &web_sys::MouseEvent = event.as_ref();
        let position = mouse_position_in_element(mouse_event);
        let modifiers = modifiers_from_wheel_event(mouse_event, is_mac);

        let delta_mode = event.delta_mode();
        let delta = if delta_mode == 1 {
            // DOM_DELTA_LINE
            // Negate: DOM positive = scroll down, GPUI positive = scroll up
            ScrollDelta::Lines(point(-event.delta_x() as f32, -event.delta_y() as f32))
        } else {
            // DOM_DELTA_PIXEL (0) or DOM_DELTA_PAGE (2, treat as pixels)
            // Negate: DOM positive = scroll down, GPUI positive = scroll up
            ScrollDelta::Pixels(point(
                px(-event.delta_x() as f32),
                px(-event.delta_y() as f32),
            ))
        };

        {
            let mut current_state = state.borrow_mut();
            current_state.modifiers = modifiers;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::ScrollWheel(ScrollWheelEvent {
                position,
                delta,
                modifiers,
                touch_phase: TouchPhase::Moved,
            }),
        );
    });
    canvas
        .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_context_menu(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::Event = event.unchecked_into();
        event.prevent_default();
    });
    canvas
        .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_drag_events(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
) {
    {
        let callbacks = Rc::clone(&callbacks);
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let event: web_sys::DragEvent = event.unchecked_into();
            event.prevent_default();

            let mouse_event: &web_sys::MouseEvent = event.as_ref();
            let position = mouse_position_in_element(mouse_event);

            {
                let mut current_state = state.borrow_mut();
                current_state.mouse_position = position;
            }

            dispatch_input(
                &callbacks,
                PlatformInput::FileDrop(FileDropEvent::Pending { position }),
            );
        });
        canvas
            .add_event_listener_with_callback("dragover", closure.as_ref().unchecked_ref())
            .ok();
        closures.push(closure);
    }

    {
        let callbacks = Rc::clone(&callbacks);
        let state = Rc::clone(&state);
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let event: web_sys::DragEvent = event.unchecked_into();
            event.prevent_default();

            let mouse_event: &web_sys::MouseEvent = event.as_ref();
            let position = mouse_position_in_element(mouse_event);

            {
                let mut current_state = state.borrow_mut();
                current_state.mouse_position = position;
            }

            let paths = extract_file_paths_from_drag(&event);

            dispatch_input(
                &callbacks,
                PlatformInput::FileDrop(FileDropEvent::Entered {
                    position,
                    paths: ExternalPaths(paths),
                }),
            );

            dispatch_input(
                &callbacks,
                PlatformInput::FileDrop(FileDropEvent::Submit { position }),
            );
        });
        canvas
            .add_event_listener_with_callback("drop", closure.as_ref().unchecked_ref())
            .ok();
        closures.push(closure);
    }

    {
        let callbacks = Rc::clone(&callbacks);
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |_event: JsValue| {
            dispatch_input(&callbacks, PlatformInput::FileDrop(FileDropEvent::Exited));
        });
        canvas
            .add_event_listener_with_callback("dragleave", closure.as_ref().unchecked_ref())
            .ok();
        closures.push(closure);
    }
}

fn register_key_down(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    is_mac: bool,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::KeyboardEvent = event.unchecked_into();

        let modifiers = modifiers_from_keyboard_event(&event, is_mac);
        let capslock = capslock_from_keyboard_event(&event);

        {
            let mut current_state = state.borrow_mut();
            current_state.modifiers = modifiers;
            current_state.capslock = capslock;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock,
            }),
        );

        let key = dom_key_to_gpui_key(&event);

        if is_modifier_only_key(&key) {
            return;
        }

        event.prevent_default();

        let is_held = event.repeat();

        let key_char = compute_key_char(&event, &key, &modifiers);

        let keystroke = Keystroke {
            modifiers,
            key,
            key_char,
        };

        dispatch_input(
            &callbacks,
            PlatformInput::KeyDown(KeyDownEvent {
                keystroke,
                is_held,
                prefer_character_input: false,
            }),
        );
    });
    canvas
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn register_key_up(
    canvas: &web_sys::HtmlCanvasElement,
    closures: &mut Vec<Closure<dyn FnMut(JsValue)>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    state: Rc<RefCell<WebWindowMutableState>>,
    is_mac: bool,
) {
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let event: web_sys::KeyboardEvent = event.unchecked_into();

        let modifiers = modifiers_from_keyboard_event(&event, is_mac);
        let capslock = capslock_from_keyboard_event(&event);

        {
            let mut current_state = state.borrow_mut();
            current_state.modifiers = modifiers;
            current_state.capslock = capslock;
        }

        dispatch_input(
            &callbacks,
            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock,
            }),
        );

        let key = dom_key_to_gpui_key(&event);

        if is_modifier_only_key(&key) {
            return;
        }

        event.prevent_default();

        let key_char = compute_key_char(&event, &key, &modifiers);

        let keystroke = Keystroke {
            modifiers,
            key,
            key_char,
        };

        dispatch_input(&callbacks, PlatformInput::KeyUp(KeyUpEvent { keystroke }));
    });
    canvas
        .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
        .ok();
    closures.push(closure);
}

fn dispatch_input(callbacks: &Rc<RefCell<WebWindowCallbacks>>, input: PlatformInput) {
    let mut borrowed = callbacks.borrow_mut();
    if let Some(ref mut callback) = borrowed.input {
        callback(input);
    }
}

fn dom_key_to_gpui_key(event: &web_sys::KeyboardEvent) -> String {
    let key = event.key();
    match key.as_str() {
        "Enter" => "enter".to_string(),
        "Backspace" => "backspace".to_string(),
        "Tab" => "tab".to_string(),
        "Escape" => "escape".to_string(),
        "Delete" => "delete".to_string(),
        " " => "space".to_string(),
        "ArrowLeft" => "left".to_string(),
        "ArrowRight" => "right".to_string(),
        "ArrowUp" => "up".to_string(),
        "ArrowDown" => "down".to_string(),
        "Home" => "home".to_string(),
        "End" => "end".to_string(),
        "PageUp" => "pageup".to_string(),
        "PageDown" => "pagedown".to_string(),
        "Insert" => "insert".to_string(),
        "Control" => "control".to_string(),
        "Alt" => "alt".to_string(),
        "Shift" => "shift".to_string(),
        "Meta" => "platform".to_string(),
        "CapsLock" => "capslock".to_string(),
        other => {
            if let Some(rest) = other.strip_prefix('F') {
                if let Ok(number) = rest.parse::<u8>() {
                    if (1..=35).contains(&number) {
                        return format!("f{number}");
                    }
                }
            }
            other.to_lowercase()
        }
    }
}

fn dom_mouse_button_to_gpui(button: i16) -> MouseButton {
    match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        3 => MouseButton::Navigate(NavigationDirection::Back),
        4 => MouseButton::Navigate(NavigationDirection::Forward),
        _ => MouseButton::Left,
    }
}

fn modifiers_from_keyboard_event(event: &web_sys::KeyboardEvent, _is_mac: bool) -> Modifiers {
    Modifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        platform: event.meta_key(),
        function: false,
    }
}

fn modifiers_from_mouse_event(event: &web_sys::PointerEvent, _is_mac: bool) -> Modifiers {
    let mouse_event: &web_sys::MouseEvent = event.as_ref();
    Modifiers {
        control: mouse_event.ctrl_key(),
        alt: mouse_event.alt_key(),
        shift: mouse_event.shift_key(),
        platform: mouse_event.meta_key(),
        function: false,
    }
}

fn modifiers_from_wheel_event(event: &web_sys::MouseEvent, _is_mac: bool) -> Modifiers {
    Modifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        platform: event.meta_key(),
        function: false,
    }
}

fn capslock_from_keyboard_event(event: &web_sys::KeyboardEvent) -> Capslock {
    Capslock {
        on: event.get_modifier_state("CapsLock"),
    }
}

fn is_mac_platform() -> bool {
    let Some(browser_window) = web_sys::window() else {
        return false;
    };
    let navigator = browser_window.navigator();

    #[allow(deprecated)]
    // navigator.platform() is deprecated but navigator.userAgentData is not widely available yet
    if let Ok(platform) = navigator.platform() {
        if platform.contains("Mac") {
            return true;
        }
    }

    if let Ok(user_agent) = navigator.user_agent() {
        return user_agent.contains("Mac");
    }

    false
}

fn is_modifier_only_key(key: &str) -> bool {
    matches!(key, "control" | "alt" | "shift" | "platform" | "capslock")
}

fn compute_key_char(
    event: &web_sys::KeyboardEvent,
    gpui_key: &str,
    modifiers: &Modifiers,
) -> Option<String> {
    if modifiers.platform || modifiers.control {
        return None;
    }

    if is_modifier_only_key(gpui_key) {
        return None;
    }

    if gpui_key == "space" {
        return Some(" ".to_string());
    }

    let raw_key = event.key();

    // Only produce key_char for printable characters (single char or known printable sequences)
    if raw_key.len() == 1 {
        return Some(raw_key);
    }

    // Multi-character key names like "Enter", "Backspace" etc. are not printable
    None
}

fn pointer_position_in_element(event: &web_sys::PointerEvent) -> Point<Pixels> {
    let mouse_event: &web_sys::MouseEvent = event.as_ref();
    mouse_position_in_element(mouse_event)
}

fn mouse_position_in_element(event: &web_sys::MouseEvent) -> Point<Pixels> {
    // offset_x/offset_y give position relative to the target element's padding edge
    point(px(event.offset_x() as f32), px(event.offset_y() as f32))
}

fn extract_file_paths_from_drag(
    event: &web_sys::DragEvent,
) -> smallvec::SmallVec<[std::path::PathBuf; 2]> {
    let mut paths = smallvec![];
    let Some(data_transfer) = event.data_transfer() else {
        return paths;
    };
    let file_list = data_transfer.files();
    let Some(files) = file_list else {
        return paths;
    };
    for index in 0..files.length() {
        if let Some(file) = files.get(index) {
            paths.push(std::path::PathBuf::from(file.name()));
        }
    }
    paths
}
