use octocrab::models::timelines::TimelineEvent;

#[tokio::test]
async fn should_deserialize() {
    let _: Vec<TimelineEvent> =
        serde_json::from_str(include_str!("resources/issues_list_timeline_events.json")).unwrap();
}
