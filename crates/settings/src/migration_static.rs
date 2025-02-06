use std::sync::LazyLock;

use collections::HashMap;

#[rustfmt::skip]
pub static TRANSFORM_ARRAY: LazyLock<HashMap<(&str, &str), &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        // activate
        (("workspace::ActivatePaneInDirection", "Up"), "workspace::ActivatePaneUp"),
        (("workspace::ActivatePaneInDirection", "Down"), "workspace::ActivatePaneDown"),
        (("workspace::ActivatePaneInDirection", "Left"), "workspace::ActivatePaneLeft"),
        (("workspace::ActivatePaneInDirection", "Right"), "workspace::ActivatePaneRight"),
        // swap
        (("workspace::SwapPaneInDirection", "Up"), "workspace::SwapPaneUp"),
        (("workspace::SwapPaneInDirection", "Down"), "workspace::SwapPaneDown"),
        (("workspace::SwapPaneInDirection", "Left"), "workspace::SwapPaneLeft"),
        (("workspace::SwapPaneInDirection", "Right"), "workspace::SwapPaneRight"),
        // menu
        (("app_menu::NavigateApplicationMenuInDirection", "Left"), "app_menu::ActivateMenuLeft"),
        (("app_menu::NavigateApplicationMenuInDirection", "Right"), "app_menu::ActivateMenuRight"),
        // vim push
        (("vim::PushOperator", "Change"), "vim::PushChange"),
        (("vim::PushOperator", "Delete"), "vim::PushDelete"),
        (("vim::PushOperator", "Yank"), "vim::PushYank"),
        (("vim::PushOperator", "Replace"), "vim::PushReplace"),
        (("vim::PushOperator", "AddSurrounds"), "vim::PushAddSurrounds"),
        (("vim::PushOperator", "DeleteSurrounds"), "vim::PushDeleteSurrounds"),
        (("vim::PushOperator", "Mark"), "vim::PushMark"),
        (("vim::PushOperator", "Indent"), "vim::PushIndent"),
        (("vim::PushOperator", "Outdent"), "vim::PushOutdent"),
        (("vim::PushOperator", "AutoIndent"), "vim::PushAutoIndent"),
        (("vim::PushOperator", "Rewrap"), "vim::PushRewrap"),
        (("vim::PushOperator", "ShellCommand"), "vim::PushShellCommand"),
        (("vim::PushOperator", "Lowercase"), "vim::PushLowercase"),
        (("vim::PushOperator", "Uppercase"), "vim::PushUppercase"),
        (("vim::PushOperator", "OppositeCase"), "vim::PushOppositeCase"),
        (("vim::PushOperator", "Register"), "vim::PushRegister"),
        (("vim::PushOperator", "RecordRegister"), "vim::PushRecordRegister"),
        (("vim::PushOperator", "ReplayRegister"), "vim::PushReplayRegister"),
        (("vim::PushOperator", "ToggleComments"), "vim::PushToggleComments"),
        // vim switch
        (("vim::SwitchMode", "Normal"), "vim::SwitchToNormalMode"),
        (("vim::SwitchMode", "Insert"), "vim::SwitchToInsertMode"),
        (("vim::SwitchMode", "Replace"), "vim::SwitchToReplaceMode"),
        (("vim::SwitchMode", "Visual"), "vim::SwitchToVisualMode"),
        (("vim::SwitchMode", "VisualLine"), "vim::SwitchToVisualLineMode"),
        (("vim::SwitchMode", "VisualBlock"), "vim::SwitchToVisualBlockMode"),
        (("vim::SwitchMode", "HelixNormal"), "vim::SwitchToHelixNormalMode"),
        // vim resize
        (("vim::ResizePane", "Widen"), "vim::ResizePaneRight"),
        (("vim::ResizePane", "Narrow"), "vim::ResizePaneLeft"),
        (("vim::ResizePane", "Shorten"), "vim::ResizePaneDown"),
        (("vim::ResizePane", "Lengthen"), "vim::ResizePaneUp"),
    ])
});

pub static UNWRAP_OBJECTS: LazyLock<HashMap<&str, Vec<(&str, &str)>>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "editor::FoldAtLevel",
            vec![("level", "editor::FoldAtLevel")],
        ),
        (
            "vim::PushOperator",
            vec![
                ("Object", "vim::PushObject"),
                ("FindForward", "vim::PushFindForward"),
                ("FindBackward", "vim::PushFindBackward"),
                ("Sneak", "vim::PushSneak"),
                ("SneakBackward", "vim::PushSneakBackward"),
                ("ChangeSurrounds", "vim::PushChangeSurrounds"),
                ("Jump", "vim::PushJump"),
                ("Digraph", "vim::PushDigraph"),
                ("Literal", "vim::PushLiteral"),
            ],
        ),
    ])
});

#[rustfmt::skip]
pub static STRING_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("inline_completion::ToggleMenu", "edit_prediction::ToggleMenu"),
        ("editor::NextInlineCompletion", "editor::NextEditPrediction"),
        ("editor::PreviousInlineCompletion", "editor::PreviousEditPrediction"),
        ("editor::AcceptPartialInlineCompletion", "editor::AcceptPartialEditPrediction"),
        ("editor::ShowInlineCompletion", "editor::ShowEditPrediction"),
        ("editor::AcceptInlineCompletion", "editor::AcceptEditPrediction"),
        ("editor::ToggleInlineCompletions", "editor::ToggleEditPrediction"),
    ])
});

pub static CONTEXT_REPLACE: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| HashMap::from_iter([("inline_completion", "edit_prediction")]));

#[rustfmt::skip]
pub static SETTINGS_STRING_REPLACE: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("show_inline_completions_in_menu", "show_edit_predictions_in_menu"),
        ("show_inline_completions", "show_edit_predictions"),
        ("inline_completions_disabled_in", "edit_predictions_disabled_in"),
        ("inline_completions", "edit_predictions")
    ])
});

#[rustfmt::skip]
pub static SETTINGS_NESTED_STRING_REPLACE: LazyLock<HashMap<&'static str, (&'static str, &'static str)>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("features", ("inline_completion_provider", "edit_prediction_provider"))
    ])
});
