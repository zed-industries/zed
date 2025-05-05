use crate::db::billing_subscription::{
    StripeCancellationReason, StripeSubscriptionStatus, SubscriptionKind,
};

use super::*;

#[derive(Debug)]
pub struct CreateBillingSubscriptionParams {
    pub billing_customer_id: BillingCustomerId,
    pub kind: Option<SubscriptionKind>,
    pub stripe_subscription_id: String,
    pub stripe_subscription_status: StripeSubscriptionStatus,
    pub stripe_cancellation_reason: Option<StripeCancellationReason>,
    pub stripe_current_period_start: Option<i64>,
    pub stripe_current_period_end: Option<i64>,
}

#[derive(Debug, Default)]
pub struct UpdateBillingSubscriptionParams {
    pub billing_customer_id: ActiveValue<BillingCustomerId>,
    pub kind: ActiveValue<Option<SubscriptionKind>>,
    pub stripe_subscription_id: ActiveValue<String>,
    pub stripe_subscription_status: ActiveValue<StripeSubscriptionStatus>,
    pub stripe_cancel_at: ActiveValue<Option<DateTime>>,
    pub stripe_cancellation_reason: ActiveValue<Option<StripeCancellationReason>>,
    pub stripe_current_period_start: ActiveValue<Option<i64>>,
    pub stripe_current_period_end: ActiveValue<Option<i64>>,
}

impl Database {
    /// Creates a new billing subscription.
    pub async fn create_billing_subscription(
        &self,
        params: &CreateBillingSubscriptionParams,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            billing_subscription::Entity::insert(billing_subscription::ActiveModel {
                billing_customer_id: ActiveValue::set(params.billing_customer_id),
                kind: ActiveValue::set(params.kind),
                stripe_subscription_id: ActiveValue::set(params.stripe_subscription_id.clone()),
                stripe_subscription_status: ActiveValue::set(params.stripe_subscription_status),
                stripe_cancellation_reason: ActiveValue::set(params.stripe_cancellation_reason),
                stripe_current_period_start: ActiveValue::set(params.stripe_current_period_start),
                stripe_current_period_end: ActiveValue::set(params.stripe_current_period_end),
                ..Default::default()
            })
            .exec_without_returning(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Updates the specified billing subscription.
    pub async fn update_billing_subscription(
        &self,
        id: BillingSubscriptionId,
        params: &UpdateBillingSubscriptionParams,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            billing_subscription::Entity::update(billing_subscription::ActiveModel {
                id: ActiveValue::set(id),
                billing_customer_id: params.billing_customer_id.clone(),
                kind: params.kind.clone(),
                stripe_subscription_id: params.stripe_subscription_id.clone(),
                stripe_subscription_status: params.stripe_subscription_status.clone(),
                stripe_cancel_at: params.stripe_cancel_at.clone(),
                stripe_cancellation_reason: params.stripe_cancellation_reason.clone(),
                stripe_current_period_start: params.stripe_current_period_start.clone(),
                stripe_current_period_end: params.stripe_current_period_end.clone(),
                created_at: ActiveValue::not_set(),
            })
            .exec(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Returns the billing subscription with the specified ID.
    pub async fn get_billing_subscription_by_id(
        &self,
        id: BillingSubscriptionId,
    ) -> Result<Option<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_subscription::Entity::find_by_id(id)
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns the billing subscription with the specified Stripe subscription ID.
    pub async fn get_billing_subscription_by_stripe_subscription_id(
        &self,
        stripe_subscription_id: &str,
    ) -> Result<Option<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_subscription::Entity::find()
                .filter(
                    billing_subscription::Column::StripeSubscriptionId.eq(stripe_subscription_id),
                )
                .one(&*tx)
                .await?)
        })
        .await
    }

