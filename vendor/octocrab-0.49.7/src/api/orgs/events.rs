//! GitHub Organization Events
use crate::{
    etag::{EntityTag, Etagged},
    models::events,
    orgs::OrgHandler,
    FromResponse, Page,
};
use http::request::Builder;
use http::{header::HeaderMap, Method, StatusCode};

pub struct ListOrgEventsBuilder<'octo, 'handler> {
    handler: &'handler OrgHandler<'octo>,
    headers: Headers,
    params: Params,
}

struct Headers {
    etag: Option<EntityTag>,
}

#[derive(serde::Serialize)]
struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'handler> ListOrgEventsBuilder<'octo, 'handler> {
    pub(crate) fn new(handler: &'handler OrgHandler<'octo>) -> Self {
        Self {
            handler,
            headers: Headers { etag: None },
            params: Params {
                per_page: None,
                page: None,
            },
        }
    }

    /// Etag for this request.
    pub fn etag(mut self, etag: Option<EntityTag>) -> Self {
        self.headers.etag = etag;
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.params.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.params.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Etagged<Page<events::Event>>> {
        let route = format!("/orgs/{owner}/events", owner = self.handler.owner);

        let uri = self
            .handler
            .crab
            .parameterized_uri(route, Some(&self.params))?;

        let mut headers = HeaderMap::new();
        if let Some(etag) = self.headers.etag {
            EntityTag::insert_if_none_match_header(&mut headers, etag)?;
        }

        let mut request = Builder::new().uri(uri).method(Method::GET);
        for (key, value) in headers.iter() {
            request = request.header(key, value);
        }

        let request = self.handler.crab.build_request(request, None::<&()>)?;
        let response = self.handler.crab.execute(request).await?;
        let etag = EntityTag::extract_from_response(&response);
        if response.status() == StatusCode::NOT_MODIFIED {
            Ok(Etagged { etag, value: None })
        } else {
            <Page<events::Event>>::from_response(crate::map_github_error(response).await?)
                .await
                .map(|page| Etagged {
                    etag,
                    value: Some(page),
                })
        }
    }
}
