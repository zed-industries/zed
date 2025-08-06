use anyhow::{Context as _, bail};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use std::{sync::Arc, time::Duration};
use stripe::{CancellationDetailsReason, EventObject, EventType, ListEvents, SubscriptionStatus};
use util::ResultExt;

use crate::AppState;
use crate::db::billing_subscription::{
    StripeCancellationReason, StripeSubscriptionStatus, SubscriptionKind,
};
use crate::db::{
    CreateBillingCustomerParams, CreateBillingSubscriptionParams, CreateProcessedStripeEventParams,
    UpdateBillingCustomerParams, UpdateBillingSubscriptionParams, billing_customer,
};
use crate::rpc::{ResultExt as _, Server};
use crate::stripe_client::{
    StripeCancellationDetailsReason, StripeClient, StripeCustomerId, StripeSubscription,
    StripeSubscriptionId,
};

/// The amount of time we wait in between each poll of Stripe events.
///
/// This value should strike a balance between:
///   1. Being short enough that we update quickly when something in Stripe changes
///   2. Being long enough that we don't eat into our rate limits.
///
/// As a point of reference, the Sequin folks say they have this at **500ms**:
///
/// > We poll the Stripe /events endpoint every 500ms per account
/// >
/// > — https://blog.sequinstream.com/events-not-webhooks/
const POLL_EVENTS_INTERVAL: Duration = Duration::from_secs(5);

/// The maximum number of events to return per page.
///
/// We set this to 100 (the max) so we have to make fewer requests to Stripe.
///
/// > Limit can range between 1 and 100, and the default is 10.
const EVENTS_LIMIT_PER_PAGE: u64 = 100;

/// The number of pages consisting entirely of already-processed events that we
/// will see before we stop retrieving events.
///
/// This is used to prevent over-fetching the Stripe events API for events we've
/// already seen and processed.
const NUMBER_OF_ALREADY_PROCESSED_PAGES_BEFORE_WE_STOP: usize = 4;

/// Polls the Stripe events API periodically to reconcile the records in our
/// database with the data in Stripe.
pub fn poll_stripe_events_periodically(app: Arc<AppState>, rpc_server: Arc<Server>) {
    let Some(real_stripe_client) = app.real_stripe_client.clone() else {
        log::warn!("failed to retrieve Stripe client");
        return;
    };
    let Some(stripe_client) = app.stripe_client.clone() else {
        log::warn!("failed to retrieve Stripe client");
        return;
    };

    let executor = app.executor.clone();
    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            loop {
                poll_stripe_events(&app, &rpc_server, &stripe_client, &real_stripe_client)
                    .await
                    .log_err();

                executor.sleep(POLL_EVENTS_INTERVAL).await;
            }
        }
    });
}

