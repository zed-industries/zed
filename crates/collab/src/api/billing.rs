use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Context};
use axum::{extract, routing::post, Extension, Json, Router};
use collections::HashSet;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use stripe::{
    BillingPortalSession, CheckoutSession, CreateBillingPortalSession,
    CreateBillingPortalSessionFlowData, CreateBillingPortalSessionFlowDataAfterCompletion,
    CreateBillingPortalSessionFlowDataAfterCompletionRedirect,
    CreateBillingPortalSessionFlowDataType, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CustomerId, EventObject, EventType, ListEvents,
};
use util::ResultExt;

use crate::db::BillingSubscriptionId;
use crate::{AppState, Error, Result};

pub fn router() -> Router {
    Router::new()
        .route("/billing/subscriptions", post(create_billing_subscription))
        .route(
            "/billing/subscriptions/manage",
            post(manage_billing_subscription),
        )
}

#[derive(Debug, Deserialize)]
struct CreateBillingSubscriptionBody {
    github_user_id: i32,
}

#[derive(Debug, Serialize)]
struct CreateBillingSubscriptionResponse {
    checkout_session_url: String,
}

/// Initiates a Stripe Checkout session for creating a billing subscription.
async fn create_billing_subscription(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(body): extract::Json<CreateBillingSubscriptionBody>,
) -> Result<Json<CreateBillingSubscriptionResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(body.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let Some((stripe_client, stripe_price_id)) = app
        .stripe_client
        .clone()
        .zip(app.config.stripe_price_id.clone())
    else {
        log::error!("failed to retrieve Stripe client or price ID");
        Err(Error::Http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    let existing_customer_id = {
        let existing_subscriptions = app.db.get_billing_subscriptions(user.id).await?;
        let distinct_customer_ids = existing_subscriptions
            .iter()
            .map(|subscription| subscription.stripe_customer_id.as_str())
            .collect::<HashSet<_>>();
        // Sanity: Make sure we can determine a single Stripe customer ID for the user.
        if distinct_customer_ids.len() > 1 {
            Err(anyhow!("user has multiple existing customer IDs"))?;
        }

        distinct_customer_ids
            .into_iter()
            .next()
            .map(|id| CustomerId::from_str(id).context("failed to parse customer ID"))
            .transpose()
    }?;

    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = existing_customer_id;
        params.client_reference_id = Some(user.github_login.as_str());
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price: Some(stripe_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.success_url = Some("https://zed.dev/billing/success");

        CheckoutSession::create(&stripe_client, params).await?
    };

    Ok(Json(CreateBillingSubscriptionResponse {
        checkout_session_url: checkout_session
            .url
            .ok_or_else(|| anyhow!("no checkout session URL"))?,
    }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ManageSubscriptionIntent {
    /// The user intends to cancel their subscription.
    Cancel,
}

#[derive(Debug, Deserialize)]
struct ManageBillingSubscriptionBody {
    github_user_id: i32,
    intent: ManageSubscriptionIntent,
    /// The ID of the subscription to manage.
    ///
    /// If not provided, we will try to use the active subscription (if there is only one).
    subscription_id: Option<BillingSubscriptionId>,
}

#[derive(Debug, Serialize)]
struct ManageBillingSubscriptionResponse {
    billing_portal_session_url: String,
}

/// Initiates a Stripe customer portal session for managing a billing subscription.
async fn manage_billing_subscription(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(body): extract::Json<ManageBillingSubscriptionBody>,
) -> Result<Json<ManageBillingSubscriptionResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(body.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let Some(stripe_client) = app.stripe_client.clone() else {
        log::error!("failed to retrieve Stripe client");
        Err(Error::Http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    let subscription = if let Some(subscription_id) = body.subscription_id {
        app.db
            .get_billing_subscription_by_id(subscription_id)
            .await?
            .ok_or_else(|| anyhow!("subscription not found"))?
    } else {
        // If no subscription ID was provided, try to find the only active subscription ID.
        let subscriptions = app.db.get_active_billing_subscriptions(user.id).await?;
        if subscriptions.len() > 1 {
            Err(anyhow!("user has multiple active subscriptions"))?;
        }

        subscriptions
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("user has no active subscriptions"))?
    };

    let customer_id = CustomerId::from_str(&subscription.stripe_customer_id)
        .context("failed to parse customer ID")?;

    let flow = match body.intent {
        ManageSubscriptionIntent::Cancel => CreateBillingPortalSessionFlowData {
            type_: CreateBillingPortalSessionFlowDataType::SubscriptionCancel,
            after_completion: Some(CreateBillingPortalSessionFlowDataAfterCompletion {
                type_: stripe::CreateBillingPortalSessionFlowDataAfterCompletionType::Redirect,
                redirect: Some(CreateBillingPortalSessionFlowDataAfterCompletionRedirect {
                    return_url: "https://zed.dev/billing".into(),
                }),
                ..Default::default()
            }),
            subscription_cancel: Some(
                stripe::CreateBillingPortalSessionFlowDataSubscriptionCancel {
                    subscription: subscription.stripe_subscription_id,
                    retention: None,
                },
            ),
            ..Default::default()
        },
    };

    let mut params = CreateBillingPortalSession::new(customer_id);
    params.flow_data = Some(flow);
    params.return_url = Some("https://zed.dev/billing");

    let session = BillingPortalSession::create(&stripe_client, params).await?;

    Ok(Json(ManageBillingSubscriptionResponse {
        billing_portal_session_url: session.url,
    }))
}

const POLL_EVENTS_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub fn poll_stripe_events_periodically(app: Arc<AppState>) {
    let Some(stripe_client) = app.stripe_client.clone() else {
        log::warn!("failed to retrieve Stripe client");
        return;
    };

    let executor = app.executor.clone();
    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            loop {
                poll_stripe_events(&app, &stripe_client).await.log_err();

                executor.sleep(POLL_EVENTS_INTERVAL).await;
            }
        }
    });
}

async fn poll_stripe_events(
    app: &Arc<AppState>,
    stripe_client: &stripe::Client,
) -> anyhow::Result<()> {
    let event_types = [
        EventType::CustomerSubscriptionCreated.to_string(),
        EventType::CustomerSubscriptionUpdated.to_string(),
        EventType::CustomerSubscriptionPaused.to_string(),
        EventType::CustomerSubscriptionResumed.to_string(),
        EventType::CustomerSubscriptionDeleted.to_string(),
    ]
    .into_iter()
    .map(|event_type| {
        // Calling `to_string` on `stripe::EventType` members gives us a quoted string,
        // so we need to unquote it.
        event_type.trim_matches('"').to_string()
    })
    .collect::<Vec<_>>();

    loop {
        log::info!("retrieving events from Stripe: {}", event_types.join(", "));

        let mut params = ListEvents::new();
        params.types = Some(event_types.clone());
        params.limit = Some(100);
        // params.starting_after

        let events = stripe::Event::list(&stripe_client, &params).await?;
        for event in events.data {
            match event.type_ {
                EventType::CustomerSubscriptionCreated
                | EventType::CustomerSubscriptionUpdated
                | EventType::CustomerSubscriptionPaused
                | EventType::CustomerSubscriptionResumed
                | EventType::CustomerSubscriptionDeleted => {
                    handle_customer_subscription_event(app, event)
                        .await
                        .log_err();
                }
                _ => {}
            }
        }

        if !events.has_more {
            break;
        }
    }

    Ok(())
}

async fn handle_customer_subscription_event(
    app: &Arc<AppState>,
    event: stripe::Event,
) -> anyhow::Result<()> {
    let EventObject::Subscription(subscription) = event.data.object else {
        bail!("unexpected event payload for {}", event.id);
    };

    dbg!(&subscription.status);

    Ok(())
}
