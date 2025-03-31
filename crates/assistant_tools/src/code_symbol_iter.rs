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
        // First, check if we have any pending symbols to process
        if let Some((symbol, depth)) = self.pending_symbols.pop() {
            // Add children to pending stack with increased depth
            for child in symbol.children.iter().rev() {
                self.pending_symbols.push((child, depth + 1));
            }

            // Return the current symbol as an Entry
            return Some(Entry {
                name: symbol.name.clone(),
                kind: symbol.kind,
                depth,
                start_line: symbol.range.start.0.row as usize,
                end_line: symbol.range.end.0.row as usize,
            });
        }

        // If no pending symbols, try to get the next symbol from the slice
        while self.current_index < self.symbols.len() {
            let symbol = &self.symbols[self.current_index];
            self.current_index += 1;

            // Process symbol based on regex pattern
            let matches = match &self.regex {
                None => true,
                Some(re) => re.is_match(&symbol.name),
            };

            // If the symbol matches or we want to check its children
            if matches {
                // Push children onto the stack with incremented depth (in reverse order to maintain traversal order)
                for child in symbol.children.iter().rev() {
                    self.pending_symbols.push((child, self.current_depth + 1));
                }

                // Return the current symbol as an Entry
                return Some(Entry {
                    name: symbol.name.clone(),
                    kind: symbol.kind,
                    depth: self.current_depth,
                    start_line: symbol.range.start.0.row as usize,
                    end_line: symbol.range.end.0.row as usize,
                });
            } else {
                // Even if the parent doesn't match, push children to check them
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