async fn poll_stripe_events(
    app: &Arc<AppState>,
    rpc_server: &Arc<Server>,
    stripe_client: &Arc<dyn StripeClient>,
    real_stripe_client: &stripe::Client,
) -> anyhow::Result<()> {
    let feature_flags = app.db.list_feature_flags().await?;
    let sync_events_using_cloud = feature_flags
        .iter()
        .any(|flag| flag.flag == "cloud-stripe-events-polling" && flag.enabled_for_all);
    if sync_events_using_cloud {
        return Ok(());
    }

    fn event_type_to_string(event_type: EventType) -> String {
        // Calling `to_string` on `stripe::EventType` members gives us a quoted string,
        // so we need to unquote it.
        event_type.to_string().trim_matches('"').to_string()
    }

    let event_types = [
        EventType::CustomerCreated,
        EventType::CustomerUpdated,
        EventType::CustomerSubscriptionCreated,
        EventType::CustomerSubscriptionUpdated,
        EventType::CustomerSubscriptionPaused,
        EventType::CustomerSubscriptionResumed,
        EventType::CustomerSubscriptionDeleted,
    ]
    .into_iter()
    .map(event_type_to_string)
    .collect::<Vec<_>>();

    let mut pages_of_already_processed_events = 0;
    let mut unprocessed_events = Vec::new();

    log::info!(
        "Stripe events: starting retrieval for {}",
        event_types.join(", ")
    );
    let mut params = ListEvents::new();
    params.types = Some(event_types.clone());
    params.limit = Some(EVENTS_LIMIT_PER_PAGE);

    let mut event_pages = stripe::Event::list(&real_stripe_client, &params)
        .await?
        .paginate(params);

    loop {
        let processed_event_ids = {
            let event_ids = event_pages
                .page
                .data
                .iter()
                .map(|event| event.id.as_str())
                .collect::<Vec<_>>();
            app.db
                .get_processed_stripe_events_by_event_ids(&event_ids)
                .await?
                .into_iter()
                .map(|event| event.stripe_event_id)
                .collect::<Vec<_>>()
        };

        let mut processed_events_in_page = 0;
        let events_in_page = event_pages.page.data.len();
        for event in &event_pages.page.data {
            if processed_event_ids.contains(&event.id.to_string()) {
                processed_events_in_page += 1;
                log::debug!("Stripe events: already processed '{}', skipping", event.id);
            } else {
                unprocessed_events.push(event.clone());
            }
        }

        if processed_events_in_page == events_in_page {
            pages_of_already_processed_events += 1;
        }

        if event_pages.page.has_more {
            if pages_of_already_processed_events >= NUMBER_OF_ALREADY_PROCESSED_PAGES_BEFORE_WE_STOP
            {
                log::info!(
                    "Stripe events: stopping, saw {pages_of_already_processed_events} pages of already-processed events"
                );
                break;
            } else {
                log::info!("Stripe events: retrieving next page");
                event_pages = event_pages.next(&real_stripe_client).await?;
            }
        } else {
            break;
        }
    }

    log::info!("Stripe events: unprocessed {}", unprocessed_events.len());

    // Sort all of the unprocessed events in ascending order, so we can handle them in the order they occurred.
    unprocessed_events.sort_by(|a, b| a.created.cmp(&b.created).then_with(|| a.id.cmp(&b.id)));

    for event in unprocessed_events {
        let event_id = event.id.clone();
        let processed_event_params = CreateProcessedStripeEventParams {
            stripe_event_id: event.id.to_string(),
            stripe_event_type: event_type_to_string(event.type_),
            stripe_event_created_timestamp: event.created,
        };

        // If the event has happened too far in the past, we don't want to
        // process it and risk overwriting other more-recent updates.
        //
        // 1 day was chosen arbitrarily. This could be made longer or shorter.
        let one_day = Duration::from_secs(24 * 60 * 60);
        let a_day_ago = Utc::now() - one_day;
        if a_day_ago.timestamp() > event.created {
            log::info!(
                "Stripe events: event '{}' is more than {one_day:?} old, marking as processed",
                event_id
            );
            app.db
                .create_processed_stripe_event(&processed_event_params)
                .await?;

            continue;
        }

        let process_result = match event.type_ {
            EventType::CustomerCreated | EventType::CustomerUpdated => {
                handle_customer_event(app, real_stripe_client, event).await
            }
            EventType::CustomerSubscriptionCreated
            | EventType::CustomerSubscriptionUpdated
            | EventType::CustomerSubscriptionPaused
            | EventType::CustomerSubscriptionResumed
            | EventType::CustomerSubscriptionDeleted => {
                handle_customer_subscription_event(app, rpc_server, stripe_client, event).await
            }
            _ => Ok(()),
        };

        if let Some(()) = process_result
            .with_context(|| format!("failed to process event {event_id} successfully"))
            .log_err()
        {
            app.db
                .create_processed_stripe_event(&processed_event_params)
                .await?;
        }
    }

    Ok(())
}

async fn handle_customer_event(
    app: &Arc<AppState>,
    _stripe_client: &stripe::Client,
    event: stripe::Event,
) -> anyhow::Result<()> {
    let EventObject::Customer(customer) = event.data.object else {
        bail!("unexpected event payload for {}", event.id);
    };

    log::info!("handling Stripe {} event: {}", event.type_, event.id);

    let Some(email) = customer.email else {
        log::info!("Stripe customer has no email: skipping");
        return Ok(());
    };

    let Some(user) = app.db.get_user_by_email(&email).await? else {
        log::info!("no user found for email: skipping");
        return Ok(());
    };

    if let Some(existing_customer) = app
        .db
        .get_billing_customer_by_stripe_customer_id(&customer.id)
        .await?
    {
        app.db
            .update_billing_customer(
                existing_customer.id,
                &UpdateBillingCustomerParams {
                    // For now we just leave the information as-is, as it is not
                    // likely to change.
                    ..Default::default()
                },
            )
            .await?;
    } else {
        app.db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id: user.id,
                stripe_customer_id: customer.id.to_string(),
            })
            .await?;
    }

    Ok(())
}

