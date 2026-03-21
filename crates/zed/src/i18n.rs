use std::collections::HashMap;
use std::sync::RwLock;

static CURRENT_LOCALE: RwLock<&'static str> = RwLock::new("en");

static EN_TRANSLATIONS: &[(&str, &str)] = &[
    ("app.name", "Zed"),
    ("app.about", "About Zed"),
    ("app.check_for_updates", "Check for Updates"),
    ("app.quit", "Quit Zed"),
    ("app.hide", "Hide Zed"),
    ("app.hide_others", "Hide Others"),
    ("app.show_all", "Show All"),
    ("menu.file", "File"),
    ("menu.edit", "Edit"),
    ("menu.view", "View"),
    ("menu.go", "Go"),
    ("menu.run", "Run"),
    ("menu.window", "Window"),
    ("menu.help", "Help"),
    ("menu.selection", "Selection"),
    ("file.new", "New"),
    ("file.new_window", "New Window"),
    ("file.open_file", "Open File..."),
    ("file.open_folder", "Open Folder..."),
    ("file.open_recent", "Open Recent..."),
    ("file.open_remote", "Open Remote..."),
    ("file.add_folder_to_project", "Add Folder to Project…"),
    ("file.save", "Save"),
    ("file.save_as", "Save As…"),
    ("file.save_all", "Save All"),
    ("file.close_editor", "Close Editor"),
    ("file.close_project", "Close Project"),
    ("file.close_window", "Close Window"),
    ("edit.undo", "Undo"),
    ("edit.redo", "Redo"),
    ("edit.cut", "Cut"),
    ("edit.copy", "Copy"),
    ("edit.copy_and_trim", "Copy and Trim"),
    ("edit.paste", "Paste"),
    ("edit.find", "Find"),
    ("edit.find_in_project", "Find in Project"),
    ("edit.toggle_line_comment", "Toggle Line Comment"),
    ("view.zoom_in", "Zoom In"),
    ("view.zoom_out", "Zoom Out"),
    ("view.reset_zoom", "Reset Zoom"),
    ("view.reset_all_zoom", "Reset All Zoom"),
    ("view.toggle_left_dock", "Toggle Left Dock"),
    ("view.toggle_right_dock", "Toggle Right Dock"),
    ("view.toggle_bottom_dock", "Toggle Bottom Dock"),
    ("view.toggle_all_docks", "Toggle All Docks"),
    ("view.split_up", "Split Up"),
    ("view.split_down", "Split Down"),
    ("view.split_left", "Split Left"),
    ("view.split_right", "Split Right"),
    ("view.project_panel", "Project Panel"),
    ("view.outline_panel", "Outline Panel"),
    ("view.collab_panel", "Collab Panel"),
    ("view.terminal_panel", "Terminal Panel"),
    ("view.debugger_panel", "Debugger Panel"),
    ("view.diagnostics", "Diagnostics"),
    ("view.toggle_gpu_inspector", "Toggle GPUI Inspector"),
    ("go.back", "Back"),
    ("go.forward", "Forward"),
    ("go.command_palette", "Command Palette..."),
    ("go.go_to_file", "Go to File..."),
    ("go.go_to_symbol", "Go to Symbol in Editor..."),
    ("go.go_to_line", "Go to Line/Column..."),
    ("go.go_to_definition", "Go to Definition"),
    ("go.go_to_declaration", "Go to Declaration"),
    ("go.go_to_type_definition", "Go to Type Definition"),
    ("go.find_all_references", "Find All References"),
    ("go.next_problem", "Next Problem"),
    ("go.previous_problem", "Previous Problem"),
    ("run.spawn_task", "Spawn Task"),
    ("run.start_debugger", "Start Debugger"),
    ("run.edit_tasks", "Edit tasks.json..."),
    ("run.edit_debug", "Edit debug.json..."),
    ("run.continue", "Continue"),
    ("run.step_over", "Step Over"),
    ("run.step_into", "Step Into"),
    ("run.step_out", "Step Out"),
    ("run.toggle_breakpoint", "Toggle Breakpoint"),
    ("run.edit_breakpoint", "Edit Breakpoint"),
    ("run.clear_all_breakpoints", "Clear All Breakpoints"),
    ("window.minimize", "Minimize"),
    ("window.zoom", "Zoom"),
    ("help.view_release_notes", "View Release Notes Locally"),
    ("help.view_telemetry", "View Telemetry"),
    ("help.view_licenses", "View Dependency Licenses"),
    ("help.show_welcome", "Show Welcome"),
    ("help.file_bug_report", "File Bug Report..."),
    ("help.request_feature", "Request Feature..."),
    ("help.email_us", "Email Us..."),
    ("help.documentation", "Documentation"),
    ("help.zed_repository", "Zed Repository"),
    ("help.zed_twitter", "Zed Twitter"),
    ("help.join_the_team", "Join the Team"),
    ("settings.open_settings", "Open Settings"),
    ("settings.open_settings_file", "Open Settings File"),
    ("settings.open_project_settings", "Open Project Settings"),
    ("settings.open_project_settings_file", "Open Project Settings File"),
    ("settings.open_default_settings", "Open Default Settings"),
    ("settings.open_keymap", "Open Keymap"),
    ("settings.open_keymap_file", "Open Keymap File"),
    ("settings.open_default_key_bindings", "Open Default Key Bindings"),
    ("settings.select_theme", "Select Theme..."),
    ("settings.select_icon_theme", "Select Icon Theme..."),
    ("selection.select_all", "Select All"),
    ("selection.expand_selection", "Expand Selection"),
    ("selection.shrink_selection", "Shrink Selection"),
    ("selection.select_next_sibling", "Select Next Sibling"),
    ("selection.select_previous_sibling", "Select Previous Sibling"),
    ("selection.add_cursor_above", "Add Cursor Above"),
    ("selection.add_cursor_below", "Add Cursor Below"),
    ("selection.select_next_occurrence", "Select Next Occurrence"),
    ("selection.select_previous_occurrence", "Select Previous Occurrence"),
    ("selection.select_all_occurrences", "Select All Occurrences"),
    ("selection.move_line_up", "Move Line Up"),
    ("selection.move_line_down", "Move Line Down"),
    ("selection.duplicate_selection", "Duplicate Selection"),
    ("editor_layout", "Editor Layout"),
    ("services", "Services"),
    ("extensions", "Extensions"),
    ("install_cli", "Install CLI"),
];

