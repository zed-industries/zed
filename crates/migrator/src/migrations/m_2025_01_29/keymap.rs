use collections::HashMap;
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::{
    KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN, KEYMAP_ACTION_ARRAY_PATTERN,
    KEYMAP_ACTION_STRING_PATTERN, KEYMAP_CONTEXT_PATTERN,
};

pub const KEYMAP_PATTERNS: MigrationPatterns = &[
    (
        KEYMAP_ACTION_ARRAY_PATTERN,
        replace_array_with_single_string,
    ),
    (
        KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN,
        replace_action_argument_object_with_single_value,
    ),
    (KEYMAP_ACTION_STRING_PATTERN, replace_string_action),
    (KEYMAP_CONTEXT_PATTERN, rename_context_key),
];

fn replace_array_with_single_string(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let argument_ix = query.capture_index_for_name("argument")?;

    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;
    let argument = contents.get(
        mat.nodes_for_capture_index(argument_ix)
            .next()?
            .byte_range(),
    )?;

    let replacement = TRANSFORM_ARRAY.get(&(action_name, argument))?;
    let replacement_as_string = format!("\"{replacement}\"");
    let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();

    Some((range_to_replace, replacement_as_string))
}

static TRANSFORM_ARRAY: LazyLock<HashMap<(&str, &str), &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        // activate
        (
            ("workspace::ActivatePaneInDirection", "Up"),
            "workspace::ActivatePaneUp",
        ),
        (
            ("workspace::ActivatePaneInDirection", "Down"),
            "workspace::ActivatePaneDown",
        ),
        (
            ("workspace::ActivatePaneInDirection", "Left"),
            "workspace::ActivatePaneLeft",
        ),
        (
            ("workspace::ActivatePaneInDirection", "Right"),
            "workspace::ActivatePaneRight",
        ),
        // swap
        (
            ("workspace::SwapPaneInDirection", "Up"),
            "workspace::SwapPaneUp",
        ),
        (
            ("workspace::SwapPaneInDirection", "Down"),
            "workspace::SwapPaneDown",
        ),
        (
            ("workspace::SwapPaneInDirection", "Left"),
            "workspace::SwapPaneLeft",
        ),
        (
            ("workspace::SwapPaneInDirection", "Right"),
            "workspace::SwapPaneRight",
        ),
        // menu
        (
            ("app_menu::NavigateApplicationMenuInDirection", "Left"),
            "app_menu::ActivateMenuLeft",
        ),
        (
            ("app_menu::NavigateApplicationMenuInDirection", "Right"),
            "app_menu::ActivateMenuRight",
        ),
        // vim push
        (("vim::PushOperator", "Change"), "vim::PushChange"),
        (("vim::PushOperator", "Delete"), "vim::PushDelete"),
        (("vim::PushOperator", "Yank"), "vim::PushYank"),
        (("vim::PushOperator", "Replace"), "vim::PushReplace"),
        (
            ("vim::PushOperator", "DeleteSurrounds"),
            "vim::PushDeleteSurrounds",
        ),
        (("vim::PushOperator", "Mark"), "vim::PushMark"),
        (("vim::PushOperator", "Indent"), "vim::PushIndent"),
        (("vim::PushOperator", "Outdent"), "vim::PushOutdent"),
        (("vim::PushOperator", "AutoIndent"), "vim::PushAutoIndent"),
        (("vim::PushOperator", "Rewrap"), "vim::PushRewrap"),
        (
            ("vim::PushOperator", "ShellCommand"),
            "vim::PushShellCommand",
        ),
        (("vim::PushOperator", "Lowercase"), "vim::PushLowercase"),
        (("vim::PushOperator", "Uppercase"), "vim::PushUppercase"),
        (
            ("vim::PushOperator", "OppositeCase"),
            "vim::PushOppositeCase",
        ),
        (("vim::PushOperator", "Register"), "vim::PushRegister"),
        (
            ("vim::PushOperator", "RecordRegister"),
            "vim::PushRecordRegister",
        ),
        (
            ("vim::PushOperator", "ReplayRegister"),
            "vim::PushReplayRegister",
        ),
        (
            ("vim::PushOperator", "ReplaceWithRegister"),
            "vim::PushReplaceWithRegister",
        ),
        (
            ("vim::PushOperator", "ToggleComments"),
            "vim::PushToggleComments",
        ),
        // vim switch
        (("vim::SwitchMode", "Normal"), "vim::SwitchToNormalMode"),
        (("vim::SwitchMode", "Insert"), "vim::SwitchToInsertMode"),
        (("vim::SwitchMode", "Replace"), "vim::SwitchToReplaceMode"),
        (("vim::SwitchMode", "Visual"), "vim::SwitchToVisualMode"),
        (
            ("vim::SwitchMode", "VisualLine"),
            "vim::SwitchToVisualLineMode",
        ),
        (
            ("vim::SwitchMode", "VisualBlock"),
            "vim::SwitchToVisualBlockMode",
        ),
        (
            ("vim::SwitchMode", "HelixNormal"),
            "vim::SwitchToHelixNormalMode",
        ),
        // vim resize
        (("vim::ResizePane", "Widen"), "vim::ResizePaneRight"),
        (("vim::ResizePane", "Narrow"), "vim::ResizePaneLeft"),
        (("vim::ResizePane", "Shorten"), "vim::ResizePaneDown"),
        (("vim::ResizePane", "Lengthen"), "vim::ResizePaneUp"),
        // fold at level
        (("editor::FoldAtLevel", "1"), "editor::FoldAtLevel1"),
        (("editor::FoldAtLevel", "2"), "editor::FoldAtLevel2"),
        (("editor::FoldAtLevel", "3"), "editor::FoldAtLevel3"),
        (("editor::FoldAtLevel", "4"), "editor::FoldAtLevel4"),
        (("editor::FoldAtLevel", "5"), "editor::FoldAtLevel5"),
        (("editor::FoldAtLevel", "6"), "editor::FoldAtLevel6"),
        (("editor::FoldAtLevel", "7"), "editor::FoldAtLevel7"),
        (("editor::FoldAtLevel", "8"), "editor::FoldAtLevel8"),
        (("editor::FoldAtLevel", "9"), "editor::FoldAtLevel9"),
    ])
});

