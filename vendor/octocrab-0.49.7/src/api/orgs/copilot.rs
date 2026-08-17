use super::*;

#[derive(serde::Serialize)]
pub struct CopilotHandler<'octo, 'r> {
    #[serde(skip)]
    handler: &'r OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<chrono::DateTime<chrono::Utc>>,
}

impl<'octo, 'r> CopilotHandler<'octo, 'r> {
    pub fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            since: None,
            until: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    // Show usage metrics since this date.
    // This is a timestamp in ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ).
    // Maximum value is 28 days ago.
    pub fn since(mut self, since: chrono::DateTime<chrono::Utc>) -> Self {
        self.since = Some(since);
        self
    }

    // Show usage metrics until this date.
    // This is a timestamp in ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ) and should not preceed the since date if it is passed.
    pub fn until(mut self, until: chrono::DateTime<chrono::Utc>) -> Self {
        self.until = Some(until);
        self
    }

    // Retrieve copilot metrics for the entire organization
    pub async fn metrics(
        self,
    ) -> crate::Result<Vec<crate::models::orgs_copilot::metrics::CopilotMetrics>> {
        let route = format!("/orgs/{org}/copilot/metrics", org = self.handler.owner);

        self.handler.crab.get(route, Some(&self)).await
    }

    // Retrieve copilot metrics for a specific team within the organization
    pub async fn metrics_team<T: ToString>(
        self,
        team: T,
    ) -> crate::Result<Vec<crate::models::orgs_copilot::metrics::CopilotMetrics>> {
        let route = format!(
            "/orgs/{org}/team/{team}/copilot/metrics",
            org = self.handler.owner,
            team = team.to_string()
        );

        self.handler.crab.get(route, Some(&self)).await
    }

    pub async fn billing(
        self,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotBilling> {
        let route = format!("/orgs/{org}/copilot/billing", org = self.handler.owner);

        self.handler.crab.get(route, Some(&self)).await
    }

    pub async fn billing_seats(
        self,
    ) -> crate::Result<crate::models::orgs_copilot::billing::CopilotBillingSeats> {
        let route = format!(
            "/orgs/{org}/copilot/billing/seats",
            org = self.handler.owner,
        );

        self.handler.crab.get(route, Some(&self)).await
    }

    /// Perform seat management operations, such as adding or removing seats.
    /// These will typically affect your billing and require admin/manage billing permissions.
    pub fn manage_seats(&self) -> copilot_seat_manager::CopilotSeatHandler<'octo, '_> {
        copilot_seat_manager::CopilotSeatHandler::new(self.handler)
    }
}
