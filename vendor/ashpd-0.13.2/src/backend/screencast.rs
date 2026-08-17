use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use enumflags2::BitFlags;
use serde::Serialize;

use crate::{
    AppID, PortalError, WindowIdentifierType,
    backend::{
        Result,
        request::{Request, RequestImpl},
        session::{CreateSessionResponse, Session, SessionImpl, SessionManager},
    },
    desktop::{
        CreateSessionOptions, HandleToken,
        request::Response,
        screencast::{CursorMode, SelectSourcesOptions, SourceType, StartCastOptions, Streams},
    },
    zvariant::{Optional, OwnedObjectPath, Type},
};

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct SelectSourcesResponse {}

#[async_trait]
pub trait ScreencastImpl: RequestImpl + SessionImpl {
    #[doc(alias = "AvailableSourceTypes")]
    fn available_source_types(&self) -> BitFlags<SourceType>;

    #[doc(alias = "AvailableCursorModes")]
    fn available_cursor_mode(&self) -> BitFlags<CursorMode>;

    #[doc(alias = "CreateSession")]
    async fn create_session(
        &self,
        token: HandleToken,
        session_token: HandleToken,
        app_id: Option<AppID>,
        options: CreateSessionOptions,
    ) -> Result<CreateSessionResponse>;

    #[doc(alias = "SelectSources")]
    async fn select_sources(
        &self,
        session_token: HandleToken,
        app_id: Option<AppID>,
        options: SelectSourcesOptions,
    ) -> Result<SelectSourcesResponse>;

    #[doc(alias = "Start")]
    async fn start_cast(
        &self,
        session_token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        options: StartCastOptions,
    ) -> Result<Streams>;
}

pub(crate) struct ScreencastInterface {
    imp: Arc<dyn ScreencastImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
    sessions: Arc<Mutex<SessionManager>>,
}

impl ScreencastInterface {
    pub fn new(
        imp: Arc<dyn ScreencastImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
        sessions: Arc<Mutex<SessionManager>>,
    ) -> Self {
        Self {
            imp,
            cnx,
            spawn,
            sessions,
        }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.ScreenCast")]
impl ScreencastInterface {
    #[zbus(
        property(emits_changed_signal = "const"),
        name = "AvailableSourceTypes"
    )]
    fn available_source_types(&self) -> u32 {
        let imp = Arc::clone(&self.imp);
        imp.available_source_types().bits()
    }

    #[zbus(
        property(emits_changed_signal = "const"),
        name = "AvailableCursorModes"
    )]
    fn available_cursor_mode(&self) -> u32 {
        let imp = Arc::clone(&self.imp);
        imp.available_cursor_mode().bits()
    }

    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        5
    }

    #[zbus(name = "CreateSession")]
    #[zbus(out_args("response", "results"))]
    async fn create_session(
        &self,
        handle: OwnedObjectPath,
        session_handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        options: CreateSessionOptions,
    ) -> Result<Response<CreateSessionResponse>> {
        let session_token = HandleToken::try_from(&session_handle).unwrap();
        {
            let sessions = self.sessions.lock().unwrap();
            if sessions.contains(&session_token) {
                let errormsg = format!("A session with handle `{session_token}` already exists");
                #[cfg(feature = "tracing")]
                tracing::error!("ScreencastInterface::create_session: {}", errormsg);
                return Err(PortalError::Exist(errormsg));
            }
        }

        let imp = Arc::clone(&self.imp);
        let token = session_token.clone();
        let result = Request::spawn(
            "ScreenCast::CreateSession",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.create_session(
                    HandleToken::try_from(&handle).unwrap(),
                    token,
                    app_id.into(),
                    options,
                )
                .await
            },
        )
        .await;

        if result.is_ok() {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "ScreencastInterface::create_session: session with handle `{session_token}` created"
            );

            let session = Session::new(
                session_handle,
                Arc::clone(&self.sessions),
                Some(Arc::clone(&self.imp) as Arc<dyn SessionImpl>),
            );
            session.serve(self.cnx.clone()).await?;
            {
                let mut sessions = self.sessions.lock().unwrap();
                sessions.add(session);
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "ScreencastInterface::create_session: failed to create a session with handle `{session_token}`"
            );
        }

        result
    }

    #[zbus(name = "SelectSources")]
    #[zbus(out_args("response", "results"))]
    async fn select_sources(
        &self,
        handle: OwnedObjectPath,
        session_handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        options: SelectSourcesOptions,
    ) -> Result<Response<SelectSourcesResponse>> {
        let session_token = HandleToken::try_from(&session_handle).unwrap();
        {
            let sessions = self.sessions.lock().unwrap();
            sessions.try_contains(&session_token)?;
        }

        let imp = Arc::clone(&self.imp);
        Request::spawn(
            "ScreenCast::SelectSources",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.select_sources(session_token, app_id.into(), options)
                    .await
            },
        )
        .await
    }

    #[zbus(name = "Start")]
    #[zbus(out_args("response", "results"))]
    async fn start(
        &self,
        handle: OwnedObjectPath,
        session_handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        options: StartCastOptions,
    ) -> Result<Response<Streams>> {
        let session_token = HandleToken::try_from(&session_handle).unwrap();
        {
            let sessions = self.sessions.lock().unwrap();
            sessions.try_contains(&session_token)?;
        }

        let imp = Arc::clone(&self.imp);
        Request::spawn(
            "ScreenCast::Start",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.start_cast(
                    session_token,
                    app_id.into(),
                    window_identifier.into(),
                    options,
                )
                .await
            },
        )
        .await
    }
}
