use super::*;
use crate::{Engine, ParseOptions, RenderSemanticModel};
use chrono::{Local, NaiveDate, TimeZone};
use futures::executor::block_on;

fn parse(text: &str) -> Value {
    let engine = Engine::new();
    block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap()
        .model
}

fn local_ms(y: i32, m0: u32, d: u32, h: u32, min: u32, s: u32) -> i64 {
    let m = m0 + 1;
    let local = Local
        .with_ymd_and_hms(y, m, d, h, min, s)
        .single()
        .or_else(|| Local.with_ymd_and_hms(y, m, d, h, min, s).earliest())
        .unwrap();
    local.fixed_offset().timestamp_millis()
}

#[test]
fn gantt_render_model_uses_fixed_today_for_missing_year_dates() {
    let engine = Engine::new()
        .with_fixed_today(Some(
            NaiveDate::from_ymd_opt(2026, 2, 15).expect("valid fixed today"),
        ))
        .with_fixed_local_offset_minutes(Some(0));
    let parsed = block_on(engine.parse_diagram_for_render_model(
        r#"
gantt
dateFormat MM-DD
section Demo
Missing year: id1,03-01,1d
Missing ref: id2,after missing,1d
"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let RenderSemanticModel::Gantt(model) = parsed.model else {
        panic!("expected Gantt render model");
    };
    let task = |id: &str| {
        model
            .tasks
            .iter()
            .find(|task| task.id == id)
            .unwrap_or_else(|| panic!("missing Gantt task {id}"))
    };

    assert_eq!(task("id1").start_ms, 1_772_323_200_000);
    assert_eq!(task("id2").start_ms, 1_771_113_600_000);
}

#[test]
fn gantt_fixed_dates() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section testa1
test1: id1,2013-01-01,2013-01-12
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(t0["id"].as_str().unwrap(), "id1");
    assert_eq!(t0["task"].as_str().unwrap(), "test1");
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 0)
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 12, 0, 0, 0)
    );
}

#[test]
fn gantt_duration_units() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section testa1
test1: id1,2013-01-01,2h
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 0)
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 2, 0, 0)
    );
}

#[test]
fn gantt_last_day_of_month_degrades_without_epoch_unwraps() {
    assert_eq!(last_day_of_month(2024, 2), 29);
    assert_eq!(last_day_of_month(2023, 2), 28);
    assert_eq!(last_day_of_month(2024, 13), 31);
    assert_eq!(last_day_of_month(i32::MAX, 12), 31);
}

#[test]
fn gantt_relative_after_id() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
test1: id1,2013-01-01,2w
test2: id2,after id1,1d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 15, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 16, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_until_id() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
task1: id1,2013-01-01,until id3
section sec2
task3: id3,2013-02-01,2d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2013, 1, 1, 0, 0, 0)
    );
}

#[test]
fn gantt_excludes_weekends_and_specific_days() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
excludes weekends 2019-02-06,friday
section weekends skip test
test1: id1,2019-02-01,1d
test2: id2,after id1,2d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[0]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 4, 0, 0, 0)
    );
    assert_eq!(
        tasks[0]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 2, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 4, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 7, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 6, 0, 0, 0)
    );
}

#[test]
fn gantt_inclusive_end_dates_adds_one_day_for_strict_dates() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
inclusiveEndDates
test1: id1,2019-02-01,1d
test2: id2,2019-02-01,2019-02-03
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 2, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 4, 0, 0, 0)
    );
    assert!(tasks[1]["renderEndTime"].is_null());
    assert!(tasks[1]["manualEndTime"].as_bool().unwrap());
}