/// [ "editor::FoldAtLevel", { "level": 1 } ] -> [ "editor::FoldAtLevel", 1 ]
fn replace_action_argument_object_with_single_value(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let argument_key_ix = query.capture_index_for_name("argument_key")?;
    let argument_value_ix = query.capture_index_for_name("argument_value")?;

    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;
    let argument_key = contents.get(
        mat.nodes_for_capture_index(argument_key_ix)
            .next()?
            .byte_range(),
    )?;
    let argument_value = contents.get(
        mat.nodes_for_capture_index(argument_value_ix)
            .next()?
            .byte_range(),
    )?;

    let new_action_name = UNWRAP_OBJECTS.get(&action_name)?.get(&argument_key)?;

    let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();
    let replacement = format!("[\"{}\", {}]", new_action_name, argument_value);
    Some((range_to_replace, replacement))
}

/// "ctrl-k ctrl-1": [ "editor::PushOperator", { "Object": {} } ] -> [ "editor::vim::PushObject", {} ]
static UNWRAP_OBJECTS: LazyLock<HashMap<&str, HashMap<&str, &str>>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "editor::FoldAtLevel",
            HashMap::from_iter([("level", "editor::FoldAtLevel")]),
        ),
        (
            "vim::PushOperator",
            HashMap::from_iter([
                ("Object", "vim::PushObject"),
                ("FindForward", "vim::PushFindForward"),
                ("FindBackward", "vim::PushFindBackward"),
                ("Sneak", "vim::PushSneak"),
                ("SneakBackward", "vim::PushSneakBackward"),
                ("AddSurrounds", "vim::PushAddSurrounds"),
                ("ChangeSurrounds", "vim::PushChangeSurrounds"),
                ("Jump", "vim::PushJump"),
                ("Digraph", "vim::PushDigraph"),
                ("Literal", "vim::PushLiteral"),
            ]),
        ),
    ])
});

fn replace_string_action(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let action_name_node = mat.nodes_for_capture_index(action_name_ix).next()?;
    let action_name_range = action_name_node.byte_range();
    let action_name = contents.get(action_name_range.clone())?;

    if let Some(new_action_name) = STRING_REPLACE.get(&action_name) {
        return Some((action_name_range, new_action_name.to_string()));
    }

    None
}

/// "ctrl-k ctrl-1": "inline_completion::ToggleMenu" -> "edit_prediction::ToggleMenu"
static STRING_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "inline_completion::ToggleMenu",
            "edit_prediction::ToggleMenu",
        ),
        ("editor::NextInlineCompletion", "editor::NextEditPrediction"),
        (
            "editor::PreviousInlineCompletion",
            "editor::PreviousEditPrediction",
        ),
        (
            "editor::AcceptPartialInlineCompletion",
            "editor::AcceptPartialEditPrediction",
        ),
        ("editor::ShowInlineCompletion", "editor::ShowEditPrediction"),
        (
            "editor::AcceptInlineCompletion",
            "editor::AcceptEditPrediction",
        ),
        (
            "editor::ToggleInlineCompletions",
            "editor::ToggleEditPrediction",
        ),
    ])
});

fn rename_context_key(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let context_predicate_ix = query.capture_index_for_name("context_predicate")?;
    let context_predicate_range = mat
        .nodes_for_capture_index(context_predicate_ix)
        .next()?
        .byte_range();
    let old_predicate = contents.get(context_predicate_range.clone())?.to_string();
    let mut new_predicate = old_predicate.to_string();
    for (old_key, new_key) in CONTEXT_REPLACE.iter() {
        new_predicate = new_predicate.replace(old_key, new_key);
    }
    if new_predicate != old_predicate {
        Some((context_predicate_range, new_predicate))
    } else {
        None
    }
}

/// "context": "Editor && inline_completion && !showing_completions" -> "Editor && edit_prediction && !showing_completions"
pub static CONTEXT_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("inline_completion", "edit_prediction"),
        (
            "inline_completion_requires_modifier",
            "edit_prediction_requires_modifier",
        ),
    ])
});
