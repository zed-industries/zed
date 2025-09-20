// To discuss: What to send to the new endpoint? Thinking it'd make sense to put `prompt.rs` from
// `zeta_context.rs` in cloud.
//
// * Run excerpt selection at several different sizes, send the largest size with offsets within for
// the smaller sizes.
//
// * Longer event history.
//
// * Many more snippets than could fit in model context - allows ranking experimentation.

pub struct Zeta2Request {
    pub event_history: Vec<Event>,
    pub excerpt: String,
    pub excerpt_subsets: Vec<Zeta2ExcerptSubset>,
    /// Within `excerpt`
    pub cursor_position: usize,
    pub signatures: Vec<String>,
    pub retrieved_declarations: Vec<ReferencedDeclaration>,
}

pub struct Zeta2ExcerptSubset {
    /// Within `excerpt` text.
    pub excerpt_range: Range<usize>,
    /// Within `signatures`.
    pub parent_signatures: Vec<usize>,
}

pub struct ReferencedDeclaration {
    pub text: Arc<str>,
    /// Range within `text`
    pub signature_range: Range<usize>,
    /// Indices within `signatures`.
    pub parent_signatures: Vec<usize>,
    // A bunch of score metrics
}
