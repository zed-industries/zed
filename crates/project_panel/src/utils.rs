pub(crate) struct ReversibleIterable<It> {
    pub(crate) it: It,
    pub(crate) reverse: bool,
}

impl<T> ReversibleIterable<T> {
    pub(crate) fn new(it: T, reverse: bool) -> Self {
        Self { it, reverse }
    }
}

impl<It, Item> ReversibleIterable<It>
where
    It: Iterator<Item = Item>,
{
    pub(crate) fn find_single_ended<F>(mut self, pred: F) -> Option<Item>
    where
        F: FnMut(&Item) -> bool,
    {
        if self.reverse {
            self.it.filter(pred).last()
        } else {
            self.it.find(pred)
        }
    }
}

impl<It, Item> ReversibleIterable<It>
where
    It: DoubleEndedIterator<Item = Item>,
{
    pub(crate) fn find<F>(mut self, mut pred: F) -> Option<Item>
    where
        F: FnMut(&Item) -> bool,
    {
        if self.reverse {
            self.it.rfind(|x| pred(x))
        } else {
            self.it.find(|x| pred(x))
        }
    }
}
