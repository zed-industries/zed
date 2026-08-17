// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use arg_enum_proc_macro::ArgEnum;
use std::env;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, ArgEnum)]
pub enum CpuFeatureLevel {
  RUST,
  NEON,
}

impl CpuFeatureLevel {
  #[cfg(test)]
  pub(crate) const fn all() -> &'static [Self] {
    use CpuFeatureLevel::*;
    &[RUST, NEON]
  }

  pub const fn len() -> usize {
    CpuFeatureLevel::NEON as usize + 1
  }

  #[inline(always)]
  pub fn as_index(self) -> usize {
    self as usize
  }
}

impl Default for CpuFeatureLevel {
  fn default() -> CpuFeatureLevel {
    let detected = CpuFeatureLevel::NEON;
    let manual: CpuFeatureLevel = match env::var("RAV1E_CPU_TARGET") {
      Ok(feature) => CpuFeatureLevel::from_str(&feature).unwrap_or(detected),
      Err(_e) => detected,
    };
    if manual > detected {
      detected
    } else {
      manual
    }
  }
}

// Create a static lookup table for CPUFeatureLevel enums
// Note: keys are CpuFeatureLevels without any prefix (no CpuFeatureLevel::)
macro_rules! cpu_function_lookup_table {
  // version for default visibility
  ($name:ident: [$type:ty], default: $empty:expr, [$(($key:ident, $value:expr)),*]) => {
    static $name: [$type; crate::cpu_features::CpuFeatureLevel::len()] = {
      use crate::cpu_features::CpuFeatureLevel;
      #[allow(unused_mut)]
      let mut out: [$type; CpuFeatureLevel::len()] = [$empty; CpuFeatureLevel::len()];

      // Can't use out[0][.] == $empty in static as of rust 1.40
      #[allow(unused_mut)]
      let mut set: [bool; CpuFeatureLevel::len()] = [false; CpuFeatureLevel::len()];

      #[allow(unused_imports)]
      use CpuFeatureLevel::*;
      $(
        out[$key as usize] = $value;
        set[$key as usize] = true;
      )*
      cpu_function_lookup_table!(waterfall_cpu_features(out, set, [NEON]));
      out
    };
  };

  ($pub:vis, $name:ident: [$type:ty], default: $empty:expr, [$(($key:ident, $value:expr)),*]) => {
    $pub cpu_function_lookup_table!($name: [$type], default: $empty, [$(($key, $value)),*]);
  };
  // Fill empty output functions with the existent functions they support.
  // cpus should be in order of lowest cpu level to highest
  // Used like an internal function
  // Put in here to avoid adding more public macros
  (waterfall_cpu_features($out:ident, $set:ident, [$($cpu:ident),*])) => {
    // Use an array to emulate if statements (not supported in static as of
    // rust 1.40). Setting best[0] (false) and best[1] (true) is equivalent to
    // doing nothing and overriding our value respectively.
    #[allow(unused_assignments)]
    let mut best = [$out[0], $out[0]];
    $(
      // If the current entry has a function, update out best function.
      best[$set[$cpu as usize] as usize] = $out[$cpu as usize];
      // Update our current entry. Does nothing if it already had a function.
      $out[$cpu as usize] = best[1];
    )*
  };


  // use $name_$key as our values
  ($pub:vis, $name:ident: [$type:ty], default: $empty:expr, [$($key:ident),*]) => {
    paste::item!{
      cpu_function_lookup_table!(
        $pub, $name: [$type], default: $empty, [$(($key, [<$name _$key>])),*]
      );
    }
  };

  // version for default visibility
  ($name:ident: [$type:ty], default: $empty:expr, [$($key:ident),*]) => {
    paste::item!{
      cpu_function_lookup_table!(
        $name: [$type], default: $empty, [$(($key, [<$name _$key>])),*]
      );
    }
  };
}