#[test]
fn gantt_rejects_ridiculous_years() {
    let engine = Engine::new();
    let err = block_on(engine.parse_diagram(
        r#"
gantt
dateFormat YYYYMMDD
test1: id1,202304,1d
"#,
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(err.to_string().contains("Invalid date:202304"));
}

#[test]
fn gantt_js_date_fallback_year_bounds_match_upstream_guardrail() {
    let dt = parse_js_date_fallback("10000").unwrap();
    assert_eq!(dt.year(), 10000);

    let err = parse_js_date_fallback("10001").unwrap_err();
    assert!(err.to_string().contains("Invalid date:10001"));
}

#[test]
fn gantt_js_date_fallback_parses_mdy_hm_strings_like_v8() {
    use chrono::{Datelike, Timelike};

    // Mermaid's gantt parser falls back to `new Date(str)`; in V8, the string
    // `08-08-09-01:00` parses as local time `2009-08-08 01:00`.
    let dt = parse_js_date_fallback("08-08-09-01:00").unwrap();
    assert_eq!(dt.year(), 2009);
    assert_eq!(dt.month(), 8);
    assert_eq!(dt.day(), 8);
    assert_eq!(dt.hour(), 1);
    assert_eq!(dt.minute(), 0);
}

#[test]
fn gantt_js_date_fallback_rejects_invalid_calendar_dates() {
    let err = parse_js_date_fallback("2019-02-30").unwrap_err();
    assert!(err.to_string().contains("Invalid date:2019-02-30"));
}

#[test]
fn gantt_parse_duration_matches_upstream_examples() {
    assert_eq!(parse_duration("1d"), (1.0, "d".to_string()));
    assert!(parse_duration("1f").0.is_nan());
    assert_eq!(parse_duration("0.1s"), (0.1, "s".to_string()));
    assert_eq!(parse_duration("1ms"), (1.0, "ms".to_string()));
    assert_eq!(parse_duration(" 2m "), (2.0, "m".to_string()));
    assert_eq!(parse_duration("01M"), (1.0, "M".to_string()));

    for invalid in [".1s", "1.s", "1MS", "1msx", "1 ms", ""] {
        let (value, unit) = parse_duration(invalid);
        assert!(value.is_nan(), "expected invalid duration for {invalid:?}");
        assert_eq!(unit, "ms");
    }
}

#[test]
fn gantt_weekends_can_start_on_friday() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
excludes weekends
weekend friday
section friday-saturday weekends skip test
test1: id1,2024-02-28,3d
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2024, 1, 28, 0, 0, 0)
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        local_ms(2024, 2, 4, 0, 0, 0)
    );
}

#[test]
fn gantt_seconds_only_date_format_is_accepted() {
    let model = parse(
        r#"
gantt
dateFormat ss
section Network Request
RTT: rtt, 0, 20
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0]["task"].as_str().unwrap(), "RTT");
    assert_eq!(tasks[0]["id"].as_str().unwrap(), "rtt");
}

#[test]
fn gantt_date_year_typos_fall_back_to_js_date_parsing() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section Vacation
London Trip 1: 2024-12-01, 7d
London Trip 2: 202-12-01, 7d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 2);

    let ms0 = tasks[0]["startTime"].as_i64().unwrap();
    let ms1 = tasks[1]["startTime"].as_i64().unwrap();
    let dt0 = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms0).unwrap();
    let dt1 = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms1).unwrap();
    assert_eq!(dt0.year(), 2024);
    assert_eq!(dt1.year(), 202);
}

#[test]
fn gantt_preserves_task_creation_order() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section A
Completed task: done, des1, 2014-01-06,2014-01-08
Active task: active, des2, 2014-01-09, 3d
section B
Future task: des3, after des2, 5d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks[0]["order"].as_i64().unwrap(), 0);
    assert_eq!(tasks[1]["order"].as_i64().unwrap(), 1);
    assert_eq!(tasks[2]["order"].as_i64().unwrap(), 2);
    assert!(tasks[0]["done"].as_bool().unwrap());
    assert!(tasks[1]["active"].as_bool().unwrap());
    assert_eq!(tasks[0]["id"].as_str().unwrap(), "des1");
    assert_eq!(tasks[1]["id"].as_str().unwrap(), "des2");
    assert_eq!(tasks[2]["id"].as_str().unwrap(), "des3");
}

#[test]
fn gantt_date_format_custom_separators_parse_strict() {
    let model = parse(
        r#"
gantt
dateFormat YYYY/MM/DD
section testa1
test1: id1,2013/01/01,2013/01/12
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(t0["id"].as_str().unwrap(), "id1");
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 0)
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 12, 0, 0, 0)
    );
}

