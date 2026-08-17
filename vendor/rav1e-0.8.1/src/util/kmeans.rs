// Copyright (c) 2022-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

/// Find k-means for a sorted slice of integers that can be summed in `i64`.
pub fn kmeans<T, const K: usize>(data: &[T]) -> [T; K]
where
  T: Copy,
  T: Into<i64>,
  T: PartialEq,
  T: PartialOrd,
  i64: TryInto<T>,
  <i64 as std::convert::TryInto<T>>::Error: std::fmt::Debug,
{
  let mut low = [0; K];
  for (i, val) in low.iter_mut().enumerate() {
    *val = (i * (data.len() - 1)) / (K - 1);
  }
  let mut means = low.map(|i| unsafe { *data.get_unchecked(i) });
  let mut high = low;
  let mut sum = [0i64; K];
  high[K - 1] = data.len();
  sum[K - 1] = means[K - 1].into();

  // Constrain complexity to O(n log n)
  let limit = 2 * (usize::BITS - data.len().leading_zeros());
  for _ in 0..limit {
    for (i, (threshold, (low, high))) in (means.iter().skip(1).zip(&means))
      .map(|(&c1, &c2)| unsafe {
        ((c1.into() + c2.into() + 1) >> 1).try_into().unwrap_unchecked()
      })
      .zip(low.iter_mut().skip(1).zip(&mut high))
      .enumerate()
    {
      unsafe {
        scan(high, low, sum.get_unchecked_mut(i..=i + 1), data, threshold);
      }
    }
    let mut changed = false;
    for (((m, sum), high), low) in
      means.iter_mut().zip(&sum).zip(&high).zip(&low)
    {
      let count = (high - low) as i64;
      if count == 0 {
        continue;
      }
      let new_mean = unsafe {
        ((sum + (count >> 1)).saturating_div(count))
          .try_into()
          .unwrap_unchecked()
      };
      changed |= *m != new_mean;
      *m = new_mean;
    }
    if !changed {
      break;
    }
  }

  means
}

#[inline(never)]
unsafe fn scan<T>(
  high: &mut usize, low: &mut usize, sum: &mut [i64], data: &[T], t: T,
) where
  T: Copy,
  T: Into<i64>,
  T: PartialEq,
  T: PartialOrd,
{
  let mut n = *high;
  let mut s = *sum.get_unchecked(0);
  for &d in data.get_unchecked(..n).iter().rev().take_while(|&d| *d > t) {
    s -= d.into();
    n -= 1;
  }
  for &d in data.get_unchecked(n..).iter().take_while(|&d| *d <= t) {
    s += d.into();
    n += 1;
  }
  *high = n;
  *sum.get_unchecked_mut(0) = s;

  let mut n = *low;
  let mut s = *sum.get_unchecked(1);
  for &d in data.get_unchecked(n..).iter().take_while(|&d| *d < t) {
    s -= d.into();
    n += 1;
  }
  for &d in data.get_unchecked(..n).iter().rev().take_while(|&d| *d >= t) {
    s += d.into();
    n -= 1;
  }
  *low = n;
  *sum.get_unchecked_mut(1) = s;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn three_means() {
    let mut data = [1, 2, 3, 10, 11, 12, 20, 21, 22];
    data.sort_unstable();
    let centroids = kmeans(&data);
    assert_eq!(centroids, [2, 11, 21]);
  }

  #[test]
  fn four_means() {
    let mut data = [30, 31, 32, 1, 2, 3, 10, 11, 12, 20, 21, 22];
    data.sort_unstable();
    let centroids = kmeans(&data);
    assert_eq!(centroids, [2, 11, 21, 31]);
  }
}
