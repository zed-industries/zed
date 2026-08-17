use super::internal::{aat::morx, raw_tag};
use super::util::*;

pub use morx::chains;
use morx::Chains;

#[derive(Copy, Clone)]
pub struct Features<'a> {
    chains: Chains<'a>,
    features: Option<morx::Features<'a>>,
    kern: bool,
    seen: SeenFeatures,
}

impl<'a> Features<'a> {
    pub fn new(chains: Chains<'a>, kern: bool) -> Self {
        Self {
            chains,
            features: None,
            kern,
            seen: SeenFeatures::new(),
        }
    }
}

impl<'a> Iterator for Features<'a> {
    type Item = (u32, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.features.is_none() {
                if let Some(chain) = self.chains.next() {
                    self.features = Some(chain.features());
                } else if self.kern {
                    let tag = raw_tag(b"kern");
                    let (_, desc) = desc_from_at(tag).unwrap_or((0, "Kerning"));
                    self.kern = false;
                    return Some((tag, desc));
                } else {
                    return None;
                }
            }
            if let Some(features) = &mut self.features {
                if let Some(feature) = features.next() {
                    if let Some((index, tag, desc)) =
                        desc_from_aat(feature.selector, feature.setting_selector)
                    {
                        if self.seen.mark(index) {
                            return Some((tag, desc));
                        }
                    }
                } else {
                    self.features = None;
                    continue;
                }
            }
        }
    }
}

pub type OnceItem<'a> = Option<Item<'a>>;
// #[derive(Copy, Clone)]
// pub struct OnceItem<'a> {
//     pub item: Item<'a>,
//     pub given: bool,
// }

// impl<'a> OnceItem<'a> {
//     pub fn new(item: Item<'a>) -> Self {
//         Self { item, given: false }
//     }
// }

// impl<'a> Iterator for OnceItem<'a> {
//     type Item = Item<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.given {
//             None
//         } else {
//             self.given = true;
//             Some(self.item)
//         }
//     }
// }

#[derive(Copy, Clone)]
pub struct Item<'a> {
    pub chains: Chains<'a>,
    pub kern: bool,
}
