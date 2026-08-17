use crate::intern::Token;
use crate::myers::sqrt;

pub fn preprocess<'a>(
    before: &[Token],
    after: &[Token],
    removed: &'a mut [bool],
    added: &'a mut [bool],
) -> (PreprocessedFile, PreprocessedFile) {
    let (occurances_before, occurances_after) = token_occurrences(before, after);
    let file1 = PreprocessedFile::new(&occurances_before, before, removed);
    let file2 = PreprocessedFile::new(&occurances_after, after, added);
    (file1, file2)
}

fn token_occurrences(file1: &[Token], file2: &[Token]) -> (Vec<Occurrences>, Vec<Occurrences>) {
    const MAX_EQLIMIT: u32 = 1024;

    // compute the limit after which tokens are treated as `Occurrences::COMMON`
    let eqlimit1 = sqrt(file1.len()).min(MAX_EQLIMIT);
    let eqlimit2 = sqrt(file2.len()).min(MAX_EQLIMIT);

    // first collect how often each token occurs in a file
    let mut occurrences1 = Vec::new();
    for token in file1 {
        let bucket = token.0 as usize;
        if bucket >= occurrences1.len() {
            occurrences1.resize(bucket + 1, 0u32);
        }
        occurrences1[bucket] += 1;
    }

    // do the same thing for
    let mut occurrences2 = Vec::new();
    let token_occurrences2: Vec<_> = file2
        .iter()
        .map(|token| {
            let bucket = token.0 as usize;
            if bucket >= occurrences2.len() {
                occurrences2.resize(bucket + 1, 0);
            }
            occurrences2[bucket] += 1;
            let occurrences1 = *occurrences1.get(bucket).unwrap_or(&0);
            Occurrences::from_occurrences(occurrences1, eqlimit2)
        })
        .collect();

    let token_occurrences1: Vec<_> = file1
        .iter()
        .map(|token| {
            let bucket = token.0 as usize;
            let occurrences2 = *occurrences2.get(bucket).unwrap_or(&0);
            Occurrences::from_occurrences(occurrences2, eqlimit1)
        })
        .collect();

    (token_occurrences1, token_occurrences2)
}

#[derive(Clone, Copy, Debug)]
enum Occurrences {
    /// Token does not occur in this file
    None,
    /// Token occurs at least once
    Some,
    /// Token occurs very frequently (exact number depends on file size).
    /// Such tokens are usually empty lines or braces and are often not meaningful to a diff
    Common,
}

impl Occurrences {
    pub fn from_occurrences(occurrences: u32, eqlimit: u32) -> Occurrences {
        if occurrences == 0 {
            Occurrences::None
        } else if occurrences >= eqlimit {
            Occurrences::Common
        } else {
            Occurrences::Some
        }
    }
}

#[derive(Debug)]
pub struct PreprocessedFile {
    pub indices: Vec<u32>,
    pub tokens: Vec<Token>,
}

impl PreprocessedFile {
    fn new(
        token_occurrences: &[Occurrences],
        tokens: &[Token],
        changed: &mut [bool],
    ) -> PreprocessedFile {
        let (tokens, indices) = prune_unmatched_tokens(tokens, token_occurrences, changed);
        PreprocessedFile { indices, tokens }
    }
}

fn prune_unmatched_tokens(
    file: &[Token],
    token_status: &[Occurrences],
    changed: &mut [bool],
) -> (Vec<Token>, Vec<u32>) {
    assert_eq!(token_status.len(), file.len());
    file.iter()
        .zip(token_status)
        .enumerate()
        .filter_map(|(i, (&token, &status))| {
            let prune = match status {
                Occurrences::None => true,
                Occurrences::Some => false,
                Occurrences::Common => should_prune_common_line(token_status, i),
            };
            if prune {
                changed[i] = true;
                None
            } else {
                Some((token, i as u32))
            }
        })
        .unzip()
}

// TODO do not unnecessarily rescan lines
fn should_prune_common_line(token_status: &[Occurrences], pos: usize) -> bool {
    const WINDOW_SIZE: usize = 100;

    let mut unmatched_before = 0;
    let mut common_before = 0;

    let start = if pos > WINDOW_SIZE { WINDOW_SIZE } else { 0 };
    for status in token_status[start..pos].iter().rev() {
        match status {
            Occurrences::None => {
                unmatched_before += 1;
            }
            Occurrences::Common => {
                common_before += 1;
            }
            Occurrences::Some => break,
        }
    }

    if unmatched_before == 0 {
        return false;
    }

    let end = token_status.len().min(pos + WINDOW_SIZE);
    let mut unmatched_after = 0;
    let mut common_after = 0;
    for status in token_status[pos..end].iter() {
        match status {
            Occurrences::None => {
                unmatched_after += 1;
            }
            Occurrences::Common => {
                common_after += 1;
            }
            Occurrences::Some => break,
        }
    }

    if unmatched_after == 0 {
        return false;
    }

    let common = common_before + common_after;
    let unmatched = unmatched_before + unmatched_after;

    unmatched > 3 * common
}
