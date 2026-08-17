use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct GanttDiagramRenderModel {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default, rename = "dateFormat")]
    pub date_format: String,
    #[serde(default, rename = "axisFormat")]
    pub axis_format: String,
    #[serde(default, rename = "tickInterval")]
    pub tick_interval: Option<String>,
    #[serde(default, rename = "todayMarker")]
    pub today_marker: String,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub excludes: Vec<String>,
    #[serde(default, rename = "displayMode")]
    pub display_mode: String,
    #[serde(default, rename = "topAxis")]
    pub top_axis: bool,
    #[serde(default)]
    pub weekday: String,
    #[serde(default)]
    pub weekend: String,
    #[serde(default)]
    pub tasks: Vec<GanttRenderTask>,
}

impl GanttDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GanttRenderTask {
    pub id: String,
    pub task: String,
    pub section: String,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub done: bool,
    #[serde(default)]
    pub crit: bool,
    #[serde(default)]
    pub milestone: bool,
    #[serde(default)]
    pub vert: bool,
    #[serde(default)]
    pub order: i64,
    #[serde(rename = "startTime")]
    pub start_ms: i64,
    #[serde(rename = "endTime")]
    pub end_ms: i64,
    #[serde(default, rename = "renderEndTime")]
    pub render_end_ms: Option<i64>,
}

#[derive(Debug, Clone)]
pub(super) enum StartTimeRaw {
    PrevTaskEnd,
    GetStartDate { start_data: String },
}

#[derive(Debug, Clone)]
pub(super) struct RawTaskRaw {
    pub(super) data: String,
    pub(super) start_time: StartTimeRaw,
    pub(super) end_data: String,
}

