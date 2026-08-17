use std::{
    collections::HashMap,
    fmt::{self, Debug},
    marker::PhantomData,
    sync::Mutex,
};

use futures_util::StreamExt;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Error as SeError, Visitor},
    ser::SerializeTuple,
};
use zbus::{
    proxy::SignalStream,
    zvariant::{ObjectPath, Type, Value},
};

use crate::{Error, desktop::HandleToken, proxy::Proxy};

/// A typical response returned by the [`Request::response`].
/// of a [`Request`].
#[derive(Debug, Type)]
#[zvariant(signature = "(ua{sv})")]
pub enum Response<T> {
    /// Success, the request is carried out.
    Ok(T),
    /// The user cancelled the request or something else happened.
    Err(ResponseError),
}

#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
impl<T> Response<T> {
    /// The corresponding response type.
    pub fn response_type(self) -> ResponseType {
        match self {
            Self::Ok(_) => ResponseType::Success,
            Self::Err(err) => match err {
                ResponseError::Cancelled => ResponseType::Cancelled,
                ResponseError::Other => ResponseType::Other,
            },
        }
    }

    /// A successful response.
    pub fn ok(inner: T) -> Self {
        Self::Ok(inner)
    }

    /// Cancelled request.
    pub fn cancelled() -> Self {
        Self::Err(ResponseError::Cancelled)
    }

    /// Another error.
    pub fn other() -> Self {
        Self::Err(ResponseError::Other)
    }
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: for<'d> Deserialize<'d> + Type,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResponseVisitor<T>(PhantomData<fn() -> (ResponseType, T)>);

        impl<'de, T> Visitor<'de> for ResponseVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = (ResponseType, Option<T>);

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a tuple composed of the response status along with the response"
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let type_: ResponseType = seq.next_element()?.ok_or_else(|| A::Error::custom(
                    "Failed to deserialize the response. Expected a numeric (u) value as the first item of the returned tuple",
                ))?;
                if type_ == ResponseType::Success {
                    let data: T = seq.next_element()?.ok_or_else(|| A::Error::custom(
                        "Failed to deserialize the response. Expected a vardict (a{sv}) with the returned results",
                    ))?;
                    Ok((type_, Some(data)))
                } else {
                    Ok((type_, None))
                }
            }
        }

        let visitor = ResponseVisitor::<T>(PhantomData);
        let response: (ResponseType, Option<T>) = deserializer.deserialize_tuple(2, visitor)?;
        Ok(response.into())
    }
}

impl<T> Serialize for Response<T>
where
    T: Serialize + Type,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_tuple(2)?;
        match self {
            Self::Err(err) => {
                map.serialize_element(&ResponseType::from(*err))?;
                map.serialize_element(&HashMap::<&str, Value<'_>>::new())?;
            }
            Self::Ok(response) => {
                map.serialize_element(&ResponseType::Success)?;
                map.serialize_element(response)?;
            }
        };
        map.end()
    }
}

#[doc(hidden)]
impl<T> From<(ResponseType, Option<T>)> for Response<T> {
    fn from(f: (ResponseType, Option<T>)) -> Self {
        match f.0 {
            ResponseType::Success => {
                Response::Ok(f.1.expect("Expected a valid response, found nothing."))
            }
            ResponseType::Cancelled => Response::Err(ResponseError::Cancelled),
            ResponseType::Other => Response::Err(ResponseError::Other),
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Hash, Clone)]
/// An error returned a portal request caused by either the user cancelling the
/// request or something else.
pub enum ResponseError {
    /// The user canceled the request.
    Cancelled,
    /// Something else happened.
    Other,
}

impl std::error::Error for ResponseError {}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => f.write_str("Cancelled"),
            Self::Other => f.write_str("Other"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Type)]
/// Possible responses.
pub enum ResponseType {
    /// Success, the request is carried out.
    Success = 0,
    /// The user cancelled the interaction.
    Cancelled = 1,
    /// The user interaction was ended in some other way.
    Other = 2,
}

#[doc(hidden)]
impl From<ResponseError> for ResponseType {
    fn from(err: ResponseError) -> Self {
        match err {
            ResponseError::Other => Self::Other,
            ResponseError::Cancelled => Self::Cancelled,
        }
    }
}