#[test]
fn dayjs_strict_parses_month_names_and_offsets() {
    let dt = parse_dayjs_like_strict("YYYY-MMM-DD", "2013-Jan-02").unwrap();
    assert_eq!(dt.timestamp_millis(), local_ms(2013, 0, 2, 0, 0, 0));

    let dt = parse_dayjs_like_strict("YYYY-MM-DDTHH:mm:ssZ", "2013-01-01T00:00:00+00:00").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2013, 1, 1, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );

    let dt = parse_dayjs_like_strict("YYYY-MM-DDTHH:mm:ssZ", "2013-01-01T00:00:00+08:00").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2012, 12, 31, 16, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );

    let dt = parse_dayjs_like_strict("YYYY-MM-DDTHH:mm:ssZZ", "2013-01-01T00:00:00+0800").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2012, 12, 31, 16, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );

    let dt = parse_dayjs_like_strict("YYYY-MM-DDTHH:mm:ssZ", "2013-01-01T00:00:00Z").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2013, 1, 1, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );
}

#[test]
fn gantt_js_date_fallback_parses_iso_date_only_as_utc() {
    let model = parse(
        r#"
gantt
dateFormat YYYYMMDD
section A
test1: id1,2013-01-01,1d
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        chrono::Utc
            .with_ymd_and_hms(2013, 1, 1, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        chrono::Utc
            .with_ymd_and_hms(2013, 1, 2, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );
}

#[test]
fn gantt_js_date_fallback_parses_iso_datetime_without_tz_as_local() {
    let model = parse(
        r#"
gantt
dateFormat YYYYMMDD
section A
test1: id1,2013-01-01T00:00:00,1d
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 0)
    );
}

#[test]
fn gantt_js_date_fallback_parses_timezone_offsets_with_or_without_colon() {
    let dt = parse_js_date_fallback("2013-01-01T00:00:00+0800").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2012, 12, 31, 16, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );

    let dt = parse_js_date_fallback("2013-01-01T00:00:00+08:00").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2012, 12, 31, 16, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );

    let dt = parse_js_date_fallback("2013-01-01T00:00:00Z").unwrap();
    assert_eq!(
        dt.timestamp_millis(),
        chrono::Utc
            .with_ymd_and_hms(2013, 1, 1, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );
}

#[test]
fn gantt_js_date_fallback_parses_slash_dates_as_local() {
    let dt = parse_js_date_fallback("2013/01/01").unwrap();
    assert_eq!(dt.timestamp_millis(), local_ms(2013, 0, 1, 0, 0, 0));

    let dt = parse_js_date_fallback("2013/01/01 00:00:00").unwrap();
    assert_eq!(dt.timestamp_millis(), local_ms(2013, 0, 1, 0, 0, 0));
}

#[test]
fn gantt_excludes_weekday_names_use_full_names() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
excludes friday
section A
test1: id1,2019-02-07,2d
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 7, 0, 0, 0)
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 10, 0, 0, 0)
    );
    assert_eq!(
        t0["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 10, 0, 0, 0)
    );
}

#[test]
fn gantt_click_call_is_ignored_unless_security_level_loose() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section A
task: id1, 2013-01-01, 1d
click id1 call myFn("a,b", c)
"#,
    );
    assert!(model["clickEvents"].as_object().unwrap().is_empty());
}

#[test]
fn gantt_click_call_parses_args_and_defaults_to_id() {
    let model = parse(
        r#"
%%{init: {"securityLevel":"loose"}}%%
gantt
dateFormat YYYY-MM-DD
section A
task: id1, 2013-01-01, 1d
task2: id2, 2013-01-02, 1d
click id2 call myFn("a,b", c)
click id1 call myFn2()
"#,
    );
    let ev1 = &model["clickEvents"]["id1"];
    assert_eq!(ev1["function_name"].as_str().unwrap(), "myFn2");
    assert_eq!(ev1["function_args"][0].as_str().unwrap(), "id1");
    assert!(ev1["raw_function_args"].is_null());

    let ev2 = &model["clickEvents"]["id2"];
    assert_eq!(ev2["function_name"].as_str().unwrap(), "myFn");
    assert_eq!(ev2["function_args"][0].as_str().unwrap(), "a,b");
    assert_eq!(ev2["function_args"][1].as_str().unwrap(), "c");
    assert_eq!(ev2["raw_function_args"].as_str().unwrap(), "\"a,b\", c");
}

#[test]
fn gantt_common_db_sanitizes_title_and_accessibility_fields() {
    let model = parse(
        r#"
gantt
title <script>alert(1)</script><b>ok</b>
accTitle: <script>alert(1)</script><b>AT</b>
accDescr { <script>alert(1)</script>line1
    line2
}
"#,
    );
    assert_eq!(model["title"], json!("<b>ok</b>"));
    assert_eq!(model["accTitle"], json!("<b>AT</b>"));
    assert_eq!(model["accDescr"], json!("line1\nline2"));
}

