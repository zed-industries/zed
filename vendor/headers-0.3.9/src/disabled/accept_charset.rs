use {Charset, QualityItem};

header! {
    /// `Accept-Charset` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.3)
    ///
    /// The `Accept-Charset` header field can be sent by a user agent to
    /// indicate what charsets are acceptable in textual response content.
    /// This field allows user agents capable of understanding more
    /// comprehensive or special-purpose charsets to signal that capability
    /// to an origin server that is capable of representing information in
    /// those charsets.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept-Charset = 1#( ( charset / "*" ) [ weight ] )
    /// ```
    ///
    /// # Example values
    /// * `iso-8859-5, unicode-1-1;q=0.8`
    ///
    /// # Examples
    /// ```
    /// use headers::{Headers, AcceptCharset, Charset, qitem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AcceptCharset(vec![qitem(Charset::Us_Ascii)])
    /// );
    /// ```
    /// ```
    /// use headers::{Headers, AcceptCharset, Charset, q, QualityItem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AcceptCharset(vec![
    ///         QualityItem::new(Charset::Us_Ascii, q(900)),
    ///         QualityItem::new(Charset::Iso_8859_10, q(200)),
    ///     ])
    /// );
    /// ```
    /// ```
    /// use headers::{Headers, AcceptCharset, Charset, qitem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AcceptCharset(vec![qitem(Charset::Ext("utf-8".to_owned()))])
    /// );
    /// ```
    (AcceptCharset, ACCEPT_CHARSET) => (QualityItem<Charset>)+

    test_accept_charset {
        /// Testcase from RFC
        test_header!(test1, vec![b"iso-8859-5, unicode-1-1;q=0.8"]);
    }
}