async fn sync_subscription(
    app: &Arc<AppState>,
    stripe_client: &Arc<dyn StripeClient>,
    subscription: StripeSubscription,
) -> anyhow::Result<billing_customer::Model> {
    let subscription_kind = if let Some(stripe_billing) = &app.stripe_billing {
        stripe_billing
            .determine_subscription_kind(&subscription)
            .await
    } else {
        None
    };

    let billing_customer =
        find_or_create_billing_customer(app, stripe_client.as_ref(), &subscription.customer)
            .await?
            .context("billing customer not found")?;

    if let Some(SubscriptionKind::ZedProTrial) = subscription_kind {
        if subscription.status == SubscriptionStatus::Trialing {
            let current_period_start =
                DateTime::from_timestamp(subscription.current_period_start, 0)
                    .context("No trial subscription period start")?;

            app.db
                .update_billing_customer(
                    billing_customer.id,
                    &UpdateBillingCustomerParams {
                        trial_started_at: ActiveValue::set(Some(current_period_start.naive_utc())),
                        ..Default::default()
                    },
                )
                .await?;
        }
    }

    let was_canceled_due_to_payment_failure = subscription.status == SubscriptionStatus::Canceled
        && subscription
            .cancellation_details
            .as_ref()
            .and_then(|details| details.reason)
            .map_or(false, |reason| {
                reason == StripeCancellationDetailsReason::PaymentFailed
            });

    if was_canceled_due_to_payment_failure {
        app.db
            .update_billing_customer(
                billing_customer.id,
                &UpdateBillingCustomerParams {
                    has_overdue_invoices: ActiveValue::set(true),
                    ..Default::default()
                },
            )
            .await?;
    }

    if let Some(existing_subscription) = app
        .db
        .get_billing_subscription_by_stripe_subscription_id(subscription.id.0.as_ref())
        .await?
    {
        app.db
            .update_billing_subscription(
                existing_subscription.id,
                &UpdateBillingSubscriptionParams {
                    billing_customer_id: ActiveValue::set(billing_customer.id),
                    kind: ActiveValue::set(subscription_kind),
                    stripe_subscription_id: ActiveValue::set(subscription.id.to_string()),
                    stripe_subscription_status: ActiveValue::set(subscription.status.into()),
                    stripe_cancel_at: ActiveValue::set(
                        subscription
                            .cancel_at
                            .and_then(|cancel_at| DateTime::from_timestamp(cancel_at, 0))
                            .map(|time| time.naive_utc()),
                    ),
                    stripe_cancellation_reason: ActiveValue::set(
                        subscription
                            .cancellation_details
                            .and_then(|details| details.reason)
                            .map(|reason| reason.into()),
                    ),
                    stripe_current_period_start: ActiveValue::set(Some(
                        subscription.current_period_start,
                    )),
                    stripe_current_period_end: ActiveValue::set(Some(
                        subscription.current_period_end,
                    )),
                },
            )
            .await?;
    } else {
        if let Some(existing_subscription) = app
            .db
            .get_active_billing_subscription(billing_customer.user_id)
            .await?
        {
            if existing_subscription.kind == Some(SubscriptionKind::ZedFree)
                && subscription_kind == Some(SubscriptionKind::ZedProTrial)
            {
                let stripe_subscription_id = StripeSubscriptionId(
                    existing_subscription.stripe_subscription_id.clone().into(),
                );

                stripe_client
                    .cancel_subscription(&stripe_subscription_id)
                    .await?;
            } else {
                // If the user already has an active billing subscription, ignore the
                // event and return an `Ok` to signal that it was processed
                // successfully.
                //
                // There is the possibility that this could cause us to not create a
                // subscription in the following scenario:
                //
                //   1. User has an active subscription A
                //   2. User cancels subscription A
                //   3. User creates a new subscription B
                //   4. We process the new subscription B before the cancellation of subscription A
                //   5. User ends up with no subscriptions
                //
                // In theory this situation shouldn't arise as we try to process the events in the order they occur.

                log::info!(
                    "user {user_id} already has an active subscription, skipping creation of subscription {subscription_id}",
                    user_id = billing_customer.user_id,
                    subscription_id = subscription.id
                );
                return Ok(billing_customer);
            }
        }

        app.db
            .create_billing_subscription(&CreateBillingSubscriptionParams {
                billing_customer_id: billing_customer.id,
                kind: subscription_kind,
                stripe_subscription_id: subscription.id.to_string(),
                stripe_subscription_status: subscription.status.into(),
                stripe_cancellation_reason: subscription
                    .cancellation_details
                    .and_then(|details| details.reason)
                    .map(|reason| reason.into()),
                stripe_current_period_start: Some(subscription.current_period_start),
                stripe_current_period_end: Some(subscription.current_period_end),
            })
            .await?;
    }

    if let Some(stripe_billing) = app.stripe_billing.as_ref() {
        if subscription.status == SubscriptionStatus::Canceled
            || subscription.status == SubscriptionStatus::Paused
        {
            let already_has_active_billing_subscription = app
                .db
                .has_active_billing_subscription(billing_customer.user_id)
                .await?;
            if !already_has_active_billing_subscription {
                let stripe_customer_id =
                    StripeCustomerId(billing_customer.stripe_customer_id.clone().into());

                stripe_billing
                    .subscribe_to_zed_free(stripe_customer_id)
                    .await?;
            }
        }
    }

    Ok(billing_customer)
}

