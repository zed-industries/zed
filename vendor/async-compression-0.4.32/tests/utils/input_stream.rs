use futures::stream::Stream;
use futures_test::stream::StreamTestExt as _;
use proptest_derive::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct InputStream(Vec<Vec<u8>>);

impl InputStream {
    pub fn new(input: Vec<Vec<u8>>) -> Self {
        InputStream(input)
    }

    pub fn as_ref(&self) -> &[Vec<u8>] {
        &self.0
    }

    pub fn stream(&self) -> impl Stream<Item = Vec<u8>> {
        // The resulting stream here will interleave empty chunks before and after each chunk, and
        // then interleave a `Poll::Pending` between each yielded chunk, that way we test the
        // handling of these two conditions in every point of the tested stream.
        futures::stream::iter(
            self.0
                .clone()
                .into_iter()
                .flat_map(|bytes| vec![vec![], bytes])
                .chain(Some(vec![])),
        )
        .interleave_pending()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.iter().flatten().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(Vec::len).sum()
    }
}

impl<I> From<I> for InputStream
where
    I: IntoIterator,
    I::Item: Into<Vec<u8>>,
{
    fn from(input: I) -> InputStream {
        Self::new(input.into_iter().map(|b| b.into()).collect())
    }
}
