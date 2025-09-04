use gpui::{IntoElement, ParentElement};
use ui::{List, ListBulletItem, prelude::*};

/// Centralized definitions for Zed AI plans
pub struct PlanDefinitions;

impl PlanDefinitions {
    pub const AI_DESCRIPTION: &'static str = "Zed offers a complete agentic experience, with robust editing and reviewing features to collaborate with AI.";

    pub fn free_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new("50 prompts with Claude models"))
            .child(ListBulletItem::new("2,000 accepted edit predictions"))
    }

    pub fn pro_trial(&self, period: bool) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new("150 prompts with Claude models"))
            .child(ListBulletItem::new(
                "Unlimited edit predictions with Zeta, our open-source model",
            ))
            .when(period, |this| {
                this.child(ListBulletItem::new(
                    "Try it out for 14 days for free, no credit card required",
                ))
            })
    }

    pub fn pro_plan(&self, price: bool) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new("500 prompts with Claude models"))
            .child(ListBulletItem::new(
                "Unlimited edit predictions with Zeta, our open-source model",
            ))
            .when(price, |this| {
                this.child(ListBulletItem::new("$20 USD per month"))
            })
    }
}
