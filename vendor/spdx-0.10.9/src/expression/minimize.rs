use super::Expression;
use crate::{LicenseReq, Licensee};
use std::fmt;

/// Errors that can occur when trying to minimize the requirements for an [`Expression`]
#[derive(Debug, PartialEq, Eq)]
pub enum MinimizeError {
    /// More than `64` unique licensees satisfied a requirement in the [`Expression`]
    TooManyRequirements(usize),
    /// The list of licensees did not fully satisfy the requirements in the [`Expression`]
    RequirementsUnmet,
}

impl fmt::Display for MinimizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyRequirements(n) => write!(
                f,
                "the license expression required {n} licensees which exceeds the limit of 64",
            ),
            Self::RequirementsUnmet => {
                f.write_str("the expression was not satisfied by the provided list of licensees")
            }
        }
    }
}

impl std::error::Error for MinimizeError {
    fn description(&self) -> &str {
        match self {
            Self::TooManyRequirements(_) => "too many requirements in license expression",
            Self::RequirementsUnmet => {
                "the expression was not satisfied by the provided list of licensees"
            }
        }
    }
}

impl Expression {
    /// Given a set of [`Licensee`]s, attempts to find the minimum number that
    /// satisfy this [`Expression`].
    ///
    /// The list of licensees should be given in priority order, eg, if you wish
    /// to accept the `Apache-2.0` license if it is available, and the `MIT` if
    /// not, putting `Apache-2.0` before `MIT` will cause the ubiquitous
    /// `Apache-2.0 OR MIT` expression to minimize to just `Apache-2.0` as only
    /// 1 of the licenses is required, and `Apache-2.0` has priority.
    ///
    /// # Errors
    ///
    /// This method will fail if more than 64 unique licensees are satisfied by
    /// this expression, but such a case is unlikely in a real world scenario.
    /// The list of licensees must also actually satisfy this expression,
    /// otherwise it can't be minimized.
    ///
    /// # Example
    ///
    /// ```
    /// let expr = spdx::Expression::parse("Apache-2.0 OR MIT").unwrap();
    ///
    /// let apache_licensee = spdx::Licensee::parse("Apache-2.0").unwrap();
    /// assert_eq!(
    ///     expr.minimized_requirements([&apache_licensee, &spdx::Licensee::parse("MIT").unwrap()]).unwrap(),
    ///     vec![apache_licensee.into_req()],
    /// );
    /// ```
    pub fn minimized_requirements<'lic>(
        &self,
        accepted: impl IntoIterator<Item = &'lic Licensee>,
    ) -> Result<Vec<LicenseReq>, MinimizeError> {
        let found_set = {
            let mut found_set = smallvec::SmallVec::<[Licensee; 5]>::new();

            for lic in accepted {
                if !found_set.contains(lic)
                    && self.requirements().any(|ereq| lic.satisfies(&ereq.req))
                {
                    found_set.push(lic.clone());
                }
            }

            if found_set.len() > 64 {
                return Err(MinimizeError::TooManyRequirements(found_set.len()));
            }

            // Ensure that the licensees provided actually _can_ be accepted by
            // this expression
            if !self.evaluate(|ereq| found_set.iter().any(|lic| lic.satisfies(ereq))) {
                return Err(MinimizeError::RequirementsUnmet);
            }

            found_set
        };

        let set_size = (1 << found_set.len()) as u64;

        for mask in 1..=set_size {
            let eval_res = self.evaluate(|req| {
                for (ind, lic) in found_set.iter().enumerate() {
                    if mask & (1 << ind) != 0 && lic.satisfies(req) {
                        return true;
                    }
                }

                false
            });

            if eval_res {
                return Ok(found_set
                    .into_iter()
                    .enumerate()
                    .filter_map(|(ind, lic)| {
                        if mask & (1 << ind) == 0 {
                            None
                        } else {
                            Some(lic.into_req())
                        }
                    })
                    .collect());
            }
        }

        // This should be impossible, but would rather not panic
        Ok(found_set.into_iter().map(Licensee::into_req).collect())
    }
}
