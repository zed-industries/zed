use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Context};
use axum::{
    extract::{self, Query},
    routing::{get, post},
    Extension, Json, Router,
};
use reqwest::StatusCode;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use stripe::{
    BillingPortalSession, CheckoutSession, CreateBillingPortalSession,
    CreateBillingPortalSessionFlowData, CreateBillingPortalSessionFlowDataAfterCompletion,
    CreateBillingPortalSessionFlowDataAfterCompletionRedirect,
    CreateBillingPortalSessionFlowDataType, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCustomer, Customer, CustomerId, EventObject, EventType, Expandable, ListEvents,
    SubscriptionStatus,
};
use util::ResultExt;

use crate::db::billing_subscription::StripeSubscriptionStatus;
use crate::db::{
    billing_customer, BillingSubscriptionId, CreateBillingCustomerParams,
    CreateBillingSubscriptionParams, CreateProcessedStripeEventParams, UpdateBillingCustomerParams,
    UpdateBillingSubscriptionParams,
};
use crate::{AppState, Error, Result};

pub fn router() -> Router {
    Router::new()
        .route(
            "/billing/subscriptions",
            get(list_billing_subscriptions).post(create_billing_subscription),
        )
        .route(
            "/billing/subscriptions/manage",
            post(manage_billing_subscription),
        )
}

#[derive(Debug, Deserialize)]
struct ListBillingSubscriptionsParams {
    github_user_id: i32,
}

#[derive(Debug, Serialize)]
struct BillingSubscriptionJson {
    id: BillingSubscriptionId,
    name: String,
    status: StripeSubscriptionStatus,
    /// Whether this subscription can be canceled.
    is_cancelable: bool,
}

#[derive(Debug, Serialize)]
struct ListBillingSubscriptionsResponse {
    subscriptions: Vec<BillingSubscriptionJson>,
}

