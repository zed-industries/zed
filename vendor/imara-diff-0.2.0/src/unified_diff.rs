use std::fmt::{self, Display};
use std::hash::Hash;

use crate::intern::{InternedInput, Interner, Token};
use crate::Diff;

impl Diff {
    pub fn unified_diff<'a, P: UnifiedDiffPrinter, T: Hash + Eq>(
        &'a self,
        printer: &'a P,
        config: UnifiedDiffConfig,
        input: &'a InternedInput<T>,
    ) -> UnifiedDiff<'a, P> {
        self.unified_diff_with(printer, config, &input.before, &input.after)
    }

    pub fn unified_diff_with<'a, P: UnifiedDiffPrinter>(
        &'a self,
        printer: &'a P,
        config: UnifiedDiffConfig,
        before: &'a [Token],
        after: &'a [Token],
    ) -> UnifiedDiff<'a, P> {
        UnifiedDiff {
            printer,
            diff: self,
            config,
            before,
            after,
        }
    }
}

pub trait UnifiedDiffPrinter {
    fn display_header(
        &self,
        f: impl fmt::Write,
        start_before: u32,
        start_after: u32,
        len_before: u32,
        len_after: u32,
    ) -> fmt::Result;
    fn display_context_token(&self, f: impl fmt::Write, token: Token) -> fmt::Result;
    fn display_hunk(&self, f: impl fmt::Write, before: &[Token], after: &[Token]) -> fmt::Result;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnifiedDiffConfig {
    context_len: u32,
}

impl Default for UnifiedDiffConfig {
    fn default() -> Self {
        UnifiedDiffConfig { context_len: 3 }
    }
}

impl UnifiedDiffConfig {
    pub fn context_len(&mut self, len: u32) -> &mut Self {
        self.context_len = len;
        self
    }
}

pub trait EndsWithNewline {
    fn ends_with_newline(&self) -> bool;
}

impl<T: AsRef<[u8]> + ?Sized> EndsWithNewline for T {
    fn ends_with_newline(&self) -> bool {
        self.as_ref().ends_with(b"\n")
    }
}

pub struct BasicLineDiffPrinter<'a, T: EndsWithNewline + ?Sized + Hash + Eq + Display>(
    pub &'a Interner<&'a T>,
);

impl<T: EndsWithNewline + Hash + Eq + Display + ?Sized> UnifiedDiffPrinter
    for BasicLineDiffPrinter<'_, T>
{
    fn display_header(
        &self,
        mut f: impl fmt::Write,
        start_before: u32,
        start_after: u32,
        len_before: u32,
        len_after: u32,
    ) -> fmt::Result {
        writeln!(
            f,
            "@@ -{},{} +{},{} @@",
            start_before + 1,
            len_before,
            start_after + 1,
            len_after
        )
    }

    fn display_context_token(&self, mut f: impl fmt::Write, token: Token) -> fmt::Result {
        write!(f, " {}", &self.0[token])?;
        if !&self.0[token].ends_with_newline() {
            writeln!(f)?;
        }
        Ok(())
    }

    fn display_hunk(
        &self,
        mut f: impl fmt::Write,
        before: &[Token],
        after: &[Token],
    ) -> fmt::Result {
        if let Some(&last) = before.last() {
            for &token in before {
                let token = self.0[token];
                write!(f, "-{token}")?;
            }
            if !self.0[last].ends_with_newline() {
                writeln!(f)?;
            }
        }
        if let Some(&last) = after.last() {
            for &token in after {
                let token = self.0[token];
                write!(f, "+{token}")?;
            }
            if !self.0[last].ends_with_newline() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub struct UnifiedDiff<'a, P: UnifiedDiffPrinter> {
    printer: &'a P,
    diff: &'a Diff,
    config: UnifiedDiffConfig,
    before: &'a [Token],
    after: &'a [Token],
}

impl<P: UnifiedDiffPrinter> Display for UnifiedDiff<'_, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pos = 0;
        let mut before_context_len = 0;
        let mut after_context_len = 0;
        let first_hunk = self.diff.hunks().next().unwrap_or_default();
        let mut before_context_start = first_hunk
            .before
            .start
            .saturating_sub(self.config.context_len);
        let mut after_context_start = first_hunk
            .after
            .start
            .saturating_sub(self.config.context_len);
        let mut buffer = String::new();
        for hunk in self.diff.hunks() {
            if hunk.before.start - pos > 2 * self.config.context_len {
                if !buffer.is_empty() {
                    let end = (pos + self.config.context_len).min(self.before.len() as u32);
                    self.printer.display_header(
                        &mut *f,
                        before_context_start,
                        after_context_start,
                        before_context_len + end - pos,
                        after_context_len + end - pos,
                    )?;
                    write!(f, "{buffer}")?;
                    for &token in &self.before[pos as usize..end as usize] {
                        self.printer.display_context_token(&mut *f, token)?;
                    }
                    buffer.clear();
                }
                pos = hunk.before.start - self.config.context_len;
                before_context_start = pos;
                after_context_start = hunk.after.start - self.config.context_len;
                before_context_len = 0;
                after_context_len = 0;
            }
            for &token in &self.before[pos as usize..hunk.before.start as usize] {
                self.printer.display_context_token(&mut buffer, token)?;
            }
            let context_len = hunk.before.start - pos;
            before_context_len += hunk.before.len() as u32 + context_len;
            after_context_len += hunk.after.len() as u32 + context_len;
            self.printer.display_hunk(
                &mut buffer,
                &self.before[hunk.before.start as usize..hunk.before.end as usize],
                &self.after[hunk.after.start as usize..hunk.after.end as usize],
            )?;
            pos = hunk.before.end;
        }
        if !buffer.is_empty() {
            let end = (pos + self.config.context_len).min(self.before.len() as u32);
            self.printer.display_header(
                &mut *f,
                before_context_start,
                after_context_start,
                before_context_len + end - pos,
                after_context_len + end - pos,
            )?;
            write!(f, "{buffer}")?;
            for &token in &self.before[pos as usize..end as usize] {
                self.printer.display_context_token(&mut *f, token)?;
            }
            buffer.clear();
        }
        Ok(())
    }
}
