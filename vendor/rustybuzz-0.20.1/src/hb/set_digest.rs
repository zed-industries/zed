use ttf_parser::GlyphId;

// To make things easier, we don't have the generic parameter mask_t,
// and assume we always use u64, since this is what is also used in
// harfbuzz.
type mask_t = u64;

pub trait hb_set_digest_ext: Clone {
    type A;
    // Instead of `init()`
    fn new() -> Self;
    fn full() -> Self;
    fn add(&mut self, g: GlyphId);
    fn add_array(&mut self, array: impl IntoIterator<Item = GlyphId> + Clone);
    fn add_range(&mut self, a: GlyphId, b: GlyphId) -> bool;
    fn may_have(&self, o: &Self::A) -> bool;
    fn may_have_glyph(&self, g: GlyphId) -> bool;
}

#[derive(Clone)]
pub struct hb_set_digest_bits_pattern_t<const shift: u8> {
    mask: mask_t,
}

impl<const shift: u8> hb_set_digest_bits_pattern_t<shift> {
    const fn mask_bytes() -> mask_t {
        core::mem::size_of::<mask_t>() as mask_t
    }

    const fn mask_bits() -> mask_t {
        (core::mem::size_of::<mask_t>() * 8) as mask_t
    }

    fn mask_for(g: GlyphId) -> mask_t {
        1 << ((g.0 as mask_t >> shift) & (hb_set_digest_bits_pattern_t::<shift>::mask_bits() - 1))
    }

    const fn num_bits() -> usize {
        let mut num = 0;

        if hb_set_digest_bits_pattern_t::<shift>::mask_bytes() >= 1 {
            num += 3;
        }

        if hb_set_digest_bits_pattern_t::<shift>::mask_bytes() >= 2 {
            num += 1;
        }

        if hb_set_digest_bits_pattern_t::<shift>::mask_bytes() >= 4 {
            num += 1;
        }

        if hb_set_digest_bits_pattern_t::<shift>::mask_bytes() >= 8 {
            num += 1;
        }

        if hb_set_digest_bits_pattern_t::<shift>::mask_bytes() >= 16 {
            num += 1;
        }

        num
    }
}

impl<const shift: u8> hb_set_digest_ext for hb_set_digest_bits_pattern_t<shift> {
    type A = hb_set_digest_bits_pattern_t<shift>;

    fn new() -> Self {
        debug_assert!((shift as usize) < core::mem::size_of::<GlyphId>() * 8);
        debug_assert!((shift as usize + Self::num_bits()) < core::mem::size_of::<GlyphId>() * 8);
        Self { mask: 0 }
    }

    fn full() -> Self {
        Self { mask: mask_t::MAX }
    }

    fn add(&mut self, g: GlyphId) {
        self.mask |= hb_set_digest_bits_pattern_t::<shift>::mask_for(g);
    }

    fn add_array(&mut self, array: impl IntoIterator<Item = GlyphId> + Clone) {
        for el in array {
            self.add(el);
        }
    }

    fn add_range(&mut self, a: GlyphId, b: GlyphId) -> bool {
        if self.mask == mask_t::MAX {
            return false;
        }

        if (b.0 as mask_t >> shift) - (a.0 as mask_t >> shift)
            >= hb_set_digest_bits_pattern_t::<shift>::mask_bits() - 1
        {
            self.mask = mask_t::MAX;
            false
        } else {
            let ma = hb_set_digest_bits_pattern_t::<shift>::mask_for(a);
            let mb = hb_set_digest_bits_pattern_t::<shift>::mask_for(b);
            self.mask |= mb + mb.wrapping_sub(ma) - mask_t::from(mb < ma);
            true
        }
    }

    fn may_have(&self, o: &hb_set_digest_bits_pattern_t<shift>) -> bool {
        self.mask & o.mask != 0
    }

    fn may_have_glyph(&self, g: GlyphId) -> bool {
        self.mask & hb_set_digest_bits_pattern_t::<shift>::mask_for(g) != 0
    }
}

#[derive(Clone)]
pub struct hb_set_digest_combiner_t<head_t, tail_t>
where
    head_t: hb_set_digest_ext,
    tail_t: hb_set_digest_ext,
{
    head: head_t,
    tail: tail_t,
}

