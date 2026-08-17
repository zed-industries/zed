use crate::key::Key;
use crate::parser::error::CustomError;
use crate::repr::Decor;
use crate::{ArrayOfTables, ImDocument, Item, RawString, Table};

pub(crate) struct ParseState {
    root: Table,
    trailing: Option<std::ops::Range<usize>>,
    current_table_position: usize,
    current_table: Table,
    current_is_array: bool,
    current_table_path: Vec<Key>,
}

impl ParseState {
    pub(crate) fn new() -> Self {
        let mut root = Table::new();
        root.span = Some(0..0);
        Self {
            root: Table::new(),
            trailing: None,
            current_table_position: 0,
            current_table: root,
            current_is_array: false,
            current_table_path: Vec::new(),
        }
    }

    pub(crate) fn into_document<S>(mut self, raw: S) -> Result<ImDocument<S>, CustomError> {
        self.finalize_table()?;
        let trailing = self.trailing.map(RawString::with_span).unwrap_or_default();
        Ok(ImDocument {
            root: Item::Table(self.root),
            trailing,
            raw,
        })
    }

    pub(crate) fn on_ws(&mut self, span: std::ops::Range<usize>) {
        if let Some(old) = self.trailing.take() {
            self.trailing = Some(old.start..span.end);
        } else {
            self.trailing = Some(span);
        }
    }

    pub(crate) fn on_comment(&mut self, span: std::ops::Range<usize>) {
        if let Some(old) = self.trailing.take() {
            self.trailing = Some(old.start..span.end);
        } else {
            self.trailing = Some(span);
        }
    }

    pub(crate) fn on_keyval(
        &mut self,
        path: Vec<Key>,
        (mut key, value): (Key, Item),
    ) -> Result<(), CustomError> {
        {
            let mut prefix = self.trailing.take();
            let prefix = match (
                prefix.take(),
                key.leaf_decor.prefix().and_then(|d| d.span()),
            ) {
                (Some(p), Some(k)) => Some(p.start..k.end),
                (Some(p), None) | (None, Some(p)) => Some(p),
                (None, None) => None,
            };
            key.leaf_decor
                .set_prefix(prefix.map(RawString::with_span).unwrap_or_default());
        }

        if let (Some(existing), Some(value)) = (self.current_table.span(), value.span()) {
            self.current_table.span = Some((existing.start)..(value.end));
        }
        let table = &mut self.current_table;
        let table = Self::descend_path(table, &path, true)?;

        // "Likewise, using dotted keys to redefine tables already defined in [table] form is not allowed"
        let mixed_table_types = table.is_dotted() == path.is_empty();
        if mixed_table_types {
            return Err(CustomError::DuplicateKey {
                key: key.get().into(),
                table: None,
            });
        }

        match table.items.entry(key) {
            indexmap::map::Entry::Vacant(o) => {
                o.insert(value);
            }
            indexmap::map::Entry::Occupied(o) => {
                // "Since tables cannot be defined more than once, redefining such tables using a [table] header is not allowed"
                return Err(CustomError::DuplicateKey {
                    key: o.key().get().into(),
                    table: Some(self.current_table_path.clone()),
                });
            }
        }

        Ok(())
    }

    pub(crate) fn start_array_table(
        &mut self,
        path: Vec<Key>,
        decor: Decor,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());
        debug_assert!(self.current_table.is_empty());
        debug_assert!(self.current_table_path.is_empty());

        // Look up the table on start to ensure the duplicate_key error points to the right line
        let root = &mut self.root;
        let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
        let key = &path[path.len() - 1];
        let entry = parent_table
            .entry_format(key)
            .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
        entry
            .as_array_of_tables()
            .ok_or_else(|| CustomError::duplicate_key(&path, path.len() - 1))?;

        self.current_table_position += 1;
        self.current_table.decor = decor;
        self.current_table.set_implicit(false);
        self.current_table.set_dotted(false);
        self.current_table.set_position(self.current_table_position);
        self.current_table.span = Some(span);
        self.current_is_array = true;
        self.current_table_path = path;

