// As we run the unit eval, we discover problems
// Some of which can be solved alorithmically and tested deterministically

// This prompt change helped:
//
// - `<old_text>` cannot be empty

// But the model still wasn't perfect: So we then wrote a deterministic test to
// gracefully handle the edge case:
#[gpui::test(iterations = 100)]
async fn test_empty_old_text(cx: &mut TestAppContext, mut rng: StdRng) {
    let agent = init_test(cx).await;
    let buffer = cx.new(|cx| {
        Buffer::local(
            indoc! {"
                abc
                def
                ghi
            "},
            cx,
        )
    });
    let (apply, _events) = agent.edit(
        buffer.clone(),
        String::new(),
        &LanguageModelRequest::default(),
        &mut cx.to_async(),
    );
    cx.run_until_parked();

    simulate_llm_output(
        &agent,
        indoc! {"
            <old_text></old_text>
            <new_text>jkl</new_text>
            <old_text>def</old_text>
            <new_text>DEF</new_text>
        "},
        &mut rng,
        cx,
    );
    apply.await.unwrap();

    pretty_assertions::assert_eq!(
        buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
        indoc! {"
            abc
            DEF
            ghi
        "}
    );
}