static ZH_CN_TRANSLATIONS: &[(&str, &str)] = &[
    ("app.name", "Zed"),
    ("app.about", "关于 Zed"),
    ("app.check_for_updates", "检查更新"),
    ("app.quit", "退出 Zed"),
    ("app.hide", "隐藏 Zed"),
    ("app.hide_others", "隐藏其他"),
    ("app.show_all", "显示全部"),
    ("menu.file", "文件"),
    ("menu.edit", "编辑"),
    ("menu.view", "视图"),
    ("menu.go", "跳转"),
    ("menu.run", "运行"),
    ("menu.window", "窗口"),
    ("menu.help", "帮助"),
    ("menu.selection", "选择"),
    ("file.new", "新建"),
    ("file.new_window", "新建窗口"),
    ("file.open_file", "打开文件..."),
    ("file.open_folder", "打开文件夹..."),
    ("file.open_recent", "打开最近..."),
    ("file.open_remote", "打开远程..."),
    ("file.add_folder_to_project", "添加文件夹到项目..."),
    ("file.save", "保存"),
    ("file.save_as", "另存为..."),
    ("file.save_all", "全部保存"),
    ("file.close_editor", "关闭编辑器"),
    ("file.close_project", "关闭项目"),
    ("file.close_window", "关闭窗口"),
    ("edit.undo", "撤销"),
    ("edit.redo", "重做"),
    ("edit.cut", "剪切"),
    ("edit.copy", "复制"),
    ("edit.copy_and_trim", "复制并去除空白"),
    ("edit.paste", "粘贴"),
    ("edit.find", "查找"),
    ("edit.find_in_project", "在项目中查找"),
    ("edit.toggle_line_comment", "切换行注释"),
    ("view.zoom_in", "放大"),
    ("view.zoom_out", "缩小"),
    ("view.reset_zoom", "重置缩放"),
    ("view.reset_all_zoom", "重置所有缩放"),
    ("view.toggle_left_dock", "切换左侧停靠栏"),
    ("view.toggle_right_dock", "切换右侧停靠栏"),
    ("view.toggle_bottom_dock", "切换底部停靠栏"),
    ("view.toggle_all_docks", "切换所有停靠栏"),
    ("view.split_up", "向上拆分"),
    ("view.split_down", "向下拆分"),
    ("view.split_left", "向左拆分"),
    ("view.split_right", "向右拆分"),
    ("view.project_panel", "项目面板"),
    ("view.outline_panel", "大纲面板"),
    ("view.collab_panel", "协作面板"),
    ("view.terminal_panel", "终端面板"),
    ("view.debugger_panel", "调试器面板"),
    ("view.diagnostics", "诊断"),
    ("view.toggle_gpu_inspector", "切换 GPUI 检查器"),
    ("go.back", "后退"),
    ("go.forward", "前进"),
    ("go.command_palette", "命令面板..."),
    ("go.go_to_file", "转到文件..."),
    ("go.go_to_symbol", "转到编辑器中的符号..."),
    ("go.go_to_line", "转到行/列..."),
    ("go.go_to_definition", "转到定义"),
    ("go.go_to_declaration", "转到声明"),
    ("go.go_to_type_definition", "转到类型定义"),
    ("go.find_all_references", "查找所有引用"),
    ("go.next_problem", "下一个问题"),
    ("go.previous_problem", "上一个问题"),
    ("run.spawn_task", "生成任务"),
    ("run.start_debugger", "启动调试器"),
    ("run.edit_tasks", "编辑 tasks.json..."),
    ("run.edit_debug", "编辑 debug.json..."),
    ("run.continue", "继续"),
    ("run.step_over", "单步跳过"),
    ("run.step_into", "单步进入"),
    ("run.step_out", "单步退出"),
    ("run.toggle_breakpoint", "切换断点"),
    ("run.edit_breakpoint", "编辑断点"),
    ("run.clear_all_breakpoints", "清除所有断点"),
    ("window.minimize", "最小化"),
    ("window.zoom", "缩放"),
    ("help.view_release_notes", "本地查看发布说明"),
    ("help.view_telemetry", "查看遥测数据"),
    ("help.view_licenses", "查看依赖许可证"),
    ("help.show_welcome", "显示欢迎页"),
    ("help.file_bug_report", "报告错误..."),
    ("help.request_feature", "请求功能..."),
    ("help.email_us", "给我们发邮件..."),
    ("help.documentation", "文档"),
    ("help.zed_repository", "Zed 代码仓库"),
    ("help.zed_twitter", "Zed Twitter"),
    ("help.join_the_team", "加入团队"),
    ("settings.open_settings", "打开设置"),
    ("settings.open_settings_file", "打开设置文件"),
    ("settings.open_project_settings", "打开项目设置"),
    ("settings.open_project_settings_file", "打开项目设置文件"),
    ("settings.open_default_settings", "打开默认设置"),
    ("settings.open_keymap", "打开快捷键配置"),
    ("settings.open_keymap_file", "打开快捷键配置文件"),
    ("settings.open_default_key_bindings", "打开默认快捷键绑定"),
    ("settings.select_theme", "选择主题..."),
    ("settings.select_icon_theme", "选择图标主题..."),
    ("selection.select_all", "全选"),
    ("selection.expand_selection", "扩大选择"),
    ("selection.shrink_selection", "缩小选择"),
    ("selection.select_next_sibling", "选择下一个同级元素"),
    ("selection.select_previous_sibling", "选择上一个同级元素"),
    ("selection.add_cursor_above", "在上面添加光标"),
    ("selection.add_cursor_below", "在下面添加光标"),
    ("selection.select_next_occurrence", "选择下一个匹配项"),
    ("selection.select_previous_occurrence", "选择上一个匹配项"),
    ("selection.select_all_occurrences", "选择所有匹配项"),
    ("selection.move_line_up", "上移行"),
    ("selection.move_line_down", "下移行"),
    ("selection.duplicate_selection", "复制选择"),
    ("editor_layout", "编辑器布局"),
    ("services", "服务"),
    ("extensions", "扩展"),
    ("install_cli", "安装 CLI"),
];