async fn handle_customer_subscription_event(
    app: &Arc<AppState>,
    rpc_server: &Arc<Server>,
    stripe_client: &Arc<dyn StripeClient>,
    event: stripe::Event,
) -> anyhow::Result<()> {
    let EventObject::Subscription(subscription) = event.data.object else {
        bail!("unexpected event payload for {}", event.id);
    };

    log::info!("handling Stripe {} event: {}", event.type_, event.id);

    let billing_customer = sync_subscription(app, stripe_client, subscription.into()).await?;

    // When the user's subscription changes, push down any changes to their plan.
    rpc_server
        .update_plan_for_user_legacy(billing_customer.user_id)
        .await
        .trace_err();

    // When the user's subscription changes, we want to refresh their LLM tokens
    // to either grant/revoke access.
    rpc_server
        .refresh_llm_tokens_for_user(billing_customer.user_id)
        .await;

    Ok(())
}

impl From<SubscriptionStatus> for StripeSubscriptionStatus {
    fn from(value: SubscriptionStatus) -> Self {
        match value {
            SubscriptionStatus::Incomplete => Self::Incomplete,
            SubscriptionStatus::IncompleteExpired => Self::IncompleteExpired,
            SubscriptionStatus::Trialing => Self::Trialing,
            SubscriptionStatus::Active => Self::Active,
            SubscriptionStatus::PastDue => Self::PastDue,
            SubscriptionStatus::Canceled => Self::Canceled,
            SubscriptionStatus::Unpaid => Self::Unpaid,
            SubscriptionStatus::Paused => Self::Paused,
        }
    }
}

impl From<CancellationDetailsReason> for StripeCancellationReason {
    fn from(value: CancellationDetailsReason) -> Self {
        match value {
            CancellationDetailsReason::CancellationRequested => Self::CancellationRequested,
            CancellationDetailsReason::PaymentDisputed => Self::PaymentDisputed,
            CancellationDetailsReason::PaymentFailed => Self::PaymentFailed,
        }
    }
}

/// Finds or creates a billing customer using the provided customer.
pub async fn find_or_create_billing_customer(
    app: &Arc<AppState>,
    stripe_client: &dyn StripeClient,
    customer_id: &StripeCustomerId,
) -> anyhow::Result<Option<billing_customer::Model>> {
    // If we already have a billing customer record associated with the Stripe customer,
    // there's nothing more we need to do.
    if let Some(billing_customer) = app
        .db
        .get_billing_customer_by_stripe_customer_id(customer_id.0.as_ref())
        .await?
    {
        return Ok(Some(billing_customer));
    }

    let customer = stripe_client.get_customer(customer_id).await?;

    let Some(email) = customer.email else {
        return Ok(None);
    };

    let Some(user) = app.db.get_user_by_email(&email).await? else {
        return Ok(None);
    };

    let billing_customer = app
        .db
        .create_billing_customer(&CreateBillingCustomerParams {
            user_id: user.id,
            stripe_customer_id: customer.id.to_string(),
        })
        .await?;

    Ok(Some(billing_customer))
}
