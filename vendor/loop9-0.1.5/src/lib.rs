use core::ops::Deref;
use imgref::ImgRef;

/// Previous, current, and next pixel or row
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
#[repr(C)]
pub struct Triple<T> {
    pub prev: T,
    pub curr: T,
    pub next: T,
}

impl<T> Triple<T> {
    #[must_use]
    #[inline(always)]
    pub fn new(prev: T, curr: T, next: T) -> Self {
        Triple { prev, curr, next }
    }
}

impl<T> AsRef<[T]> for Triple<T> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self as *const Triple<T> as *const T, 3)
        }
    }
}

impl<T> Deref for Triple<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Copy> Triple<T> {
    /// Add the next item, and shift others (prev is gone, prev = current)
    /// If the item is `None`, it'll copy the last one instead.
    #[must_use]
    #[inline(always)]
    pub fn advance(self, next: Option<T>) -> Self {
        Triple {
            prev: self.curr,
            curr: self.next,
            next: next.unwrap_or(self.next),
        }
    }
}

/// Loop over 9 neighboring pixels in the image described by [`ImgRef`] (`Img.as_ref()`)
///
/// The callback is: (`x`, `y`, `previous_row`, `current_row`, `next_row`)
///
/// This function will never panic, if your callback doesn't panic.
#[inline(always)]
pub fn loop9_img<Pixel, Callback>(img: ImgRef<'_, Pixel>, cb: Callback)
    where Pixel: Copy, Callback: FnMut(usize, usize, Triple<Pixel>,Triple<Pixel>,Triple<Pixel>)
{
    loop9(img, 0, 0, img.width(), img.height(), cb);
}

/// Loop over 9 neighboring pixels in the left/top/width/height fragment of the image described by [`ImgRef`] (`Img.as_ref()`)
///
/// The callback is: (`x`, `y`, `previous_row`, `current_row`, `next_row`)
///
/// This function will never panic, if your callback doesn't panic.
pub fn loop9<Pixel, Callback>(img: ImgRef<'_, Pixel>, left: usize, top: usize, width: usize, height: usize, mut cb: Callback)
    where Pixel: Copy, Callback: FnMut(usize, usize, Triple<Pixel>,Triple<Pixel>,Triple<Pixel>)
{
    let max_width = img.width();
    let max_height = img.height();
    let stride = img.stride();
    if stride == 0 || max_height == 0 || max_width == 0 {
        return;
    }
    let data = img.buf();
    let t = top.min(max_height-1) * stride;
    let start_row = if let Some(r) = data.get(t..t+max_width) { r } else { return };
    if start_row.is_empty() { return; }
    let mut row = Triple {
        prev: start_row,
        curr: start_row,
        next: start_row,
    };
    for y in top..top+height {
        row = row.advance({
            let t = (y+1) * stride;
            data.get(t..t+max_width)
        });
        if row.prev.is_empty() || row.curr.is_empty() || row.next.is_empty() {
            return;
        }
        let mut tp;
        let mut tn = row.prev[left.min(row.prev.len()-1)];
        let mut tc = row.prev[left.saturating_sub(1).min(row.prev.len()-1)];
        let mut mp;
        let mut mn = row.curr[left.min(row.curr.len()-1)];
        let mut mc = row.curr[left.saturating_sub(1).min(row.curr.len()-1)];
        let mut bp;
        let mut bn = row.next[left.min(row.next.len()-1)];
        let mut bc = row.next[left.saturating_sub(1).min(row.next.len()-1)];
        for x in left..left+width {
            tp = tc;
            tc = tn;
            tn = row.prev.get(x+1).copied().unwrap_or(tc);
            mp = mc;
            mc = mn;
            mn = row.curr.get(x+1).copied().unwrap_or(mc);
            bp = bc;
            bc = bn;
            bn = row.next.get(x+1).copied().unwrap_or(bc);
            cb(x-left, y-top, Triple::new(tp, tc, tn), Triple::new(mp, mc, mn), Triple::new(bp, bc, bn));
        }
    }
}


#[test]
fn test_oob() {
    use imgref::Img;
    let img = Img::new(vec![0; 5*4], 5, 4);
    for w in 1..8 {
        for h in 1..8 {
            for x in 0..8 {
                for y in 0..8 {
                    let mut n = 0;
                    loop9(img.as_ref(), x,y,w,h, |_x,_y,_top,_mid,_bot| { n += 1 });
                    assert_eq!(n, w*h, "{x},{y},{w},{h}");
                }
            }
        }
    }
}

#[test]
fn test_loop9() {
    use imgref::Img;

    let src = vec![
         1, 2, 3, 4, 0,
         5, 6, 7, 8, 0,
         9,10,11,12, 0,
        13,14,15,16, 0,
    ];
    let img = Img::new_stride(src.clone(), 4, 4, 5);
    assert_eq!(4, img.width());
    assert_eq!(5, img.stride());
    assert_eq!(4, img.height());

    let check = |l,t,w,h,exp: &[_]|{
        let mut res = Vec::new();
        loop9(img.as_ref(), l,t,w,h, |_x,_y,_top,mid,_bot| res.push(mid.curr));
        assert_eq!(exp, res);
    };

    check(0,0,4,4, &[
         1, 2, 3, 4,
         5, 6, 7, 8,
         9,10,11,12,
        13,14,15,16,
    ]);

    check(0,0,4,1, &[1, 2, 3, 4]);

    check(0,3,4,1, &[13,14,15,16]);

    check(0,0,3,3, &[
         1, 2, 3,
         5, 6, 7,
         9,10,11,
    ]);

    check(0,0,1,1, &[1]);
    check(1,0,1,1, &[2]);
    check(2,0,1,1, &[3]);
    check(3,0,1,1, &[4]);

    check(1,0,3,4,&[
         2, 3, 4,
         6, 7, 8,
        10,11,12,
        14,15,16,
    ]);
}
