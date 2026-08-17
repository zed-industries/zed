use crate::tree::Node;
use crate::{InsertError, MatchError, Params};

/// A URL router.
///
/// See [the crate documentation](crate) for details.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Router<T> {
    root: Node<T>,
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self {
            root: Node::default(),
        }
    }
}

impl<T> Router<T> {
    /// Construct a new router.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use matchit::Router;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut router = Router::new();
    /// router.insert("/home", "Welcome!")?;
    /// router.insert("/users/:id", "A User")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert(&mut self, route: impl Into<String>, value: T) -> Result<(), InsertError> {
        self.root.insert(route, value)
    }

    /// Tries to find a value in the router matching the given path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use matchit::Router;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut router = Router::new();
    /// router.insert("/home", "Welcome!")?;
    ///
    /// let matched = router.at("/home").unwrap();
    /// assert_eq!(*matched.value, "Welcome!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn at<'m, 'p>(&'m self, path: &'p str) -> Result<Match<'m, 'p, &'m T>, MatchError> {
        match self.root.at(path.as_bytes()) {
            Ok((value, params)) => Ok(Match {
                // SAFETY: We only expose &mut T through &mut self
                value: unsafe { &*value.get() },
                params,
            }),
            Err(e) => Err(e),
        }
    }

    /// Tries to find a value in the router matching the given path,
    /// returning a mutable reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use matchit::Router;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut router = Router::new();
    /// router.insert("/", 1)?;
    ///
    /// *router.at_mut("/").unwrap().value += 1;
    /// assert_eq!(*router.at("/").unwrap().value, 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn at_mut<'m, 'p>(
        &'m mut self,
        path: &'p str,
    ) -> Result<Match<'m, 'p, &'m mut T>, MatchError> {
        match self.root.at(path.as_bytes()) {
            Ok((value, params)) => Ok(Match {
                // SAFETY: We have &mut self
                value: unsafe { &mut *value.get() },
                params,
            }),
            Err(e) => Err(e),
        }
    }

    #[cfg(feature = "__test_helpers")]
    pub fn check_priorities(&self) -> Result<u32, (u32, u32)> {
        self.root.check_priorities()
    }
}

/// A successful match consisting of the registered value
/// and URL parameters, returned by [`Router::at`](Router::at).
#[derive(Debug)]
pub struct Match<'k, 'v, V> {
    /// The value stored under the matched node.
    pub value: V,
    /// The route parameters. See [parameters](crate#parameters) for more details.
    pub params: Params<'k, 'v>,
}
