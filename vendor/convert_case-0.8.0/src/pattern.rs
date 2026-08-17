//! Functions for transforming a list of words.
//!
//! A pattern is a function that maps a list of words into another list
//! after changing the casing of each letter.  How a patterns mutates
//! each letter can be dependent on the word the letters are present in.

#[cfg(feature = "random")]
use rand::prelude::*;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use unicode_segmentation::UnicodeSegmentation;

mod word_pattern {
    use super::*;

    pub fn lowercase(word: &str) -> String {
        word.to_lowercase()
    }

    pub fn uppercase(word: &str) -> String {
        word.to_uppercase()
    }

    pub fn capital(word: &str) -> String {
        let mut graphemes = word.graphemes(true);

        if let Some(c) = graphemes.next() {
            [c.to_uppercase(), graphemes.as_str().to_lowercase()].concat()
        } else {
            String::new()
        }
    }

    pub fn toggle(word: &str) -> String {
        let mut graphemes = word.graphemes(true);

        if let Some(c) = graphemes.next() {
            [c.to_lowercase(), graphemes.as_str().to_uppercase()].concat()
        } else {
            String::new()
        }
    }
}

pub type Pattern = fn(&[&str]) -> Vec<String>;

/// The no-op pattern performs no mutations.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["Case", "CONVERSION", "library"],
///     pattern::noop(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn noop(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// Makes all words lowercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["case", "conversion", "library"],
///     pattern::lowercase(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn lowercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::lowercase(word))
        .collect()
}

/// Makes all words uppercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["CASE", "CONVERSION", "LIBRARY"],
///     pattern::uppercase(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn uppercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::uppercase(word))
        .collect()
}

/// Makes the first letter of each word uppercase
/// and the remaining letters of each word lowercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["Case", "Conversion", "Library"],
///     pattern::capital(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn capital(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::capital(word))
        .collect()
}

/// Makes the first word lowercase and the
/// remaining capitalized.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["case", "Conversion", "Library"],
///     pattern::camel(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn camel(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| {
            if i == 0 {
                word_pattern::lowercase(&word)
            } else {
                word_pattern::capital(&word)
            }
        })
        .collect()
}

/// Makes the first word capitalized and the
/// remaining lowercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["Case", "conversion", "library"],
///     pattern::sentence(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn sentence(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| {
            if i == 0 {
                word_pattern::capital(&word)
            } else {
                word_pattern::lowercase(&word)
            }
        })
        .collect()
}

/// Makes the first letter of each word lowercase
/// and the remaining letters of each word uppercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["cASE", "cONVERSION", "lIBRARY"],
///     pattern::toggle(&["Case", "CONVERSION", "library"])
/// );
/// ```
pub fn toggle(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::toggle(word))
        .collect()
}

/// Makes each letter of each word alternate
/// between lowercase and uppercase.  
///
/// It alternates across words,
/// which means the last letter of one word and the first letter of the
/// next will not be the same letter casing.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     vec!["cAsE", "cOnVeRsIoN", "lIbRaRy"],
///     pattern::alternating(&["Case", "CONVERSION", "library"])
/// );
/// assert_eq!(
///     vec!["aNoThEr", "ExAmPlE"],
///     pattern::alternating(&["Another", "Example"]),
/// );
/// ```
pub fn alternating(words: &[&str]) -> Vec<String> {
    let mut upper = false;
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    if letter.is_uppercase() || letter.is_lowercase() {
                        if upper {
                            upper = false;
                            letter.to_uppercase().to_string()
                        } else {
                            upper = true;
                            letter.to_lowercase().to_string()
                        }
                    } else {
                        letter.to_string()
                    }
                })
                .collect()
        })
        .collect()
}

/// Lowercases or uppercases each letter
/// uniformly randomly.  
///
/// This uses the `rand` crate and is only available with the "random"
/// feature.  This example will not pass the assertion due to randomness, but it used as an
/// example of what output is possible.
/// ```should_panic
/// # use convert_case::pattern;
/// # #[cfg(any(doc, feature = "random"))]
/// assert_eq!(
///     vec!["Case", "coNVeRSiOn", "lIBraRY"],
///     pattern::random(&["Case", "CONVERSION", "library"])
/// );
/// ```
#[cfg(feature = "random")]
pub fn random(words: &[&str]) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    if rng.gen::<f32>() > 0.5 {
                        letter.to_uppercase().to_string()
                    } else {
                        letter.to_lowercase().to_string()
                    }
                })
                .collect()
        })
        .collect()
}

/// Case each letter in random-like patterns.  
///
/// Instead of randomizing
/// each letter individually, it mutates each pair of characters
/// as either (Lowercase, Uppercase) or (Uppercase, Lowercase).  This generates
/// more "random looking" words.  A consequence of this algorithm for randomization
/// is that there will never be three consecutive letters that are all lowercase
/// or all uppercase.  This uses the `rand` crate and is only available with the "random"
/// feature.  This example will not pass the assertion due to randomness, but it used as an
/// example of what output is possible.
/// ```should_panic
/// # use convert_case::pattern;
/// # #[cfg(any(doc, feature = "random"))]
/// assert_eq!(
///     vec!["cAsE", "cONveRSioN", "lIBrAry"],
///     pattern::pseudo_random(&["Case", "CONVERSION", "library"]),
/// );
/// ```
#[cfg(feature = "random")]
pub fn pseudo_random(words: &[&str]) -> Vec<String> {
    let mut rng = rand::thread_rng();

    // Keeps track of when to alternate
    let mut alt: Option<bool> = None;
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    match alt {
                        // No existing pattern, start one
                        None => {
                            if rng.gen::<f32>() > 0.5 {
                                alt = Some(false); // Make the next char lower
                                letter.to_uppercase().to_string()
                            } else {
                                alt = Some(true); // Make the next char upper
                                letter.to_lowercase().to_string()
                            }
                        }
                        // Existing pattern, do what it says
                        Some(upper) => {
                            alt = None;
                            if upper {
                                letter.to_uppercase().to_string()
                            } else {
                                letter.to_lowercase().to_string()
                            }
                        }
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "random")]
    #[test]
    fn pseudo_no_triples() {
        let words = vec!["abcdefg", "hijklmnop", "qrstuv", "wxyz"];
        for _ in 0..5 {
            let new = pseudo_random(&words).join("");
            let mut iter = new
                .chars()
                .zip(new.chars().skip(1))
                .zip(new.chars().skip(2));
            assert!(!iter
                .clone()
                .any(|((a, b), c)| a.is_lowercase() && b.is_lowercase() && c.is_lowercase()));
            assert!(
                !iter.any(|((a, b), c)| a.is_uppercase() && b.is_uppercase() && c.is_uppercase())
            );
        }
    }

    #[cfg(feature = "random")]
    #[test]
    fn randoms_are_random() {
        let words = vec!["abcdefg", "hijklmnop", "qrstuv", "wxyz"];

        for _ in 0..5 {
            let transformed = pseudo_random(&words);
            assert_ne!(words, transformed);
            let transformed = random(&words);
            assert_ne!(words, transformed);
        }
    }

    #[test]
    fn mutate_empty_strings() {
        for word_pattern in [
            word_pattern::lowercase,
            word_pattern::uppercase,
            word_pattern::capital,
            word_pattern::toggle,
        ] {
            assert_eq!(String::new(), word_pattern(&String::new()))
        }
    }
}
