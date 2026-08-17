//! Github Notifications API

use crate::error::HttpSnafu;
use crate::models::activity::Notification;
use crate::models::activity::ThreadSubscription;
use crate::models::{NotificationId, ThreadId};
use crate::Octocrab;
use crate::Page;
use http::Uri;
use snafu::ResultExt;

type DateTime = chrono::DateTime<chrono::Utc>;

/// Handler for GitHub's notifications API.
///
/// Created with [`ActivityHandler::notifications`].
/// **Note:** All of these methods require authentication using
/// your GitHub Access Token with the right privileges.
///
/// [`ActivityHandler::notifications`]: ../struct.ActivityHandler.html#method.notifications
pub struct NotificationsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> NotificationsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Gets a notification by their id.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let thread = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .get(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: NotificationId) -> crate::Result<Notification> {
        let route = format!("/notifications/threads/{id}");
        self.crab.get(route, None::<&()>).await
    }

    /// Marks a single thread as read.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .mark_as_read(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_as_read(&self, id: NotificationId) -> crate::Result<()> {
        let route = format!("/notifications/threads/{id}");
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        let response = self.crab._patch(uri, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Marks all notifications in a repository as read.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .mark_repo_as_read("XAMPPRocky", "octocrab", None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_repo_as_read(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        last_read_at: impl Into<Option<DateTime>>,
    ) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        struct Inner {
            last_read_at: DateTime,
        }

        let body = last_read_at
            .into()
            .map(|last_read_at| Inner { last_read_at });

        let route = format!("/repos/{}/{}/notifications", owner.as_ref(), repo.as_ref());
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        let response = self.crab._put(uri, body.as_ref()).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// Marks all notifications as read.
    ///
    /// If you provide a `last_read_at` parameter,
    /// anything updated since this time will not be marked as read.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .mark_all_as_read(None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_all_as_read(
        &self,
        last_read_at: impl Into<Option<DateTime>>,
    ) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        struct Inner {
            last_read_at: DateTime,
        }

        let body = last_read_at
            .into()
            .map(|last_read_at| Inner { last_read_at });
        let uri = Uri::builder()
            .path_and_query("/notifications")
            .build()
            .context(HttpSnafu)?;

        let response = self.crab._put(uri, body.as_ref()).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// This checks to see if the current user is subscribed to a thread.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let subscription = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .get_thread_subscription(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_thread_subscription(
        &self,
        thread: ThreadId,
    ) -> crate::Result<ThreadSubscription> {
        let route = format!("/notifications/threads/{thread}/subscription");

        self.crab.get(route, None::<&()>).await
    }

    /// Ignore or unignore a thread subscription, that is enabled by watching a repository.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let subscription = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .set_thread_subscription(123u64.into(), true)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_thread_subscription(
        &self,
        thread: ThreadId,
        ignored: bool,
    ) -> crate::Result<ThreadSubscription> {
        #[derive(serde::Serialize)]
        struct Inner {
            ignored: bool,
        }

        let route = format!("/notifications/threads/{thread}/subscription");
        let body = Inner { ignored };

        self.crab.put(route, Some(&body)).await
    }

    /// Mutes the whole thread conversation until you comment or get mentioned.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .delete_thread_subscription(123u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_thread_subscription(&self, thread: ThreadId) -> crate::Result<()> {
        let route = format!("/notifications/threads/{thread}/subscription");

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let response = self.crab._delete(uri, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List all notifications for the current user, that are in a given repository.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let notifications = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .list_for_repo("XAMPPRocky", "octocrab")
    ///     // Also show notifications that are marked as read.
    ///     .all(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_for_repo(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> ListNotificationsBuilder<'octo> {
        let route = format!("/repos/{}/{}/notifications", owner.as_ref(), repo.as_ref());
        ListNotificationsBuilder::new(self.crab, route)
    }

    /// List all notifications for the current user.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let notifications = octocrab::instance()
    ///     .activity()
    ///     .notifications()
    ///     .list()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListNotificationsBuilder<'octo> {
        ListNotificationsBuilder::new(self.crab, "/notifications".to_string())
    }
}

/// A builder pattern struct for listing pull requests.
///
/// Created by [`NotificationsHandler::list`].
///
/// [`NotificationsHandler::list`]: ./struct.NotificationsHandler.html#method.list
#[derive(serde::Serialize)]
pub struct ListNotificationsBuilder<'octo> {
    #[serde(skip)]
    url: String,
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    participating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo> ListNotificationsBuilder<'octo> {
    fn new(crab: &'octo Octocrab, url: String) -> Self {
        Self {
            url,
            crab,
            all: None,
            participating: None,
            since: None,
            before: None,
            per_page: None,
            page: None,
        }
    }

    /// If set, show notifications marked as read.
    pub fn all(mut self, v: bool) -> Self {
        self.all = Some(v);
        self
    }

    /// If set, only shows notifications in which the user is directly participating or mentioned.
    pub fn participating(mut self, v: bool) -> Self {
        self.participating = Some(v);
        self
    }

    /// Only show notifications updated after the given time.
    pub fn since(mut self, since: chrono::DateTime<chrono::Utc>) -> Self {
        self.since = Some(since);
        self
    }

    /// Only show notifications updated before the given time.
    pub fn before(mut self, before: chrono::DateTime<chrono::Utc>) -> Self {
        self.before = Some(before);
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Notification>> {
        self.crab.get(&self.url, Some(&self)).await
    }
}
