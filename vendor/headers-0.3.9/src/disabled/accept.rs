use mime::{self, Mime};

use {QualityItem, qitem};

header! {
    /// `Accept` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.2)
    ///
    /// The `Accept` header field can be used by user agents to specify
    /// response media types that are acceptable.  Accept header fields can
    /// be used to indicate that the request is specifically limited to a
    /// small set of desired types, as in the case of a request for an
    /// in-line image
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept = #( media-range [ accept-params ] )
    ///
    /// media-range    = ( "*/*"
    ///                  / ( type "/" "*" )
    ///                  / ( type "/" subtype )
    ///                  ) *( OWS ";" OWS parameter )
    /// accept-params  = weight *( accept-ext )
    /// accept-ext = OWS ";" OWS token [ "=" ( token / quoted-string ) ]
    /// ```
    ///
    /// # Example values
    /// * `audio/*; q=0.2, audio/basic`
    /// * `text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c`
    ///
    /// # Examples
    /// ```
    /// # extern crate headers;
    /// extern crate mime;
    /// use headers::{Headers, Accept, qitem};
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.set(
    ///     Accept(vec![
    ///         qitem(mime::TEXT_HTML),
    ///     ])
    /// );
    /// ```
    ///
    /// ```
    /// # extern crate headers;
    /// extern crate mime;
    /// use headers::{Headers, Accept, qitem};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     Accept(vec![
    ///         qitem(mime::APPLICATION_JSON),
    ///     ])
    /// );
    /// ```
    /// ```
    /// # extern crate headers;
    /// extern crate mime;
    /// use headers::{Headers, Accept, QualityItem, q, qitem};
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.set(
    ///     Accept(vec![
    ///         qitem(mime::TEXT_HTML),
    ///         qitem("application/xhtml+xml".parse().unwrap()),
    ///         QualityItem::new(
    ///             mime::TEXT_XML,
    ///             q(900)
    ///         ),
    ///         qitem("image/webp".parse().unwrap()),
    ///         QualityItem::new(
    ///             mime::STAR_STAR,
    ///             q(800)
    ///         ),
    ///     ])
    /// );
    /// ```
    (Accept, ACCEPT) => (QualityItem<Mime>)+

    test_accept {
        // Tests from the RFC
         test_header!(
            test1,
            vec![b"audio/*; q=0.2, audio/basic"],
            Some(HeaderField(vec![
                QualityItem::new("audio/*".parse().unwrap(), q(200)),
                qitem("audio/basic".parse().unwrap()),
                ])));
        test_header!(
            test2,
            vec![b"text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c"],
            Some(HeaderField(vec![
                QualityItem::new(TEXT_PLAIN, q(500)),
                qitem(TEXT_HTML),
                QualityItem::new(
                    "text/x-dvi".parse().unwrap(),
                    q(800)),
                qitem("text/x-c".parse().unwrap()),
                ])));
        // Custom tests
        test_header!(
            test3,
            vec![b"text/plain; charset=utf-8"],
            Some(Accept(vec![
                qitem(TEXT_PLAIN_UTF_8),
                ])));
        test_header!(
            test4,
            vec![b"text/plain; charset=utf-8; q=0.5"],
            Some(Accept(vec![
                QualityItem::new(TEXT_PLAIN_UTF_8,
                    q(500)),
            ])));

        #[test]
        fn test_fuzzing1() {
            let raw: Raw = "chunk#;e".into();
            let header = Accept::parse_header(&raw);
            assert!(header.is_ok());
        }
    }
}

impl Accept {
    /// A constructor to easily create `Accept: */*`.
    pub fn star() -> Accept {
        Accept(vec![qitem(mime::STAR_STAR)])
    }

    /// A constructor to easily create `Accept: application/json`.
    pub fn json() -> Accept {
        Accept(vec![qitem(mime::APPLICATION_JSON)])
    }

    /// A constructor to easily create `Accept: text/*`.
    pub fn text() -> Accept {
        Accept(vec![qitem(mime::TEXT_STAR)])
    }

    /// A constructor to easily create `Accept: image/*`.
    pub fn image() -> Accept {
        Accept(vec![qitem(mime::IMAGE_STAR)])
    }
}


bench_header!(bench, Accept, { vec![b"text/plain; q=0.5, text/html".to_vec()] });
