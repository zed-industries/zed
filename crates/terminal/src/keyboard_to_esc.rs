use gpui::KeyDownEvent;

pub fn to_esc_str(event: &KeyDownEvent) -> String {
    let key = event.keystroke.key.clone();
    let modifiers = (
        event.keystroke.alt,
        event.keystroke.cmd,
        event.keystroke.ctrl,
        event.keystroke.shift,
    );
    match (key.as_str(), modifiers) {
        //ctrl-l
        //shift-tab
        //alt-back
        //shift-back
        //shift + Home, end, page up, page down + NOT alt screen => We handle those
        //shift + Home, end, page up, page down + alt screen => Send escape sequence
        ("l", (false, false, true, false)) => "\x0c".to_string(),
        _ => event.input.clone().unwrap().clone(),
    }
}