async fn list_billing_subscriptions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<ListBillingSubscriptionsParams>,
) -> Result<Json<ListBillingSubscriptionsResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(params.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let subscriptions = app.db.get_billing_subscriptions(user.id).await?;

    Ok(Json(ListBillingSubscriptionsResponse {
        subscriptions: subscriptions
            .into_iter()
            .map(|subscription| BillingSubscriptionJson {
                id: subscription.id,
                name: "Zed Pro".to_string(),
                status: subscription.stripe_subscription_status,
                is_cancelable: subscription.stripe_subscription_status.is_cancelable(),
            })
            .collect(),
    }))
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

    let customer_id =
        if let Some(existing_customer) = app.db.get_billing_customer_by_user_id(user.id).await? {
            CustomerId::from_str(&existing_customer.stripe_customer_id)
                .context("failed to parse customer ID")?
        } else {
            let customer = Customer::create(
                &stripe_client,
                CreateCustomer {
                    email: user.email_address.as_deref(),
                    ..Default::default()
                },
            )
            .await?;

            customer.id
        };

    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = Some(customer_id);
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
    subscription_id: BillingSubscriptionId,
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

    let customer = app
        .db
        .get_billing_customer_by_user_id(user.id)
        .await?
        .ok_or_else(|| anyhow!("billing customer not found"))?;
    let customer_id = CustomerId::from_str(&customer.stripe_customer_id)
        .context("failed to parse customer ID")?;

    let subscription = app
        .db
        .get_billing_subscription_by_id(body.subscription_id)
        .await?
        .ok_or_else(|| anyhow!("subscription not found"))?;

    let flow = match body.intent {
        ManageSubscriptionIntent::Cancel => CreateBillingPortalSessionFlowData {
            type_: CreateBillingPortalSessionFlowDataType::SubscriptionCancel,
            after_completion: Some(CreateBillingPortalSessionFlowDataAfterCompletion {
                type_: stripe::CreateBillingPortalSessionFlowDataAfterCompletionType::Redirect,
                redirect: Some(CreateBillingPortalSessionFlowDataAfterCompletionRedirect {
                    return_url: "https://zed.dev/settings".into(),
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
    params.return_url = Some("https://zed.dev/settings");

    let session = BillingPortalSession::create(&stripe_client, params).await?;

    Ok(Json(ManageBillingSubscriptionResponse {
        billing_portal_session_url: session.url,
    }))
}

const POLL_EVENTS_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Polls the Stripe events API periodically to reconcile the records in our
/// database with the data in Stripe.
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

    let mut unprocessed_events = Vec::new();

    loop {
        log::info!("retrieving events from Stripe: {}", event_types.join(", "));

        let mut params = ListEvents::new();
        params.types = Some(event_types.clone());
        params.limit = Some(100);

        let events = stripe::Event::list(stripe_client, &params).await?;

        let processed_event_ids = {
            let event_ids = &events
                .data
                .iter()
                .map(|event| event.id.as_str())
                .collect::<Vec<_>>();

            app.db
                .get_processed_stripe_events_by_event_ids(event_ids)
                .await?
                .into_iter()
                .map(|event| event.stripe_event_id)
                .collect::<Vec<_>>()
        };

        for event in events.data {
            if processed_event_ids.contains(&event.id.to_string()) {
                log::info!("Stripe event {} already processed: skipping", event.id);
            } else {
                unprocessed_events.push(event);
            }
        }

        if !events.has_more {
            break;
        }
    }

    log::info!(
        "unprocessed events from Stripe: {}",
        unprocessed_events.len()
    );

    // Sort all of the unprocessed events in ascending order, so we can handle them in the order they occurred.
    unprocessed_events.sort_by(|a, b| a.created.cmp(&b.created).then_with(|| a.id.cmp(&b.id)));

    for event in unprocessed_events {
        let processed_event_params = CreateProcessedStripeEventParams {
            stripe_event_id: event.id.to_string(),
            stripe_event_type: event_type_to_string(event.type_),
            stripe_event_created_timestamp: event.created,
        };

        match event.type_ {
            EventType::CustomerCreated | EventType::CustomerUpdated => {
                handle_customer_event(app, stripe_client, event)
                    .await
                    .log_err();
            }
            EventType::CustomerSubscriptionCreated
            | EventType::CustomerSubscriptionUpdated
            | EventType::CustomerSubscriptionPaused
            | EventType::CustomerSubscriptionResumed
            | EventType::CustomerSubscriptionDeleted => {
                handle_customer_subscription_event(app, stripe_client, event)
                    .await
                    .log_err();
            }
            _ => {}
        }

        app.db
            .create_processed_stripe_event(&processed_event_params)
            .await?;
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

async fn handle_customer_subscription_event(
    app: &Arc<AppState>,
    stripe_client: &stripe::Client,
    event: stripe::Event,
) -> anyhow::Result<()> {
    let EventObject::Subscription(subscription) = event.data.object else {
        bail!("unexpected event payload for {}", event.id);
    };

    log::info!("handling Stripe {} event: {}", event.type_, event.id);

    let billing_customer =
        find_or_create_billing_customer(app, stripe_client, subscription.customer)
            .await?
            .ok_or_else(|| anyhow!("billing customer not found"))?;

    if let Some(existing_subscription) = app
        .db
        .get_billing_subscription_by_stripe_subscription_id(&subscription.id)
        .await?
    {
        app.db
            .update_billing_subscription(
                existing_subscription.id,
                &UpdateBillingSubscriptionParams {
                    billing_customer_id: ActiveValue::set(billing_customer.id),
                    stripe_subscription_id: ActiveValue::set(subscription.id.to_string()),
                    stripe_subscription_status: ActiveValue::set(subscription.status.into()),
                },
            )
            .await?;
    } else {
        app.db
            .create_billing_subscription(&CreateBillingSubscriptionParams {
                billing_customer_id: billing_customer.id,
                stripe_subscription_id: subscription.id.to_string(),
                stripe_subscription_status: subscription.status.into(),
            })
            .await?;
    }

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

/// Finds or creates a billing customer using the provided customer.
async fn find_or_create_billing_customer(
    app: &Arc<AppState>,
    stripe_client: &stripe::Client,
    customer_or_id: Expandable<Customer>,
) -> anyhow::Result<Option<billing_customer::Model>> {
    let customer_id = match &customer_or_id {
        Expandable::Id(id) => id,
        Expandable::Object(customer) => customer.id.as_ref(),
    };

    // If we already have a billing customer record associated with the Stripe customer,
    // there's nothing more we need to do.
    if let Some(billing_customer) = app
        .db
        .get_billing_customer_by_stripe_customer_id(&customer_id)
        .await?
    {
        return Ok(Some(billing_customer));
    }

    // If all we have is a customer ID, resolve it to a full customer record by
    // hitting the Stripe API.
    let customer = match customer_or_id {
        Expandable::Id(id) => Customer::retrieve(&stripe_client, &id, &[]).await?,
        Expandable::Object(customer) => *customer,
    };

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
