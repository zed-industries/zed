use std::str::FromStr;
use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, bail};
use axum::{
    extract::{self, Query},
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{NaiveDateTime, SecondsFormat};
use collections::HashSet;
use serde::{Deserialize, Serialize};
use stripe::{
    CheckoutSession, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCustomer,
    Customer, CustomerId,
};

use crate::api::AuthenticatedUserParams;
use crate::db::billing_subscription::{self, StripeSubscriptionStatus};
use crate::db::{ContributorSelector, CreateBillingSubscriptionParams};
use crate::{AppState, Result};

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

async fn create_billing_subscription(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(body): extract::Json<CreateBillingSubscriptionBody>,
) -> Result<Json<CreateBillingSubscriptionResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(body.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let stripe_client = stripe::Client::new("");

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

    // let customer = if let Some(customer_id) = existing_customer_id {
    //     Customer::retrieve(&stripe_client, &customer_id, &[]).await?
    // } else {
    //     Customer::create(
    //         &stripe_client,
    //         CreateCustomer {
    //             email: user.email_address.as_deref(),
    //             metadata: Some(std::collections::HashMap::from([(
    //                 "github_login".into(),
    //                 user.github_login.clone(),
    //             )])),
    //             ..Default::default()
    //         },
    //     )
    //     .await?
    // };

    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = existing_customer_id;
        params.client_reference_id = Some(user.github_login.as_str());
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price: Some("".into()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.return_url = Some("");
        params.success_url = Some("");

        CheckoutSession::create(&stripe_client, params).await?
    };

    Ok(Json(CreateBillingSubscriptionResponse {
        checkout_session_url: checkout_session
            .url
            .ok_or_else(|| anyhow!("no checkout session URL"))?,
    }))
}
