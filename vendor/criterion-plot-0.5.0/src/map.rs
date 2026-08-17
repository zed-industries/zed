//! Enum Maps

pub mod axis {
    use crate::Axis;

    const LENGTH: usize = 4;

    pub struct Items<'a, T>
    where
        T: 'a,
    {
        map: &'a Map<T>,
        state: Option<Axis>,
    }

    impl<'a, T> Iterator for Items<'a, T> {
        type Item = (Axis, &'a T);

        fn next(&mut self) -> Option<(Axis, &'a T)> {
            while let Some(key) = self.state {
                self.state = key.next();

                if let Some(value) = self.map.get(key) {
                    return Some((key, value));
                }
            }

            None
        }
    }

    pub struct Map<T>([Option<T>; LENGTH]);

    impl<T> Default for Map<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Map<T> {
        pub fn new() -> Map<T> {
            Map([None, None, None, None])
        }

        pub fn contains_key(&self, key: Axis) -> bool {
            self.0[key as usize].is_some()
        }

        pub fn get(&self, key: Axis) -> Option<&T> {
            self.0[key as usize].as_ref()
        }

        pub fn get_mut(&mut self, key: Axis) -> Option<&mut T> {
            self.0[key as usize].as_mut()
        }

        pub fn insert(&mut self, key: Axis, value: T) -> Option<T> {
            let key = key as usize;
            let old = self.0[key].take();

            self.0[key] = Some(value);

            old
        }

        pub fn iter(&self) -> Items<T> {
            Items {
                map: self,
                state: Some(Axis::BottomX),
            }
        }
    }

    impl<T> Clone for Map<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Map<T> {
            Map([
                self.0[0].clone(),
                self.0[1].clone(),
                self.0[2].clone(),
                self.0[3].clone(),
            ])
        }
    }
}

pub mod grid {
    use crate::Grid;

    const LENGTH: usize = 2;

    pub struct Items<'a, T>
    where
        T: 'a,
    {
        map: &'a Map<T>,
        state: Option<Grid>,
    }

    impl<'a, T> Iterator for Items<'a, T> {
        type Item = (Grid, &'a T);

        fn next(&mut self) -> Option<(Grid, &'a T)> {
            while let Some(key) = self.state {
                self.state = key.next();

                if let Some(value) = self.map.get(key) {
                    return Some((key, value));
                }
            }

            None
        }
    }

    pub struct Map<T>([Option<T>; LENGTH]);

    impl<T> Map<T> {
        pub fn new() -> Map<T> {
            Map([None, None])
        }

        pub fn contains_key(&self, key: Grid) -> bool {
            self.0[key as usize].is_some()
        }

        pub fn get(&self, key: Grid) -> Option<&T> {
            self.0[key as usize].as_ref()
        }

        pub fn get_mut(&mut self, key: Grid) -> Option<&mut T> {
            self.0[key as usize].as_mut()
        }

        pub fn insert(&mut self, key: Grid, value: T) -> Option<T> {
            let key = key as usize;
            let old = self.0[key].take();

            self.0[key] = Some(value);

            old
        }

        pub fn iter(&self) -> Items<T> {
            Items {
                map: self,
                state: Some(Grid::Major),
            }
        }
    }

    impl<T> Clone for Map<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Map<T> {
            Map([self.0[0].clone(), self.0[1].clone()])
        }
    }

    impl<T> Default for Map<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
