use gpui::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_action(quit);
        cx.set_menus(vec![Menu {
            name: "editable_text",
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| EditableText {
                text: cx.new_model(|_| "Hello, world!".into()),
                poem: cx.new_model(|_| {
                    r#"Two roads diverged in a yellow wood,
And sorry I could not travel both
And be one traveler, long I stood
And looked down one as far as I could
To where it bent in the undergrowth;

Then took the other, as just as fair,
And having perhaps the better claim,
Because it was grassy and wanted wear;
Though as for that the passing there
Had worn them really about the same,

And both that morning equally lay
In leaves no step had trodden black.
Oh, I kept the first for another day!
Yet knowing how way leads on to way,
I doubted if I should ever come back.

I shall be telling this with a sigh
Somewhere ages and ages hence:
Two roads diverged in a wood, and I—
I took the one less traveled by,
And that has made all the difference."#
                        .into()
                }),
                before_focus_handle: cx.focus_handle(),
                after_focus_handle: cx.focus_handle(),
            })
        });
    });
}

actions!(editable_text, [Quit]);

fn quit(_: &Quit, cx: &mut AppContext) {
    cx.quit();
}

struct EditableText {
    text: Model<String>,
    poem: Model<String>,
    before_focus_handle: FocusHandle,
    after_focus_handle: FocusHandle,
}

impl Render for EditableText {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .p_3()
            .items_start()
            .text_2xl()
            .text_color(rgb(0xffffff))
            .flex()
            .flex_row()
            .justify_around()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .track_focus(&self.before_focus_handle)
                            .text_sm()
                            .text_color(rgb(0x000000))
                            .focus(|this| this.border_1().border_color(rgb(0xffffff)))
                            .child("Before (this is focusable, not editable)"),
                    )
                    .child(
                        editable_text(ElementId::View(self.text.entity_id()), self.text.clone())
                            .on_focus_next({
                                let after_focus_handle = self.after_focus_handle.clone();

                                move |cx| {
                                    cx.focus(&after_focus_handle);
                                }
                            })
                            .on_focus_prev({
                                let before_focus_handle = self.before_focus_handle.clone();

                                move |cx| {
                                    cx.focus(&before_focus_handle);
                                }
                            })
                            .on_enter({
                                let text = self.text.clone();

                                move |cx| {
                                    let prompt_task = cx.prompt(
                                        PromptLevel::Info,
                                        "Enter was pressed",
                                        Some(&format!("The value is: `{}`", text.read(cx))),
                                        &[],
                                    );

                                    cx.background_executor()
                                        .spawn(async {
                                            prompt_task.await.unwrap();
                                        })
                                        .detach();
                                }
                            }),
                    )
                    .child(
                        div()
                            .track_focus(&self.after_focus_handle)
                            .text_sm()
                            .text_color(rgb(0x000000))
                            .focus(|this| this.border_1().border_color(rgb(0xffffff)))
                            .child("After (this is focusable, not editable)"),
                    ),
            )
            .child(
                div()
                    .child(div().text_xl().child(String::from("The Road Not Taken")))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xc0c0c0))
                            .py_2()
                            .child(String::from("By Robert Frost")),
                    )
                    .child(
                        div().text_base().justify_center().items_center().child(
                            editable_text(
                                ElementId::View(self.poem.entity_id()),
                                self.poem.clone(),
                            )
                            .multiline()
                            .on_focus_prev({
                                let before_focus_handle = self.after_focus_handle.clone();

                                move |cx| {
                                    cx.focus(&before_focus_handle);
                                }
                            })
                            .on_enter({
                                let poem = self.poem.clone();

                                move |cx| {
                                    let prompt_task = cx.prompt(
                                        PromptLevel::Info,
                                        "Enter was pressed",
                                        Some(&format!("The value is: `{}`", poem.read(cx))),
                                        &[],
                                    );

                                    cx.background_executor()
                                        .spawn(async {
                                            prompt_task.await.unwrap();
                                        })
                                        .detach();
                                }
                            }),
                        ),
                    ),
            )
    }
}