/// The Request interface is shared by all portal interfaces.
///
/// When a portal method is called, the reply includes a handle (i.e. object
/// path) for a Request object, which will stay alive for the duration of the
/// user interaction related to the method call.
///
/// The portal indicates that a portal request interaction is over by emitting
/// the "Response" signal on the Request object.
///
/// The application can abort the interaction calling
/// [`close()`][`Request::close`] on the Request object.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Request`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html).
#[doc(alias = "org.freedesktop.portal.Request")]
pub struct Request<T>(
    Proxy<'static>,
    SignalStream<'static>,
    Mutex<Option<Result<T, Error>>>,
    PhantomData<T>,
)
where
    T: for<'de> Deserialize<'de> + Type + Debug;

impl<T> Request<T>
where
    T: for<'de> Deserialize<'de> + Type + Debug,
{
    pub(crate) async fn with_connection<P>(
        connection: zbus::Connection,
        path: P,
    ) -> Result<Request<T>, Error>
    where
        P: TryInto<ObjectPath<'static>>,
        P::Error: Into<zbus::Error>,
    {
        let proxy =
            Proxy::new_desktop_with_path(connection, "org.freedesktop.portal.Request", path)
                .await?;
        // Start listening for a response signal the moment request is created
        let stream = proxy.receive_signal("Response").await?;
        Ok(Self(proxy, stream, Default::default(), PhantomData))
    }

    pub(crate) async fn from_unique_name(
        connection: zbus::Connection,
        handle_token: &HandleToken,
    ) -> Result<Request<T>, Error> {
        let path = Proxy::unique_name(
            &connection,
            "/org/freedesktop/portal/desktop/request",
            handle_token,
        )
        .await?;
        #[cfg(feature = "tracing")]
        tracing::info!("Creating a org.freedesktop.portal.Request {}", path);
        Self::with_connection(connection, path).await
    }

    pub(crate) async fn prepare_response(&mut self) -> Result<(), Error> {
        let message = self.1.next().await.ok_or(Error::NoResponse)?;
        #[cfg(feature = "tracing")]
        tracing::info!("Received signal 'Response' on '{}'", self.0.interface());
        let response = match message.body().deserialize::<Response<T>>()? {
            Response::Err(e) => Err(e.into()),
            Response::Ok(r) => Ok(r),
        };
        #[cfg(feature = "tracing")]
        tracing::debug!("Received response {:#?}", response);
        let r = response as Result<T, Error>;
        *self.2.get_mut().unwrap() = Some(r);
        Ok(())
    }

    /// The corresponding response if the request was successful.
    ///
    /// # Specifications
    ///
    /// See also [`Response`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request-response).
    #[doc(alias = "Response")]
    pub fn response(&self) -> Result<T, Error> {
        // It should be safe to unwrap here as we are sure we have received a response
        // by the time the user calls response
        self.2.lock().unwrap().take().unwrap()
    }

    /// Closes the portal request to which this object refers and ends all
    /// related user interaction (dialogs, etc). A Response signal will not
    /// be emitted in this case.
    ///
    /// # Specifications
    ///
    /// See also [`Close`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request-close).
    #[doc(alias = "Close")]
    pub async fn close(&self) -> Result<(), Error> {
        self.0.call("Close", &()).await
    }

    /// Return the object path of the request.
    pub(crate) fn path(&self) -> &ObjectPath<'_> {
        self.0.path()
    }
}

impl<T> Debug for Request<T>
where
    T: for<'de> Deserialize<'de> + Type + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Request")
            .field(&self.path().as_str())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use zbus::zvariant::Value;

    use super::*;

    #[cfg(feature = "account")]
    #[test]
    fn response_signature() {
        use crate::desktop::account::UserInformation;
        assert_eq!(
            <(ResponseType, HashMap<&str, Value<'_>>)>::SIGNATURE,
            Response::<()>::SIGNATURE,
        );
        assert_eq!(
            <(ResponseType, UserInformation)>::SIGNATURE,
            Response::<UserInformation>::SIGNATURE,
        );
        assert_eq!(Response::<()>::SIGNATURE, "(ua{sv})");
    }
}
