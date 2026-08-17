/// An error type returned by `Sink::try_send`, when the sink is full, or is closed.
#[derive(Debug, PartialEq, Eq)]
pub enum TrySendError<T> {
    /// The sink could accept the item at a later time
    Pending(T),
    /// The sink is closed, and will never accept the item
    Rejected(T),
}

impl<T> std::fmt::Display for TrySendError<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self))?;

        Ok(())
    }
}

impl<T> std::error::Error for TrySendError<T> where T: std::fmt::Debug {}

/// An error type returned by `Sink::send`, if the sink is closed while a send is in progress.
#[derive(Debug, PartialEq, Eq)]
pub struct SendError<T>(pub T);

impl<T> std::fmt::Display for SendError<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self))?;

        Ok(())
    }
}

impl<T> std::error::Error for SendError<T> where T: std::fmt::Debug {}
