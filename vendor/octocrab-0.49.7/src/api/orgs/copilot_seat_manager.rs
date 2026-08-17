use super::*;

pub struct CopilotSeatHandler<'octo, 'r> {
    handler: &'r OrgHandler<'octo>,
}

#[derive(serde::Serialize)]
struct SelectedTeams {
    selected_teams: Vec<String>,
}

#[derive(serde::Serialize)]
struct SelectedUsernames {
    selected_usernames: Vec<String>,
}

impl<'octo, 'r> CopilotSeatHandler<'octo, 'r> {
    pub fn new(handler: &'r OrgHandler<'octo>) -> Self {
        Self { handler }
    }

    /// Adds the specified teams from copilot seats.
    /// Note that this adds new seats immediately to your billing cycle.
    pub async fn add_teams(
        self,
        teams: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCreated> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_teams",
            org = self.handler.owner,
        );
        let teams = SelectedTeams {
            selected_teams: teams,
        };

        self.handler.crab.post(route, Some(&teams)).await
    }

    /// Removes the specified teams from copilot seats.
    /// Note that the seat removal takes effect the next billing cycle.
    pub async fn remove_teams(
        self,
        teams: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCancelled> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_teams",
            org = self.handler.owner,
        );
        let teams = SelectedTeams {
            selected_teams: teams,
        };

        self.handler.crab.delete(route, Some(&teams)).await
    }

    /// Adds the specified usernames from copilot seats.
    /// Note that this adds new seats immediately to your billing cycle.
    pub async fn add_usernames(
        self,
        usernames: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCreated> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_users",
            org = self.handler.owner,
        );
        let usernames = SelectedUsernames {
            selected_usernames: usernames,
        };

        self.handler.crab.post(route, Some(&usernames)).await
    }

    /// Removes the specified users from copilot seats.
    /// Note that the seat removal takes effect the next billing cycle.
    pub async fn remove_usernames(
        self,
        usernames: Vec<String>,
    ) -> crate::Result<crate::models::orgs_copilot::billing::SeatsCancelled> {
        let route = format!(
            "/orgs/{org}/copilot/billing/selected_users",
            org = self.handler.owner,
        );
        let usernames = SelectedUsernames {
            selected_usernames: usernames,
        };

        self.handler.crab.delete(route, Some(&usernames)).await
    }
}