    pub async fn get_active_billing_subscription(
        &self,
        user_id: UserId,
    ) -> Result<Option<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_subscription::Entity::find()
                .inner_join(billing_customer::Entity)
                .filter(billing_customer::Column::UserId.eq(user_id))
                .filter(
                    Condition::all()
                        .add(
                            Condition::any()
                                .add(
                                    billing_subscription::Column::StripeSubscriptionStatus
                                        .eq(StripeSubscriptionStatus::Active),
                                )
                                .add(
                                    billing_subscription::Column::StripeSubscriptionStatus
                                        .eq(StripeSubscriptionStatus::Trialing),
                                ),
                        )
                        .add(billing_subscription::Column::Kind.is_not_null()),
                )
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns all of the billing subscriptions for the user with the specified ID.
    ///
    /// Note that this returns the subscriptions regardless of their status.
    /// If you're wanting to check if a use has an active billing subscription,
    /// use `get_active_billing_subscriptions` instead.
    pub async fn get_billing_subscriptions(
        &self,
        user_id: UserId,
    ) -> Result<Vec<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            let subscriptions = billing_subscription::Entity::find()
                .inner_join(billing_customer::Entity)
                .filter(billing_customer::Column::UserId.eq(user_id))
                .order_by_asc(billing_subscription::Column::Id)
                .all(&*tx)
                .await?;

            Ok(subscriptions)
        })
        .await
    }

    pub async fn get_active_billing_subscriptions(
        &self,
        user_ids: HashSet<UserId>,
    ) -> Result<HashMap<UserId, (billing_customer::Model, billing_subscription::Model)>> {
        self.transaction(|tx| {
            let user_ids = user_ids.clone();
            async move {
                let mut rows = billing_subscription::Entity::find()
                    .inner_join(billing_customer::Entity)
                    .select_also(billing_customer::Entity)
                    .filter(billing_customer::Column::UserId.is_in(user_ids))
                    .filter(
                        billing_subscription::Column::StripeSubscriptionStatus
                            .eq(StripeSubscriptionStatus::Active),
                    )
                    .filter(billing_subscription::Column::Kind.is_null())
                    .order_by_asc(billing_subscription::Column::Id)
                    .stream(&*tx)
                    .await?;

                let mut subscriptions = HashMap::default();
                while let Some(row) = rows.next().await {
                    if let (subscription, Some(customer)) = row? {
                        subscriptions.insert(customer.user_id, (customer, subscription));
                    }
                }
                Ok(subscriptions)
            }
        })
        .await
    }

    pub async fn get_active_zed_pro_billing_subscriptions(
        &self,
        user_ids: HashSet<UserId>,
    ) -> Result<HashMap<UserId, (billing_customer::Model, billing_subscription::Model)>> {
        self.transaction(|tx| {
            let user_ids = user_ids.clone();
            async move {
                let mut rows = billing_subscription::Entity::find()
                    .inner_join(billing_customer::Entity)
                    .select_also(billing_customer::Entity)
                    .filter(billing_customer::Column::UserId.is_in(user_ids))
                    .filter(
                        billing_subscription::Column::StripeSubscriptionStatus
                            .eq(StripeSubscriptionStatus::Active),
                    )
                    .filter(billing_subscription::Column::Kind.eq(SubscriptionKind::ZedPro))
                    .order_by_asc(billing_subscription::Column::Id)
                    .stream(&*tx)
                    .await?;

                let mut subscriptions = HashMap::default();
                while let Some(row) = rows.next().await {
                    if let (subscription, Some(customer)) = row? {
                        subscriptions.insert(customer.user_id, (customer, subscription));
                    }
                }
                Ok(subscriptions)
            }
        })
        .await
    }

    /// Returns whether the user has an active billing subscription.
    pub async fn has_active_billing_subscription(&self, user_id: UserId) -> Result<bool> {
        Ok(self.count_active_billing_subscriptions(user_id).await? > 0)
    }

    /// Returns the count of the active billing subscriptions for the user with the specified ID.
    pub async fn count_active_billing_subscriptions(&self, user_id: UserId) -> Result<usize> {
        self.transaction(|tx| async move {
            let count = billing_subscription::Entity::find()
                .inner_join(billing_customer::Entity)
                .filter(
                    billing_customer::Column::UserId.eq(user_id).and(
                        billing_subscription::Column::StripeSubscriptionStatus
                            .eq(StripeSubscriptionStatus::Active),
                    ),
                )
                .count(&*tx)
                .await?;

            Ok(count as usize)
        })
        .await
    }
}