        Ok(())
    }

    pub(crate) fn start_table(
        &mut self,
        path: Vec<Key>,
        decor: Decor,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());
        debug_assert!(self.current_table.is_empty());
        debug_assert!(self.current_table_path.is_empty());

        // 1. Look up the table on start to ensure the duplicate_key error points to the right line
        // 2. Ensure any child tables from an implicit table are preserved
        let root = &mut self.root;
        let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
        let key = &path[path.len() - 1];
        if let Some(entry) = parent_table.remove(key.get()) {
            match entry {
                Item::Table(t) if t.implicit && !t.is_dotted() => {
                    self.current_table = t;
                }
                // Since tables cannot be defined more than once, redefining such tables using a [table] header is not allowed. Likewise, using dotted keys to redefine tables already defined in [table] form is not allowed.
                _ => return Err(CustomError::duplicate_key(&path, path.len() - 1)),
            }
        }

        self.current_table_position += 1;
        self.current_table.decor = decor;
        self.current_table.set_implicit(false);
        self.current_table.set_dotted(false);
        self.current_table.set_position(self.current_table_position);
        self.current_table.span = Some(span);
        self.current_is_array = false;
        self.current_table_path = path;

        Ok(())
    }

    pub(crate) fn finalize_table(&mut self) -> Result<(), CustomError> {
        let mut table = std::mem::take(&mut self.current_table);
        let path = std::mem::take(&mut self.current_table_path);

        let root = &mut self.root;
        if path.is_empty() {
            assert!(root.is_empty());
            std::mem::swap(&mut table, root);
        } else if self.current_is_array {
            let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
            let key = &path[path.len() - 1];

            let entry = parent_table
                .entry_format(key)
                .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
            let array = entry
                .as_array_of_tables_mut()
                .ok_or_else(|| CustomError::duplicate_key(&path, path.len() - 1))?;
            array.push(table);
            let span = if let (Some(first), Some(last)) = (
                array.values.first().and_then(|t| t.span()),
                array.values.last().and_then(|t| t.span()),
            ) {
                Some((first.start)..(last.end))
            } else {
                None
            };
            array.span = span;
        } else {
            let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
            let key = &path[path.len() - 1];

            let entry = parent_table.entry_format(key);
            match entry {
                crate::Entry::Occupied(entry) => {
                    match entry.into_mut() {
                        // if [a.b.c] header preceded [a.b]
                        Item::Table(ref mut t) if t.implicit => {
                            std::mem::swap(t, &mut table);
                        }
                        _ => return Err(CustomError::duplicate_key(&path, path.len() - 1)),
                    }
                }
                crate::Entry::Vacant(entry) => {
                    let item = Item::Table(table);
                    entry.insert(item);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn descend_path<'t>(
        mut table: &'t mut Table,
        path: &[Key],
        dotted: bool,
    ) -> Result<&'t mut Table, CustomError> {
        for (i, key) in path.iter().enumerate() {
            table = match table.entry_format(key) {
                crate::Entry::Vacant(entry) => {
                    let mut new_table = Table::new();
                    new_table.set_implicit(true);
                    new_table.set_dotted(dotted);

                    entry.insert(Item::Table(new_table)).as_table_mut().unwrap()
                }
                crate::Entry::Occupied(entry) => {
                    match entry.into_mut() {
                        Item::Value(ref v) => {
                            return Err(CustomError::extend_wrong_type(path, i, v.type_name()));
                        }
                        Item::ArrayOfTables(ref mut array) => {
                            debug_assert!(!array.is_empty());

                            let index = array.len() - 1;
                            let last_child = array.get_mut(index).unwrap();

                            last_child
                        }
                        Item::Table(ref mut sweet_child_of_mine) => {
                            // Since tables cannot be defined more than once, redefining such tables using a
                            // [table] header is not allowed. Likewise, using dotted keys to redefine tables
                            // already defined in [table] form is not allowed.
                            if dotted && !sweet_child_of_mine.is_implicit() {
                                return Err(CustomError::DuplicateKey {
                                    key: key.get().into(),
                                    table: None,
                                });
                            }
                            sweet_child_of_mine
                        }
                        Item::None => unreachable!(),
                    }
                }
            };
        }
        Ok(table)
    }

    pub(crate) fn on_std_header(
        &mut self,
        path: Vec<Key>,
        trailing: std::ops::Range<usize>,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        self.finalize_table()?;
        let leading = self
            .trailing
            .take()
            .map(RawString::with_span)
            .unwrap_or_default();
        self.start_table(
            path,
            Decor::new(leading, RawString::with_span(trailing)),
            span,
        )?;

        Ok(())
    }

    pub(crate) fn on_array_header(
        &mut self,
        path: Vec<Key>,
        trailing: std::ops::Range<usize>,
        span: std::ops::Range<usize>,
    ) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        self.finalize_table()?;
        let leading = self
            .trailing
            .take()
            .map(RawString::with_span)
            .unwrap_or_default();
        self.start_array_table(
            path,
            Decor::new(leading, RawString::with_span(trailing)),
            span,
        )?;

        Ok(())
    }
}
