use {Encoding, QualityItem};

header! {
    /// `Accept-Encoding` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.4)
    ///
    /// The `Accept-Encoding` header field can be used by user agents to
    /// indicate what response content-codings are
    /// acceptable in the response.  An  `identity` token is used as a synonym
    /// for "no encoding" in order to communicate when no encoding is
    /// preferred.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept-Encoding  = #( codings [ weight ] )
    /// codings          = content-coding / "identity" / "*"
    /// ```
    ///
    /// # Example values
    /// * `compress, gzip`
    /// * ``
    /// * `*`
    /// * `compress;q=0.5, gzip;q=1`
    /// * `gzip;q=1.0, identity; q=0.5, *;q=0`
    ///
    /// # Examples
    /// ```
    /// use headers::{Headers, AcceptEncoding, Encoding, qitem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AcceptEncoding(vec![qitem(Encoding::Chunked)])
    /// );
    /// ```
    /// ```
    /// use headers::{Headers, AcceptEncoding, Encoding, qitem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AcceptEncoding(vec![
    ///         qitem(Encoding::Chunked),
    ///         qitem(Encoding::Gzip),
    ///         qitem(Encoding::Deflate),
    ///     ])
    /// );
    /// ```
    /// ```
    /// use headers::{Headers, AcceptEncoding, Encoding, QualityItem, q, qitem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AcceptEncoding(vec![
    ///         qitem(Encoding::Chunked),
    ///         QualityItem::new(Encoding::Gzip, q(600)),
    ///         QualityItem::new(Encoding::EncodingExt("*".to_owned()), q(0)),
    ///     ])
    /// );
    /// ```
    (AcceptEncoding, ACCEPT_ENCODING) => (QualityItem<Encoding>)*

    test_accept_encoding {
        // From the RFC
        test_header!(test1, vec![b"compress, gzip"]);
        test_header!(test2, vec![b""], Some(AcceptEncoding(vec![])));
        test_header!(test3, vec![b"*"]);
        // Note: Removed quality 1 from gzip
        test_header!(test4, vec![b"compress;q=0.5, gzip"]);
        // Note: Removed quality 1 from gzip
        test_header!(test5, vec![b"gzip, identity; q=0.5, *;q=0"]);
    }
}
