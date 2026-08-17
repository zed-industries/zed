use std::marker::PhantomData;

use super::*;
use chrono::{DateTime, Utc};

pub trait EndpointSelector {
    const ENDPOINT: &'static str;
}
pub struct AllOrByAuth;
pub struct PublicOnly;

impl EndpointSelector for AllOrByAuth {
    const ENDPOINT: &'static str = "/gists";
}

impl EndpointSelector for PublicOnly {
    const ENDPOINT: &'static str = "/gists/public";
}

#[derive(Debug, serde::Serialize)]
pub struct ListGistsBuilder<'octo, T: EndpointSelector> {
    #[serde(skip)]
    visibility_type: PhantomData<T>,

    #[serde(skip)]
    crab: &'octo Octocrab,

    /// Only show gists that were created after this UTC timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    /// The maximum number of results in each page retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    /// The page number to fetch. This starts at (and defaults to) 1
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, T: EndpointSelector> ListGistsBuilder<'octo, T> {
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self {
            visibility_type: PhantomData,
            crab,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// Only show gists that were created after this UTC timestamp.
    pub fn since(mut self, created_after: impl Into<DateTime<Utc>>) -> Self {
        self.since = Some(created_after.into());
        self
    }

    /// The maximum number of results in each page retrieved.
    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    /// The page number to fetch. This starts at (and defaults to) 1
    pub fn page(mut self, number: u32) -> Self {
        self.page = Some(number);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::gists::Gist>> {
        self.crab.get(T::ENDPOINT, Some(&self)).await
    }
}

/// Handles query data for the `GET /gists` endpoint.
///
/// This endpoint has differing behaviour depending on the status of
/// authentication.
pub type ListAllGistsBuilder<'octo> = ListGistsBuilder<'octo, AllOrByAuth>;

/// Handles query data for the `GET /gists/public` endpoint.
///
/// Fetches all publicly available gists on the GitHub instance with pagination.
pub type ListPublicGistsBuilder<'octo> = ListGistsBuilder<'octo, PublicOnly>;

/// Handles query data for the `GET /users/{username}/gists` endpoint.
#[derive(Debug, serde::Serialize)]
pub struct ListUserGistsBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip)]
    /// Username for which to retrieve gists
    username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListUserGistsBuilder<'octo> {
    pub fn new(crab: &'octo Octocrab, username: String) -> Self {
        Self {
            crab,
            username,
            since: None,
            per_page: None,
            page: None,
        }
    }

    pub fn since(mut self, last_updated: DateTime<Utc>) -> Self {
        self.since = Some(last_updated);
        self
    }

    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    pub fn page(mut self, number: u32) -> Self {
        self.page = Some(number);
        self
    }

    pub async fn send(self) -> crate::Result<crate::Page<Gist>> {
        self.crab
            .get(
                format!("/users/{username}/gists", username = self.username),
                Some(&self),
            )
            .await
    }
}
