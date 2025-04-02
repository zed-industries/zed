use project::DocumentSymbol;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub depth: u32,
    pub start_line: usize,
    pub end_line: usize,
}

/// An iterator that filters document symbols based on a regex pattern.
/// This iterator recursively traverses the document symbol tree, incrementing depth for child symbols.
#[derive(Debug, Clone)]
pub struct CodeSymbolIterator<'a> {
    symbols: &'a [DocumentSymbol],
    regex: Option<Regex>,
    // Stack of (symbol, depth) pairs to process
    pending_symbols: Vec<(&'a DocumentSymbol, u32)>,
    current_index: usize,
    current_depth: u32,
}

impl<'a> CodeSymbolIterator<'a> {
    pub fn new(symbols: &'a [DocumentSymbol], regex: Option<Regex>) -> Self {
        Self {
            symbols,
            regex,
            pending_symbols: Vec::new(),
            current_index: 0,
            current_depth: 0,
        }
    }
}

impl Iterator for CodeSymbolIterator<'_> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((symbol, depth)) = self.pending_symbols.pop() {
            for child in symbol.children.iter().rev() {
                self.pending_symbols.push((child, depth + 1));
            }

            return Some(Entry {
                name: symbol.name.clone(),
                kind: symbol.kind,
                depth,
                start_line: symbol.range.start.0.row as usize,
                end_line: symbol.range.end.0.row as usize,
            });
        }

        while self.current_index < self.symbols.len() {
            let regex = self.regex.as_ref();
            let symbol = &self.symbols[self.current_index];
            self.current_index += 1;

            if regex.is_none_or(|regex| regex.is_match(&symbol.name)) {
                // Push in reverse order to maintain traversal order
                for child in symbol.children.iter().rev() {
                    self.pending_symbols.push((child, self.current_depth + 1));
                }

                return Some(Entry {
                    name: symbol.name.clone(),
                    kind: symbol.kind,
                    depth: self.current_depth,
                    start_line: symbol.range.start.0.row as usize,
                    end_line: symbol.range.end.0.row as usize,
                });
            } else {
                // Even if parent doesn't match, push children to check them later
                for child in symbol.children.iter().rev() {
                    self.pending_symbols.push((child, self.current_depth + 1));
                }

                // Check if any pending children match our criteria
                if let Some(result) = self.next() {
                    return Some(result);
                }
            }
        }

        None
    }
}
