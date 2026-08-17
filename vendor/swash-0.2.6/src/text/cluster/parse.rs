use super::{
    super::Script, complex::ComplexState, myanmar::MyanmarState, simple::SimpleState, CharCluster,
    Token,
};

/// Parser that accepts a sequence of characters and outputs character clusters.
pub struct Parser<I> {
    inner: Inner<I>,
}

// enum Inner<I> {
//     Simple(SimpleClusters<Filter<I>>),
//     Myanmar(MyanmarClusters<Filter<I>>),
//     Complex(ComplexClusters<Filter<I>>),
// }

enum Inner<I> {
    Simple(SimpleState<I>),
    Myanmar(MyanmarState<I>),
    Complex(ComplexState<I>),
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token> + Clone,
{
    /// Creates a new cluster parser for the specified script and iterator
    /// over tokens.
    pub fn new(script: Script, tokens: I) -> Self {
        Self {
            inner: if script.is_complex() {
                if script == Script::Myanmar {
                    Inner::Myanmar(MyanmarState::new(tokens))
                } else {
                    Inner::Complex(ComplexState::new(script, tokens))
                }
            } else {
                Inner::Simple(SimpleState::new(tokens))
            },
        }
    }

    /// Parses the next cluster.
    #[inline]
    pub fn next(&mut self, cluster: &mut CharCluster) -> bool {
        cluster.clear();
        match self.inner {
            Inner::Simple(ref mut c) => c.next(cluster),
            Inner::Myanmar(ref mut c) => c.next(cluster),
            Inner::Complex(ref mut c) => c.next(cluster),
        }
    }
}