#[derive(Debug, Clone)]
pub(super) struct RawTask {
    pub(super) section: String,
    pub(super) type_: String,
    pub(super) processed: bool,
    pub(super) manual_end_time: bool,
    pub(super) render_end_time: Option<DateTimeFixed>,
    pub(super) raw: RawTaskRaw,
    pub(super) task: String,
    pub(super) classes: Vec<String>,
    pub(super) id: String,
    pub(super) prev_task_id: Option<String>,
    pub(super) active: bool,
    pub(super) done: bool,
    pub(super) crit: bool,
    pub(super) milestone: bool,
    pub(super) vert: bool,
    pub(super) order: i64,
    pub(super) start_time: Option<DateTimeFixed>,
    pub(super) end_time: Option<DateTimeFixed>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(super) struct ClickEvent {
    function_name: String,
    function_args: Vec<String>,
    raw_function_args: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct GanttDb {
    pub(super) acc_title: String,
    pub(super) acc_descr: String,
    pub(super) diagram_title: String,

    pub(super) date_format: String,
    pub(super) axis_format: String,
    pub(super) tick_interval: Option<String>,
    pub(super) today_marker: String,
    pub(super) includes: Vec<String>,
    pub(super) excludes: Vec<String>,
    pub(super) links: HashMap<String, String>,
    pub(super) click_events: HashMap<String, ClickEvent>,

    pub(super) sections: Vec<String>,
    pub(super) current_section: String,
    pub(super) display_mode: String,

    pub(super) inclusive_end_dates: bool,
    pub(super) top_axis: bool,
    pub(super) weekday: String,
    pub(super) weekend: String,

    pub(super) raw_tasks: Vec<RawTask>,
    pub(super) task_index: HashMap<String, usize>,
    pub(super) task_cnt: i64,
    pub(super) last_task_id: Option<String>,
    pub(super) last_order: i64,

    pub(super) security_level: String,
}

impl GanttDb {
    pub(super) fn clear(&mut self) {
        *self = Self::default();
        self.weekday = "sunday".to_string();
        self.weekend = "saturday".to_string();
    }

    pub(super) fn set_security_level(&mut self, level: Option<&str>) {
        self.security_level = level.unwrap_or("strict").to_string();
    }

    pub(super) fn set_date_format(&mut self, txt: &str) {
        self.date_format = txt.to_string();
    }

    pub(super) fn enable_inclusive_end_dates(&mut self) {
        self.inclusive_end_dates = true;
    }

    pub(super) fn enable_top_axis(&mut self) {
        self.top_axis = true;
    }

    pub(super) fn set_axis_format(&mut self, txt: &str) {
        self.axis_format = txt.to_string();
    }

    pub(super) fn set_tick_interval(&mut self, txt: &str) {
        self.tick_interval = Some(txt.to_string());
    }

    pub(super) fn set_today_marker(&mut self, txt: &str) {
        self.today_marker = txt.to_string();
    }

    pub(super) fn set_includes(&mut self, txt: &str) {
        self.includes = split_list_lower(txt);
    }

    pub(super) fn set_excludes(&mut self, txt: &str) {
        self.excludes = split_list_lower(txt);
    }

    pub(super) fn set_weekday(&mut self, txt: &str) {
        self.weekday = txt.to_string();
    }

    pub(super) fn set_weekend(&mut self, txt: &str) {
        self.weekend = txt.to_string();
    }

    pub(super) fn set_diagram_title(&mut self, txt: &str) {
        self.diagram_title = txt.to_string();
    }

    pub(super) fn set_display_mode(&mut self, txt: &str) {
        self.display_mode = txt.to_string();
    }

    pub(super) fn set_acc_title(&mut self, txt: &str) {
        self.acc_title = txt.to_string();
    }

    pub(super) fn set_acc_descr(&mut self, txt: &str) {
        self.acc_descr = txt.to_string();
    }

    pub(super) fn add_section(&mut self, txt: &str) {
        self.current_section = txt.to_string();
        self.sections.push(txt.to_string());
    }

    pub(super) fn find_task_by_id(&self, id: &str) -> Option<&RawTask> {
        // Mermaid's upstream ganttDb uses a plain JS object (`taskDb`) for id → index mapping,
        // which makes `__proto__` non-addressable via `taskDb[id]` (prototype mutation). Mirror
        // that observable behavior for parity.
        if id == "__proto__" {
            return None;
        }
        let pos = self.task_index.get(id).copied()?;
        self.raw_tasks.get(pos)
    }

    pub(super) fn find_task_by_id_mut(&mut self, id: &str) -> Option<&mut RawTask> {
        if id == "__proto__" {
            return None;
        }
        let pos = self.task_index.get(id).copied()?;
        self.raw_tasks.get_mut(pos)
    }

    pub(super) fn set_class(&mut self, ids: &str, class_name: &str) {
        for id in ids.split(',') {
            let id = id.trim();
            let Some(task) = self.find_task_by_id_mut(id) else {
                continue;
            };
            task.classes.push(class_name.to_string());
        }
    }

    pub(super) fn set_link(&mut self, ids: &str, link_str: &str) {
        let mut link_str = link_str.to_string();
        if self.security_level != "loose" {
            link_str = utils::sanitize_url(&link_str);
        }
        for id in ids.split(',') {
            let id = id.trim();
            if self.find_task_by_id(id).is_some() {
                self.links.insert(id.to_string(), link_str.clone());
            }
        }
        self.set_class(ids, "clickable");
    }

    pub(super) fn set_click_event(
        &mut self,
        ids: &str,
        function_name: &str,
        function_args: Option<&str>,
    ) {
        if self.security_level == "loose" {
            for id in ids.split(',') {
                let id = id.trim();
                if self.find_task_by_id(id).is_some() {
                    let args = parse_callback_args(function_args).unwrap_or_default();
                    let args = if args.is_empty() {
                        vec![id.to_string()]
                    } else {
                        args
                    };
                    self.click_events.insert(
                        id.to_string(),
                        ClickEvent {
                            function_name: function_name.to_string(),
                            function_args: args,
                            raw_function_args: function_args.map(|s| s.to_string()),
                        },
                    );
                }
            }
        }
        self.set_class(ids, "clickable");
    }

    pub(super) fn add_task(&mut self, descr: &str, data: &str) {
        let prev_task_id = self.last_task_id.clone();
        let task_info = parse_task_data(&mut self.task_cnt, data);

        let raw_task = RawTask {
            section: self.current_section.clone(),
            type_: self.current_section.clone(),
            processed: false,
            manual_end_time: false,
            render_end_time: None,
            raw: RawTaskRaw {
                data: data.to_string(),
                start_time: task_info.start_time,
                end_data: task_info.end_data,
            },
            task: descr.to_string(),
            classes: Vec::new(),
            id: task_info.id.clone(),
            prev_task_id,
            active: task_info.active,
            done: task_info.done,
            crit: task_info.crit,
            milestone: task_info.milestone,
            vert: task_info.vert,
            order: self.last_order,
            start_time: None,
            end_time: None,
        };

        self.last_order += 1;
        let pos = self.raw_tasks.len();
        self.raw_tasks.push(raw_task);
        self.last_task_id = Some(task_info.id.clone());
        self.task_index.insert(task_info.id, pos);
    }

    fn compile_tasks(&mut self) -> Result<bool> {
        let mut all_processed = true;
        for i in 0..self.raw_tasks.len() {
            let processed = self.compile_task(i)?;
            all_processed = all_processed && processed;
        }
        Ok(all_processed)
    }

    fn compile_task(&mut self, pos: usize) -> Result<bool> {
        let start_spec = self.raw_tasks.get(pos).map(|t| t.raw.start_time.clone());
        let Some(start_spec) = start_spec else {
            return Ok(false);
        };

        match start_spec {
            StartTimeRaw::PrevTaskEnd => {
                let prev_id = self.raw_tasks[pos].prev_task_id.clone();
                if let Some(prev_id) = prev_id {
                    if let Some(prev_task) = self.find_task_by_id(&prev_id) {
                        self.raw_tasks[pos].start_time = prev_task.end_time;
                    }
                }
            }
            StartTimeRaw::GetStartDate { start_data } => {
                let start_time = get_start_date(self, &self.date_format, &start_data)?;
                if let Some(start_time) = start_time {
                    self.raw_tasks[pos].start_time = Some(start_time);
                }
            }
        }

        let Some(start_time) = self.raw_tasks[pos].start_time else {
            return Ok(false);
        };

        let end_data = self.raw_tasks[pos].raw.end_data.clone();
        let end_time = get_end_date(
            self,
            start_time,
            &self.date_format,
            &end_data,
            self.inclusive_end_dates,
        )?;
        self.raw_tasks[pos].end_time = end_time;
        self.raw_tasks[pos].processed = self.raw_tasks[pos].end_time.is_some();

        if self.raw_tasks[pos].processed {
            self.raw_tasks[pos].manual_end_time = is_strict_yyyy_mm_dd(&end_data);
            self.check_task_dates(pos)?;
        }

        Ok(self.raw_tasks[pos].processed)
    }

    fn check_task_dates(&mut self, pos: usize) -> Result<()> {
        if self.excludes.is_empty() || self.raw_tasks[pos].manual_end_time {
            return Ok(());
        }
        let Some(start_time) = self.raw_tasks[pos].start_time else {
            return Ok(());
        };
        let Some(end_time) = self.raw_tasks[pos].end_time else {
            return Ok(());
        };

        let Some(start_time) = add_days_local(start_time, 1) else {
            return Ok(());
        };
        let (fixed_end_time, render_end_time) =
            fix_task_dates(self, start_time, end_time, &self.date_format)?;
        self.raw_tasks[pos].end_time = Some(fixed_end_time);
        self.raw_tasks[pos].render_end_time = render_end_time;

        Ok(())
    }

    pub(super) fn get_tasks(&mut self) -> Result<Vec<RawTask>> {
        let mut all = self.compile_tasks()?;
        let max_depth = 10;
        let mut iters = 0;
        while !all && iters < max_depth {
            all = self.compile_tasks()?;
            iters += 1;
        }
        Ok(self.raw_tasks.clone())
    }
}

fn parse_callback_args(raw: Option<&str>) -> Option<Vec<String>> {
    let raw = raw?;
    let mut out: Vec<String> = Vec::new();

    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in raw.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                cur.push(ch);
            }
            ',' if !in_quotes => {
                out.push(cur);
                cur = String::new();
            }
            _ => cur.push(ch),
        }
    }
    out.push(cur);

    let out: Vec<String> = out
        .into_iter()
        .map(|s| {
            let mut item = s.trim().to_string();
            if item.starts_with('"') && item.ends_with('"') && item.len() >= 2 {
                item = item[1..item.len() - 1].to_string();
            }
            item
        })
        .collect();

    Some(out)
}

