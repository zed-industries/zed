use gpui::{IntoElement, ParentElement};
use ui::{List, ListBulletItem, prelude::*};

/// Centralized definitions for Zed AI plans
pub struct PlanDefinitions;

impl PlanDefinitions {
    pub const AI_DESCRIPTION: &'static str = "Zed offers a complete agentic experience, with robust editing and reviewing features to collaborate with AI.";

    pub fn free_plan(&self, is_v2: bool) -> impl IntoElement {
        if is_v2 {
            List::new()
                .child(ListBulletItem::new("2,000 accepted edit predictions"))
                .child(ListBulletItem::new(
                    "Unlimited prompts with your AI API keys",
                ))
                .child(ListBulletItem::new(
                    "Unlimited use of external agents like Claude Code",
                ))
        } else {
            List::new()
                .child(ListBulletItem::new("50 prompts with Claude models"))
                .child(ListBulletItem::new("2,000 accepted edit predictions"))
        }
    }

    pub fn pro_trial(&self, is_v2: bool, period: bool) -> impl IntoElement {
        if is_v2 {
            List::new()
                .child(ListBulletItem::new("Unlimited edit predictions"))
                .child(ListBulletItem::new("$20 of tokens"))
                .when(period, |this| {
                    this.child(ListBulletItem::new(
                        "Try it out for 14 days, no credit card required",
                    ))
                })
        } else {
            List::new()
                .child(ListBulletItem::new("150 prompts with Claude models"))
                .child(ListBulletItem::new(
                    "Unlimited edit predictions with Zeta, our open-source model",
                ))
                .when(period, |this| {
                    this.child(ListBulletItem::new(
                        "Try it out for 14 days, no credit card required",
                    ))
                })
        }
    }

    pub fn pro_plan(&self, is_v2: bool, price: bool) -> impl IntoElement {
        if is_v2 {
            List::new()
                .child(ListBulletItem::new("Unlimited edit predictions"))
                .child(ListBulletItem::new("$5 of tokens"))
                .child(ListBulletItem::new("Usage-based billing beyond $5"))
        } else {
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
}