#[test]
fn gantt_duration_minutes_and_seconds_match_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section testa1
test1: id1,2013-01-01,2m
test2: id2,2013-01-01,2s
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 2, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 2)
    );
}

#[test]
fn gantt_fixed_dates_without_id_match_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section testa1
test1: 2013-01-01,2013-01-12
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(t0["id"].as_str().unwrap(), "task1");
    assert_eq!(
        t0["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 0)
    );
    assert_eq!(
        t0["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 12, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_after_auto_task_ids_works_with_custom_date_format() {
    let model = parse(
        r#"
gantt
dateFormat DD.MM.YYYY
section Section
Task 1 :done, 07.01.2024, 08.02.2024
Task 2 :active, after task1, 20d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks[0]["id"].as_str().unwrap(), "task1");
    assert_eq!(tasks[1]["id"].as_str().unwrap(), "task2");
    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2024, 1, 8, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_refs_work_across_sections_like_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
test1: id1,2013-01-01,2w
test2: id2,after id3,1d
section sec2
test3: id3,after id1,2d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 17, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 18, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_after_multiple_ids_uses_latest_end_like_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
task1: id1,after id2 id3 id4,1d
task2: id2,2013-01-01,1d
task3: id3,2013-02-01,3d
task4: id4,2013-02-01,2d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2013, 1, 5, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_until_multiple_ids_uses_earliest_start_like_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
task1: id1,2013-01-01,until id2 id3 id4
task2: id2,2013-01-11,1d
task3: id3,2013-02-10,1d
task4: id4,2013-02-12,1d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 11, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_refs_accept_ascii_word_and_hyphen_ids_like_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
base: ref_1-2,2013-01-01,2d
next: id2,after ref_1-2,1d
limit: until_1-2,2013-02-01,1d
bounded: id3,2013-01-01,until until_1-2
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2013, 0, 3, 0, 0, 0)
    );
    assert_eq!(
        tasks[3]["endTime"].as_i64().unwrap(),
        local_ms(2013, 1, 1, 0, 0, 0)
    );
}

#[test]
fn gantt_relative_ref_ids_preserve_source_regex_space_backtracking() {
    assert_eq!(relative_ref_ids("after id_1-2", "after"), Some("id_1-2"));
    assert_eq!(relative_ref_ids("after  #", "after"), Some(" "));
    assert_eq!(relative_ref_ids("after \t #", "after"), Some(" "));
    assert_eq!(relative_ref_ids("after #", "after"), None);
    assert_eq!(relative_ref_ids("After id1", "after"), None);
}

#[test]
fn gantt_relative_keywords_are_case_sensitive_like_upstream() {
    let engine = Engine::new();
    let err = block_on(engine.parse_diagram(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
base: id1,2013-01-01,2d
next: id2,After id1,1d
"#,
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(err.to_string().contains("Invalid date:After id1"));

    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section sec1
limit: id2,2013-02-01,1d
bounded: id1,2013-01-01,Until id2
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2013, 0, 1, 0, 0, 0)
    );
}

