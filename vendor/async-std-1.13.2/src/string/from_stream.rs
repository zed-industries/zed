use std::borrow::Cow;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, FromStream, IntoStream};

impl FromStream<char> for String {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = char> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = String::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}

impl<'b> FromStream<&'b char> for String {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = &'b char> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = String::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}

impl<'b> FromStream<&'b str> for String {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = &'b str> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = String::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}

impl FromStream<String> for String {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = String> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = String::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}

impl<'b> FromStream<Cow<'b, str>> for String {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = Cow<'b, str>> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = String::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}
