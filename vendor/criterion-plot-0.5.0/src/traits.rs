//! Traits

/// Overloaded `configure` method
pub trait Configure<This> {
    /// The properties of what's being configured
    type Properties;

    /// Configure some set of properties
    fn configure<F>(&mut self, this: This, function: F) -> &mut Self
    where
        F: FnOnce(&mut Self::Properties) -> &mut Self::Properties;
}

/// Types that can be plotted
pub trait Data {
    /// Convert the type into a double precision float
    fn f64(self) -> f64;
}

/// Overloaded `plot` method
pub trait Plot<This> {
    /// The properties associated to the plot
    type Properties;

    /// Plots some `data` with some `configuration`
    fn plot<F>(&mut self, this: This, function: F) -> &mut Self
    where
        F: FnOnce(&mut Self::Properties) -> &mut Self::Properties;
}

/// Overloaded `set` method
pub trait Set<T> {
    /// Sets some property
    fn set(&mut self, value: T) -> &mut Self;
}
