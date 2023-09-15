pub struct Ordered<P> {
    pub order: u32,
    pub primitive: P,
}

impl<P> Ord for Ordered<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl<P> PartialOrd for Ordered<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<P> Eq for Ordered<P> {}

impl<P> PartialEq for Ordered<P> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}
