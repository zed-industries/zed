use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::Result;
use regex::{Regex, RegexBuilder};
use std::{
    borrow::Cow,
    io::{BufRead, BufReader, Read},
    ops::Range,
    sync::Arc,
};

#[derive(Clone)]
pub enum SearchQuery {
    Text { search: Arc<AhoCorasick<usize>> },
    Regex { multiline: bool, regex: Regex },
}

impl SearchQuery {
    pub fn text(query: &str) -> Self {
        let search = AhoCorasickBuilder::new()
            .auto_configure(&[query])
            .build(&[query]);
        Self::Text {
            search: Arc::new(search),
        }
    }

    pub fn regex(query: &str, whole_word: bool, case_sensitive: bool) -> Result<Self> {
        let mut query = Cow::Borrowed(query);
        if whole_word {
            let mut word_query = String::new();
            word_query.push_str("\\b");
            word_query.push_str(&query);
            word_query.push_str("\\b");
            query = Cow::Owned(word_query);
        }

        let multiline = query.contains("\n") || query.contains("\\n");
        let regex = RegexBuilder::new(&query)
            .case_insensitive(!case_sensitive)
            .multi_line(multiline)
            .build()?;
        Ok(Self::Regex { multiline, regex })
    }

    pub fn detect<T: Read>(&self, stream: T) -> Result<bool> {
        match self {
            SearchQuery::Text { search } => {
                let mat = search.stream_find_iter(stream).next();
                match mat {
                    Some(Ok(_)) => Ok(true),
                    Some(Err(err)) => Err(err.into()),
                    None => Ok(false),
                }
            }
            SearchQuery::Regex { multiline, regex } => {
                let mut reader = BufReader::new(stream);
                if *multiline {
                    let mut text = String::new();
                    if let Err(err) = reader.read_to_string(&mut text) {
                        Err(err.into())
                    } else {
                        Ok(regex.find(&text).is_some())
                    }
                } else {
                    for line in reader.lines() {
                        let line = line?;
                        if regex.find(&line).is_some() {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
            }
        }
    }

    pub fn search<'a, T: 'a + Read>(&'a self, stream: T) -> Result<Vec<Range<usize>>> {
        let mut matches = Vec::new();
        match self {
            SearchQuery::Text { search } => {
                for mat in search.stream_find_iter(stream) {
                    let mat = mat?;
                    matches.push(mat.start()..mat.end())
                }
            }
            SearchQuery::Regex { multiline, regex } => {
                let mut reader = BufReader::new(stream);
                if *multiline {
                    let mut text = String::new();
                    reader.read_to_string(&mut text)?;
                    matches.extend(regex.find_iter(&text).map(|mat| mat.start()..mat.end()));
                } else {
                    let mut line_ix = 0;
                    for line in reader.lines() {
                        let line = line?;
                        matches.extend(
                            regex
                                .find_iter(&line)
                                .map(|mat| (line_ix + mat.start())..(line_ix + mat.end())),
                        );
                        line_ix += line.len();
                    }
                }
            }
        }
        Ok(matches)
    }
}