#[test]
fn gantt_timestamp_formats_x_and_x_support_signed_and_seconds() {
    let model = parse(
        r#"
gantt
dateFormat x
section T
t1: id1,-1,1ms
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(t0["startTime"].as_i64().unwrap(), -1);
    assert_eq!(t0["endTime"].as_i64().unwrap(), 0);

    let model = parse(
        r#"
gantt
dateFormat X
section T
t1: id1,20,1s
"#,
    );
    let t0 = &model["tasks"][0];
    assert_eq!(t0["startTime"].as_i64().unwrap(), 20);
    assert_eq!(t0["endTime"].as_i64().unwrap(), 1_020);
}

#[test]
fn gantt_ignore_weekends_matches_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
excludes weekends 2019-02-06,friday
section weekends skip test
test1: id1,2019-02-01,1d
test2: id2,after id1,2d
test3: id3,after id2,7d
test4: id4,2019-02-01,2019-02-20
test5: id5,after id4,1d
section full ending task on last day
test6: id6,2019-02-13,2d
test7: id7,after id6,1d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 7);

    assert_eq!(
        tasks[0]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 4, 0, 0, 0)
    );
    assert_eq!(
        tasks[0]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 2, 0, 0, 0)
    );
    assert_eq!(tasks[0]["id"].as_str().unwrap(), "id1");
    assert_eq!(tasks[0]["task"].as_str().unwrap(), "test1");

    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 4, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 7, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 6, 0, 0, 0)
    );
    assert_eq!(tasks[1]["id"].as_str().unwrap(), "id2");
    assert_eq!(tasks[1]["task"].as_str().unwrap(), "test2");

    assert_eq!(
        tasks[2]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 7, 0, 0, 0)
    );
    assert_eq!(
        tasks[2]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 20, 0, 0, 0)
    );
    assert_eq!(
        tasks[2]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 20, 0, 0, 0)
    );
    assert_eq!(tasks[2]["id"].as_str().unwrap(), "id3");
    assert_eq!(tasks[2]["task"].as_str().unwrap(), "test3");

    assert_eq!(
        tasks[3]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 1, 0, 0, 0)
    );
    assert_eq!(
        tasks[3]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 20, 0, 0, 0)
    );
    assert!(tasks[3]["renderEndTime"].is_null());
    assert!(tasks[3]["manualEndTime"].as_bool().unwrap());
    assert_eq!(tasks[3]["id"].as_str().unwrap(), "id4");
    assert_eq!(tasks[3]["task"].as_str().unwrap(), "test4");

    assert_eq!(
        tasks[4]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 20, 0, 0, 0)
    );
    assert_eq!(
        tasks[4]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 21, 0, 0, 0)
    );
    assert_eq!(
        tasks[4]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 21, 0, 0, 0)
    );
    assert_eq!(tasks[4]["id"].as_str().unwrap(), "id5");
    assert_eq!(tasks[4]["task"].as_str().unwrap(), "test5");

    assert_eq!(
        tasks[5]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 13, 0, 0, 0)
    );
    assert_eq!(
        tasks[5]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 18, 0, 0, 0)
    );
    assert_eq!(
        tasks[5]["renderEndTime"].as_i64().unwrap(),
        local_ms(2019, 1, 15, 0, 0, 0)
    );
    assert_eq!(tasks[5]["id"].as_str().unwrap(), "id6");
    assert_eq!(tasks[5]["task"].as_str().unwrap(), "test6");

    assert_eq!(
        tasks[6]["startTime"].as_i64().unwrap(),
        local_ms(2019, 1, 18, 0, 0, 0)
    );
    assert_eq!(
        tasks[6]["endTime"].as_i64().unwrap(),
        local_ms(2019, 1, 19, 0, 0, 0)
    );
    assert_eq!(tasks[6]["id"].as_str().unwrap(), "id7");
    assert_eq!(tasks[6]["task"].as_str().unwrap(), "test7");
}

#[test]
fn gantt_maintains_task_creation_order_matches_upstream_sample() {
    let model = parse(
        r#"
gantt
accTitle: Project Execution
dateFormat YYYY-MM-DD
section section A section
Completed task: done,    des1, 2014-01-06,2014-01-08
Active task: active,  des2, 2014-01-09, 3d
Future task: des3, after des2, 5d
Future task2: des4, after des3, 5d

section section Critical tasks
Completed task in the critical line: crit, done, 2014-01-06,24h
Implement parser and jison: crit, done, after des1, 2d
Create tests for parser: crit, active, 3d
Future task in critical line: crit, 5d
Create tests for renderer: 2d
Add to mermaid: 1d

section section Documentation
Describe gantt syntax: active, a1, after des1, 3d
Add gantt diagram to demo page: after a1  , 20h
Add another diagram to demo page: doc1, after a1  , 48h

section section Last section
Describe gantt syntax: after doc1, 3d
Add gantt diagram to demo page: 20h
Add another diagram to demo page: 48h
"#,
    );

    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 16);

    for (i, t) in tasks.iter().enumerate() {
        assert_eq!(t["order"].as_i64().unwrap(), i as i64);
    }

    assert_eq!(tasks[0]["id"].as_str().unwrap(), "des1");
    assert_eq!(tasks[0]["task"].as_str().unwrap(), "Completed task");
    assert_eq!(
        tasks[0]["startTime"].as_i64().unwrap(),
        local_ms(2014, 0, 6, 0, 0, 0)
    );
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2014, 0, 8, 0, 0, 0)
    );

    assert_eq!(tasks[1]["id"].as_str().unwrap(), "des2");
    assert_eq!(tasks[1]["task"].as_str().unwrap(), "Active task");
    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2014, 0, 9, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2014, 0, 12, 0, 0, 0)
    );

    assert_eq!(
        tasks[11]["task"].as_str().unwrap(),
        "Add gantt diagram to demo page"
    );
    assert_eq!(
        tasks[11]["startTime"].as_i64().unwrap(),
        local_ms(2014, 0, 11, 0, 0, 0)
    );
    assert_eq!(
        tasks[11]["endTime"].as_i64().unwrap(),
        local_ms(2014, 0, 11, 20, 0, 0)
    );

    assert_eq!(
        tasks[14]["task"].as_str().unwrap(),
        "Add gantt diagram to demo page"
    );
    assert_eq!(
        tasks[14]["startTime"].as_i64().unwrap(),
        local_ms(2014, 0, 16, 0, 0, 0)
    );
    assert_eq!(
        tasks[14]["endTime"].as_i64().unwrap(),
        local_ms(2014, 0, 16, 20, 0, 0)
    );

    assert_eq!(
        tasks[15]["task"].as_str().unwrap(),
        "Add another diagram to demo page"
    );
    assert_eq!(
        tasks[15]["startTime"].as_i64().unwrap(),
        local_ms(2014, 0, 16, 20, 0, 0)
    );
    assert_eq!(
        tasks[15]["endTime"].as_i64().unwrap(),
        local_ms(2014, 0, 18, 20, 0, 0)
    );
}

