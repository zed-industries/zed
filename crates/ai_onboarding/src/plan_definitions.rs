use gpui::{IntoElement, ParentElement};
use i18n::t;
use ui::{List, ListBulletItem, prelude::*};

/// Centralized definitions for Zed AI plans
pub struct PlanDefinitions;

impl PlanDefinitions {
    pub fn free_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("2,000 accepted edit predictions")))
            .child(ListBulletItem::new(t!(
                "Unlimited prompts with your AI API keys"
            )))
            .child(ListBulletItem::new(t!("Unlimited use of external agents")))
    }

    pub fn sign_in_upsell(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("Unlimited edit predictions")))
            .child(ListBulletItem::new(t!("$20 of tokens in Zed agent")))
            .child(ListBulletItem::new(t!("No credit card required")))
    }

    pub fn pro_trial(&self, period: bool) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("$20 of tokens in Zed agent")))
            .child(ListBulletItem::new(t!("Unlimited edit predictions")))
            .when(period, |this| {
                this.child(ListBulletItem::new(t!(
                    "Try it out for 14 days, no credit card required"
                )))
            })
    }

    pub fn pro_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("$5 of tokens in Zed agent")))
            .child(ListBulletItem::new(t!("Usage-based billing beyond $5")))
            .child(ListBulletItem::new(t!("Unlimited edit predictions")))
    }

    pub fn business_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("Unlimited edit predictions")))
            .child(ListBulletItem::new(t!("Usage-based billing")))
    }

    pub fn vip_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("Unlimited edit predictions")))
            .child(ListBulletItem::new(t!("Tokens in the Zed agent")))
    }

    pub fn student_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(t!("Unlimited edit predictions")))
            .child(ListBulletItem::new(t!("$10 of tokens in Zed agent")))
            .child(ListBulletItem::new(t!(
                "Optional credit packs for additional usage"
            )))
    }
}
