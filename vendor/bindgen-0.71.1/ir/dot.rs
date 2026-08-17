//! Generating Graphviz `dot` files from our IR.

use super::context::{BindgenContext, ItemId};
use super::traversal::Trace;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// A trait for anything that can write attributes as `<table>` rows to a dot
/// file.
pub(crate) trait DotAttributes {
    /// Write this thing's attributes to the given output. Each attribute must
    /// be its own `<tr>...</tr>`.
    fn dot_attributes<W>(
        &self,
        ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: Write;
}

/// Write a graphviz dot file containing our IR.
pub(crate) fn write_dot_file<P>(ctx: &BindgenContext, path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut dot_file = io::BufWriter::new(file);
    writeln!(&mut dot_file, "digraph {{")?;

    let mut err: Option<io::Result<_>> = None;

    for (id, item) in ctx.items() {
        let is_allowlisted = ctx.allowlisted_items().contains(&id);

        writeln!(
            &mut dot_file,
            r#"{} [fontname="courier", color={}, label=< <table border="0" align="left">"#,
            id.as_usize(),
            if is_allowlisted { "black" } else { "gray" }
        )?;
        item.dot_attributes(ctx, &mut dot_file)?;
        writeln!(&mut dot_file, r#"</table> >];"#)?;

        item.trace(
            ctx,
            &mut |sub_id: ItemId, edge_kind| {
                if err.is_some() {
                    return;
                }

                match writeln!(
                    &mut dot_file,
                    "{} -> {} [label={edge_kind:?}, color={}];",
                    id.as_usize(),
                    sub_id.as_usize(),
                    if is_allowlisted { "black" } else { "gray" }
                ) {
                    Ok(_) => {}
                    Err(e) => err = Some(Err(e)),
                }
            },
            &(),
        );

        if let Some(err) = err {
            return err;
        }

        if let Some(module) = item.as_module() {
            for child in module.children() {
                writeln!(
                    &mut dot_file,
                    "{} -> {} [style=dotted, color=gray]",
                    item.id().as_usize(),
                    child.as_usize()
                )?;
            }
        }
    }

    writeln!(&mut dot_file, "}}")?;
    Ok(())
}
