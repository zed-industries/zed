//! Builder typestates.

/// Start of URI/IRI reference.
pub struct Start(());
/// Start of URI/IRI.
pub struct NonRefStart(());
/// End of scheme.
pub struct SchemeEnd(());
/// Start of authority.
pub struct AuthorityStart(());
/// End of userinfo.
pub struct UserinfoEnd(());
/// End of host.
pub struct HostEnd(());
/// End of port.
pub struct PortEnd(());
/// End of authority.
pub struct AuthorityEnd(());
/// End of path.
pub struct PathEnd(());
/// End of query.
pub struct QueryEnd(());
/// End of fragment
pub struct FragmentEnd(());
/// End of URI/IRI (reference).
pub struct End(());

/// Indicates the next possible state.
pub trait To<T> {}
/// Indicates the next possible state to advance to.
pub trait AdvanceTo<T>: To<T> {}

macro_rules! impl_many {
    ($trait:ident for $($x:ty => $($y:ty),+)*) => {
        $($(
            impl $trait<$y> for $x {}
        )+)*
    };
}

impl_many! { To for
    Start => SchemeEnd, AuthorityStart, PathEnd
    NonRefStart => SchemeEnd
    SchemeEnd => AuthorityStart, PathEnd
    AuthorityStart => UserinfoEnd, HostEnd
    UserinfoEnd => HostEnd
    HostEnd => PortEnd, AuthorityEnd
    PortEnd => AuthorityEnd
    AuthorityEnd => PathEnd
    PathEnd => QueryEnd, FragmentEnd, End
    QueryEnd => FragmentEnd, End
    FragmentEnd => End
}

impl<S: To<AuthorityStart>> To<AuthorityEnd> for S {}

impl_many! { AdvanceTo for
    Start => SchemeEnd, AuthorityEnd
    SchemeEnd => AuthorityEnd
    AuthorityStart => UserinfoEnd
    HostEnd => PortEnd
    PathEnd => QueryEnd, FragmentEnd
    QueryEnd => FragmentEnd
}
