use crate::source::Chunk;
use crate::{Filter, FilterResult, Lexer, Logos, Skip};

/// Trait used by the functions contained in the `Lexicon`.
///
/// # WARNING!
///
/// **This trait, and its methods, are not meant to be used outside of the
/// code produced by `#[derive(Logos)]` macro.**
pub trait LexerInternal<'source> {
    type Token: Logos<'source>;

    /// Read a chunk at current position.
    fn read<T: Chunk<'source>>(&self) -> Option<T>;

    /// Read a chunk at current position, offset by `n`.
    fn read_at<T: Chunk<'source>>(&self, n: usize) -> Option<T>;

    /// Unchecked read a byte at current position, offset by `n`.
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn read_byte_unchecked(&self, n: usize) -> u8;

    /// Checked read a byte at current position, offset by `n`.
    #[cfg(feature = "forbid_unsafe")]
    fn read_byte(&self, n: usize) -> u8;

    /// Test a chunk at current position with a closure.
    fn test<T: Chunk<'source>, F: FnOnce(T) -> bool>(&self, test: F) -> bool;

    /// Bump the position by `size`.
    fn bump_unchecked(&mut self, size: usize);

    /// Reset `token_start` to `token_end`.
    fn trivia(&mut self);

    /// Set the current token to appropriate `#[error]` variant.
    /// Guarantee that `token_end` is at char boundary for `&str`.
    fn error(&mut self);

    fn end(&mut self);

    fn set(
        &mut self,
        token: Result<
            Self::Token,
            <<Self as LexerInternal<'source>>::Token as Logos<'source>>::Error,
        >,
    );
}

pub trait CallbackResult<'s, P, T: Logos<'s>> {
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(P) -> T;
}

impl<'s, P, T: Logos<'s>> CallbackResult<'s, P, T> for P {
    #[inline]
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(P) -> T,
    {
        lex.set(Ok(c(self)))
    }
}

impl<'s, T: Logos<'s>> CallbackResult<'s, (), T> for bool {
    #[inline]
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(()) -> T,
    {
        match self {
            true => lex.set(Ok(c(()))),
            false => lex.set(Err(T::Error::default())),
        }
    }
}

impl<'s, P, T: Logos<'s>> CallbackResult<'s, P, T> for Option<P> {
    #[inline]
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(P) -> T,
    {
        match self {
            Some(product) => lex.set(Ok(c(product))),
            None => lex.set(Err(T::Error::default())),
        }
    }
}

impl<'s, P, E, T: Logos<'s>> CallbackResult<'s, P, T> for Result<P, E>
where
    E: Into<T::Error>,
{
    #[inline]
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(P) -> T,
    {
        match self {
            Ok(product) => lex.set(Ok(c(product))),
            Err(err) => lex.set(Err(err.into())),
        }
    }
}

impl<'s, T: Logos<'s>> CallbackResult<'s, (), T> for Skip {
    #[inline]
    fn construct<Constructor>(self, _: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(()) -> T,
    {
        lex.trivia();
        T::lex(lex);
    }
}

impl<'s, P, T: Logos<'s>> CallbackResult<'s, P, T> for Filter<P> {
    #[inline]
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(P) -> T,
    {
        match self {
            Filter::Emit(product) => lex.set(Ok(c(product))),
            Filter::Skip => {
                lex.trivia();
                T::lex(lex);
            }
        }
    }
}

impl<'s, P, E, T: Logos<'s>> CallbackResult<'s, P, T> for FilterResult<P, E>
where
    E: Into<T::Error>,
{
    fn construct<Constructor>(self, c: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(P) -> T,
    {
        match self {
            FilterResult::Emit(product) => lex.set(Ok(c(product))),
            FilterResult::Skip => {
                lex.trivia();
                T::lex(lex);
            }
            FilterResult::Error(err) => lex.set(Err(err.into())),
        }
    }
}

impl<'s, T: Logos<'s>> CallbackResult<'s, (), T> for T {
    #[inline]
    fn construct<Constructor>(self, _: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(()) -> T,
    {
        lex.set(Ok(self))
    }
}

impl<'s, T: Logos<'s>> CallbackResult<'s, (), T> for Result<T, T::Error> {
    #[inline]
    fn construct<Constructor>(self, _: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(()) -> T,
    {
        match self {
            Ok(product) => lex.set(Ok(product)),
            Err(err) => lex.set(Err(err)),
        }
    }
}

impl<'s, T: Logos<'s>> CallbackResult<'s, (), T> for Filter<T> {
    #[inline]
    fn construct<Constructor>(self, _: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(()) -> T,
    {
        match self {
            Filter::Emit(product) => lex.set(Ok(product)),
            Filter::Skip => {
                lex.trivia();
                T::lex(lex);
            }
        }
    }
}

impl<'s, T: Logos<'s>> CallbackResult<'s, (), T> for FilterResult<T, T::Error> {
    fn construct<Constructor>(self, _: Constructor, lex: &mut Lexer<'s, T>)
    where
        Constructor: Fn(()) -> T,
    {
        match self {
            FilterResult::Emit(product) => lex.set(Ok(product)),
            FilterResult::Skip => {
                lex.trivia();
                T::lex(lex);
            }
            FilterResult::Error(err) => lex.set(Err(err)),
        }
    }
}