#[test]
fn gantt_end_date_on_31st_matches_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section Task endTime is on the 31st day of the month
test1: id1,2019-09-30,11d
test2: id2,after id1,20d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 2);

    assert_eq!(
        tasks[0]["startTime"].as_i64().unwrap(),
        local_ms(2019, 8, 30, 0, 0, 0)
    );
    assert_eq!(
        tasks[0]["endTime"].as_i64().unwrap(),
        local_ms(2019, 9, 11, 0, 0, 0)
    );
    assert_eq!(tasks[0]["id"].as_str().unwrap(), "id1");
    assert_eq!(tasks[0]["task"].as_str().unwrap(), "test1");

    assert_eq!(
        tasks[1]["startTime"].as_i64().unwrap(),
        local_ms(2019, 9, 11, 0, 0, 0)
    );
    assert_eq!(
        tasks[1]["endTime"].as_i64().unwrap(),
        local_ms(2019, 9, 31, 0, 0, 0)
    );
    assert!(tasks[1]["renderEndTime"].is_null());
    assert_eq!(tasks[1]["id"].as_str().unwrap(), "id2");
    assert_eq!(tasks[1]["task"].as_str().unwrap(), "test2");
}

#[test]
fn gantt_today_marker_is_stored() {
    let model = parse(
        r#"
gantt
todayMarker off
"#,
    );
    assert_eq!(model["todayMarker"].as_str().unwrap(), "off");

    let model = parse(
        r#"
gantt
todayMarker stoke:stroke-width:5px,stroke:#00f,opacity:0.5
"#,
    );
    assert_eq!(
        model["todayMarker"].as_str().unwrap(),
        "stoke:stroke-width:5px,stroke:#00f,opacity:0.5"
    );
}

#[test]
fn gantt_section_allows_hash_character() {
    let model = parse(
        r#"
gantt
section A #1
test: id1,2013-01-01,1d
"#,
    );
    assert_eq!(model["sections"][0].as_str().unwrap(), "A #1");
}

#[test]
fn gantt_weekday_rejects_unknown_values() {
    let engine = Engine::new();
    let err = block_on(engine.parse_diagram(
        r#"
gantt
weekday foo
"#,
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(err.to_string().contains("invalid weekday"));
}

#[test]
fn gantt_weekend_rejects_unknown_values() {
    let engine = Engine::new();
    let err = block_on(engine.parse_diagram(
        r#"
gantt
weekend monday
"#,
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(err.to_string().contains("invalid weekend"));
}

#[test]
fn gantt_after_missing_id_defaults_to_today_midnight_like_upstream() {
    let model = parse(
        r#"
gantt
dateFormat YYYY-MM-DD
section testa1
test1: id1,2013-01-01,2w
test2: id2,after missing,1d
"#,
    );
    let tasks = model["tasks"].as_array().unwrap();
    let expected = today_midnight_local().timestamp_millis();
    assert_eq!(tasks[1]["startTime"].as_i64().unwrap(), expected);
}