fn build_translations_map() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut m = HashMap::new();
    m.insert(
        "en",
        EN_TRANSLATIONS.iter().copied().collect(),
    );
    m.insert(
        "zh-CN",
        ZH_CN_TRANSLATIONS.iter().copied().collect(),
    );
    m
}

static TRANSLATIONS_MAP: RwLock<Option<HashMap<&'static str, HashMap<&'static str, &'static str>>>> =
    RwLock::new(None);

fn get_translations_map(
) -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let guard = TRANSLATIONS_MAP.read().unwrap();
    if let Some(map) = guard.as_ref() {
        return map.clone();
    }
    drop(guard);

    let mut write_guard = TRANSLATIONS_MAP.write().unwrap();
    if write_guard.is_none() {
        *write_guard = Some(build_translations_map());
    }
    write_guard.as_ref().unwrap().clone()
}

pub fn get_locale() -> String {
    CURRENT_LOCALE.read().unwrap().to_string()
}

pub fn set_locale(locale: &str) {
    if locale == "en" || locale == "zh-CN" {
        *CURRENT_LOCALE.write().unwrap() = locale;
    }
}

pub fn t(key: &str) -> String {
    let locale = get_locale();
    let map = get_translations_map();
    map.get(locale.as_str())
        .and_then(|m| m.get(key))
        .copied()
        .unwrap_or_else(|| {
            map.get("en")
                .and_then(|m| m.get(key))
                .copied()
                .unwrap_or(key)
        })
        .to_string()
}

pub fn available_locales() -> Vec<&'static str> {
    vec!["en", "zh-CN"]
}

pub fn detect_system_locale() -> &'static str {
    let lang = std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .or_else(|_| std::env::var("LC_MESSAGES"))
        .unwrap_or_default();

    let lang_lower = lang.to_lowercase();
    if lang_lower.contains("zh") {
        "zh-CN"
    } else {
        "en"
    }
}

pub fn init_with_system_locale() {
    let system_locale = detect_system_locale();
    set_locale(system_locale);
}

pub fn init_with_locale(locale: &str) {
    if locale == "en" || locale == "zh-CN" {
        set_locale(locale);
    } else {
        init_with_system_locale();
    }
}