impl<head_t, tail_t> hb_set_digest_ext for hb_set_digest_combiner_t<head_t, tail_t>
where
    head_t: hb_set_digest_ext<A = head_t>,
    tail_t: hb_set_digest_ext<A = tail_t>,
{
    type A = hb_set_digest_combiner_t<head_t, tail_t>;

    fn new() -> Self {
        Self {
            head: head_t::new(),
            tail: tail_t::new(),
        }
    }

    fn full() -> Self {
        Self {
            head: head_t::full(),
            tail: tail_t::full(),
        }
    }

    fn add(&mut self, g: GlyphId) {
        self.head.add(g);
        self.tail.add(g);
    }

    fn add_array(&mut self, array: impl IntoIterator<Item = GlyphId> + Clone) {
        // TODO: Is this expensive if someone passes e.g. a vector?
        self.head.add_array(array.clone());
        self.tail.add_array(array);
    }

    fn add_range(&mut self, a: GlyphId, b: GlyphId) -> bool {
        let first = self.head.add_range(a, b);
        let second = self.tail.add_range(a, b);
        first || second
    }

    fn may_have(&self, o: &Self::A) -> bool {
        self.head.may_have(&o.head) && self.tail.may_have(&o.tail)
    }

    fn may_have_glyph(&self, g: GlyphId) -> bool {
        self.head.may_have_glyph(g) && self.tail.may_have_glyph(g)
    }
}

#[rustfmt::skip]
pub type hb_set_digest_t = hb_set_digest_combiner_t<
    hb_set_digest_bits_pattern_t<4>,
    hb_set_digest_combiner_t<
        hb_set_digest_bits_pattern_t<0>,
        hb_set_digest_bits_pattern_t<9>
    >,
>;

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let mut set = hb_set_digest_t::new();

        set.add(GlyphId(2));
        assert!(set.may_have_glyph(GlyphId(2)))
    }

    #[test]
    fn test_multiple_1() {
        let mut set = hb_set_digest_t::new();

        set.add(GlyphId(2));
        set.add(GlyphId(10));
        set.add(GlyphId(300));
        set.add(GlyphId(255));
        assert!(set.may_have_glyph(GlyphId(2)));
        assert!(set.may_have_glyph(GlyphId(300)));
        assert!(set.may_have_glyph(GlyphId(10)));
        assert!(set.may_have_glyph(GlyphId(255)));
    }

    #[test]
    fn test_multiple_2() {
        let mut set = hb_set_digest_t::new();

        set.add(GlyphId(245));
        set.add(GlyphId(1060));
        set.add(GlyphId(300));
        set.add(GlyphId(599));
        assert!(set.may_have_glyph(GlyphId(245)));
        assert!(set.may_have_glyph(GlyphId(1060)));
        assert!(set.may_have_glyph(GlyphId(300)));
        assert!(set.may_have_glyph(GlyphId(599)));
    }

    #[test]
    fn test_range_1() {
        let mut set = hb_set_digest_t::new();

        set.add_range(GlyphId(10), GlyphId(12));
        assert!(set.may_have_glyph(GlyphId(10)));
        assert!(set.may_have_glyph(GlyphId(11)));
        assert!(set.may_have_glyph(GlyphId(12)));
    }

    #[test]
    fn test_range_2() {
        let mut set = hb_set_digest_t::new();

        set.add_range(GlyphId(15), GlyphId(20));
        assert!(set.may_have_glyph(GlyphId(15)));
        assert!(set.may_have_glyph(GlyphId(16)));
        assert!(set.may_have_glyph(GlyphId(17)));
        assert!(set.may_have_glyph(GlyphId(18)));
        assert!(set.may_have_glyph(GlyphId(19)));
        assert!(set.may_have_glyph(GlyphId(20)));
    }

    #[test]
    fn test_range_3() {
        let mut set = hb_set_digest_t::new();

        for i in 170..=239 {
            set.add(GlyphId(i));
        }
        assert!(set.may_have_glyph(GlyphId(200)));
    }

    #[test]
    fn test_complex() {
        let mut set = hb_set_digest_t::new();

        set.add_range(GlyphId(5670), GlyphId(5675));
        set.add(GlyphId(3));
        set.add(GlyphId(8769));
        set.add(GlyphId(10000));
        set.add_range(GlyphId(3456), GlyphId(3460));
        assert!(set.may_have_glyph(GlyphId(5670)));
        assert!(set.may_have_glyph(GlyphId(5675)));
        assert!(set.may_have_glyph(GlyphId(3)));
        assert!(set.may_have_glyph(GlyphId(8769)));
        assert!(set.may_have_glyph(GlyphId(10000)));
        assert!(set.may_have_glyph(GlyphId(3456)));
        assert!(set.may_have_glyph(GlyphId(3460)));
    }
}