#[derive(Debug, Clone)]
pub(super) struct TaskInfo {
    id: String,
    start_time: StartTimeRaw,
    end_data: String,
    active: bool,
    done: bool,
    crit: bool,
    milestone: bool,
    vert: bool,
}

fn split_list_lower(txt: &str) -> Vec<String> {
    txt.to_lowercase()
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn parse_task_data(task_cnt: &mut i64, data_str: &str) -> TaskInfo {
    let ds = data_str.strip_prefix(':').unwrap_or(data_str);
    let mut data: Vec<String> = ds.split(',').map(|s| s.to_string()).collect();

    let mut active = false;
    let mut done = false;
    let mut crit = false;
    let mut milestone = false;
    let mut vert = false;

    let tags = ["active", "done", "crit", "milestone", "vert"];
    let mut match_found = true;
    while match_found && !data.is_empty() {
        match_found = false;
        for tag in tags {
            if data.first().is_some_and(|s| s.trim() == tag) {
                match tag {
                    "active" => active = true,
                    "done" => done = true,
                    "crit" => crit = true,
                    "milestone" => milestone = true,
                    "vert" => vert = true,
                    _ => {}
                }
                data.remove(0);
                match_found = true;
                break;
            }
        }
    }

    for d in &mut data {
        *d = d.trim().to_string();
    }

    let mut next_id = |id_str: Option<&str>| -> String {
        match id_str {
            Some(s) => s.to_string(),
            None => {
                *task_cnt += 1;
                format!("task{}", *task_cnt)
            }
        }
    };

    match data.len() {
        1 => TaskInfo {
            id: next_id(None),
            start_time: StartTimeRaw::PrevTaskEnd,
            end_data: data[0].clone(),
            active,
            done,
            crit,
            milestone,
            vert,
        },
        2 => TaskInfo {
            id: next_id(None),
            start_time: StartTimeRaw::GetStartDate {
                start_data: data[0].clone(),
            },
            end_data: data[1].clone(),
            active,
            done,
            crit,
            milestone,
            vert,
        },
        3 => TaskInfo {
            id: next_id(Some(&data[0])),
            start_time: StartTimeRaw::GetStartDate {
                start_data: data[1].clone(),
            },
            end_data: data[2].clone(),
            active,
            done,
            crit,
            milestone,
            vert,
        },
        _ => TaskInfo {
            id: next_id(None),
            start_time: StartTimeRaw::PrevTaskEnd,
            end_data: String::new(),
            active,
            done,
            crit,
            milestone,
            vert,
        },
    }
}
