use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use axum::{extract, routing::post, Extension, Json, Router};
use collections::HashSet;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use stripe::{CheckoutSession, CreateCheckoutSession, CreateCheckoutSessionLineItems, CustomerId};

use crate::{AppState, Error, Result};

pub fn router() -> Router {
    Router::new().route("/billing/subscriptions", post(create_billing_subscription))
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
            .map(|id| CustomerId::from_str(id).map_err(|err| anyhow!(err)))
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
