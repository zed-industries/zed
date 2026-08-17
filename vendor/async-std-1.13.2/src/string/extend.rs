use std::borrow::Cow;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, IntoStream};

impl stream::Extend<char> for String {
    fn extend<'a, S: IntoStream<Item = char> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();
        self.reserve(stream.size_hint().0);

        Box::pin(async move {
            pin_utils::pin_mut!(stream);

            while let Some(item) = stream.next().await {
                self.push(item);
            }
        })
    }
}

impl<'b> stream::Extend<&'b char> for String {
    fn extend<'a, S: IntoStream<Item = &'b char> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            pin_utils::pin_mut!(stream);

            while let Some(item) = stream.next().await {
                self.push(*item);
            }
        })
    }
}

impl<'b> stream::Extend<&'b str> for String {
    fn extend<'a, S: IntoStream<Item = &'b str> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            pin_utils::pin_mut!(stream);

            while let Some(item) = stream.next().await {
                self.push_str(item);
            }
        })
    }
}

impl stream::Extend<String> for String {
    fn extend<'a, S: IntoStream<Item = String> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            pin_utils::pin_mut!(stream);

            while let Some(item) = stream.next().await {
                self.push_str(&item);
            }
        })
    }
}

impl<'b> stream::Extend<Cow<'b, str>> for String {
    fn extend<'a, S: IntoStream<Item = Cow<'b, str>> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            pin_utils::pin_mut!(stream);

            while let Some(item) = stream.next().await {
                self.push_str(&item);
            }
        })
    }
}
