use gpui::{Entity, Render};
use ui::{ProgressBar, prelude::*};

#[derive(RegisterComponent)]
pub struct UserSpending {
    free_tier_current: u32,
    free_tier_cap: u32,
    over_tier_current: u32,
    over_tier_cap: u32,
    free_tier_progress: Entity<ProgressBar>,
    over_tier_progress: Entity<ProgressBar>,
}

impl UserSpending {
    pub fn new(
        free_tier_current: u32,
        free_tier_cap: u32,
        over_tier_current: u32,
        over_tier_cap: u32,
        cx: &mut App,
    ) -> Self {
        let free_tier_capped = free_tier_current == free_tier_cap;
        let free_tier_near_capped =
            free_tier_current as f32 / 100.0 >= free_tier_cap as f32 / 100.0 * 0.9;
        let over_tier_capped = over_tier_current == over_tier_cap;
        let over_tier_near_capped =
            over_tier_current as f32 / 100.0 >= over_tier_cap as f32 / 100.0 * 0.9;

        let free_tier_progress = cx.new(|cx| {
            ProgressBar::new(
                "free_tier",
                free_tier_current as f32,
                free_tier_cap as f32,
                cx,
            )
        });
        let over_tier_progress = cx.new(|cx| {
            ProgressBar::new(
                "over_tier",
                over_tier_current as f32,
                over_tier_cap as f32,
                cx,
            )
        });

        if free_tier_capped {
            free_tier_progress.update(cx, |progress_bar, cx| {
                progress_bar.fg_color(cx.theme().status().error);
            });
        } else if free_tier_near_capped {
            free_tier_progress.update(cx, |progress_bar, cx| {
                progress_bar.fg_color(cx.theme().status().warning);
            });
        }

        if over_tier_capped {
            over_tier_progress.update(cx, |progress_bar, cx| {
                progress_bar.fg_color(cx.theme().status().error);
            });
        } else if over_tier_near_capped {
            over_tier_progress.update(cx, |progress_bar, cx| {
                progress_bar.fg_color(cx.theme().status().warning);
            });
        }

        Self {
            free_tier_current,
            free_tier_cap,
            over_tier_current,
            over_tier_cap,
            free_tier_progress,
            over_tier_progress,
        }
    }
}

impl Render for UserSpending {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let formatted_free_tier = format!(
            "${} / ${}",
            self.free_tier_current as f32 / 100.0,
            self.free_tier_cap as f32 / 100.0
        );
        let formatted_over_tier = format!(
            "${} / ${}",
            self.over_tier_current as f32 / 100.0,
            self.over_tier_cap as f32 / 100.0
        );

        v_group()
            .elevation_2(cx)
            .py_1p5()
            .px_2p5()
            .w(px(360.))
            .child(
                v_flex()
                    .child(
                        v_flex()
                            .p_1p5()
                            .gap_0p5()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Free Tier Usage").size(LabelSize::Small))
                                    .child(
                                        Label::new(formatted_free_tier)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(self.free_tier_progress.clone()),
                    )
                    .child(
                        v_flex()
                            .p_1p5()
                            .gap_0p5()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Current Spending").size(LabelSize::Small))
                                    .child(
                                        Label::new(formatted_over_tier)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(self.over_tier_progress.clone()),
                    ),
            )
    }
}

impl Component for UserSpending {
    fn scope() -> ComponentScope {
        ComponentScope::None
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let new_user = cx.new(|cx| UserSpending::new(0, 2000, 0, 2000, cx));
        let free_capped = cx.new(|cx| UserSpending::new(2000, 2000, 0, 2000, cx));
        let free_near_capped = cx.new(|cx| UserSpending::new(1800, 2000, 0, 2000, cx));
        let over_near_capped = cx.new(|cx| UserSpending::new(2000, 2000, 1800, 2000, cx));
        let over_capped = cx.new(|cx| UserSpending::new(1000, 2000, 2000, 2000, cx));

        Some(
            v_flex()
                .gap_6()
                .p_4()
                .children(vec![example_group(vec![
                    single_example(
                        "New User",
                        div().size_full().child(new_user.clone()).into_any_element(),
                    ),
                    single_example(
                        "Free Tier Capped",
                        div()
                            .size_full()
                            .child(free_capped.clone())
                            .into_any_element(),
                    ),
                    single_example(
                        "Free Tier Near Capped",
                        div()
                            .size_full()
                            .child(free_near_capped.clone())
                            .into_any_element(),
                    ),
                    single_example(
                        "Over Tier Near Capped",
                        div()
                            .size_full()
                            .child(over_near_capped.clone())
                            .into_any_element(),
                    ),
                    single_example(
                        "Over Tier Capped",
                        div()
                            .size_full()
                            .child(over_capped.clone())
                            .into_any_element(),
                    ),
                ])])
                .into_any_element(),
        )
    }
}
