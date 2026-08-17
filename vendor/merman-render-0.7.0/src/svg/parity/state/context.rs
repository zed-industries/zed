use super::*;

pub(super) fn state_is_hidden(ctx: &StateRenderCtx<'_>, id: &str) -> bool {
    ctx.hidden_prefixes
        .iter()
        .any(|p| id == p || id.starts_with(&format!("{p}----")))
}

fn state_leaf_context_impl<'a>(
    ctx: &'a StateRenderCtx<'_>,
    id: &str,
    respect_nested_roots: bool,
) -> Option<&'a str> {
    let mut p = ctx.parent.get(id).copied();
    while p.is_some() {
        let pid = state_strip_note_group(ctx, p)?;
        let Some(pn) = ctx.nodes_by_id.get(pid).copied() else {
            return Some(pid);
        };
        if pn.is_group && pn.shape != "noteGroup" {
            if !respect_nested_roots || ctx.nested_roots.contains(pid) {
                return Some(pid);
            }
            p = ctx.parent.get(pid).copied();
            continue;
        }
        p = ctx.parent.get(pid).copied();
    }
    None
}

fn state_strip_note_group<'a>(
    ctx: &'a StateRenderCtx<'_>,
    mut parent: Option<&'a str>,
) -> Option<&'a str> {
    while let Some(pid) = parent {
        let Some(pn) = ctx.nodes_by_id.get(pid).copied() else {
            return Some(pid);
        };
        if pn.shape == "noteGroup" {
            parent = ctx.parent.get(pid).copied();
            continue;
        }
        return Some(pid);
    }
    None
}

fn state_insertion_context_impl<'a>(
    ctx: &'a StateRenderCtx<'_>,
    cluster_id: &str,
    respect_nested_roots: bool,
) -> Option<&'a str> {
    state_leaf_context_impl(ctx, cluster_id, respect_nested_roots)
}

fn state_endpoint_context_impl<'a>(
    ctx: &'a StateRenderCtx<'_>,
    id: &str,
    respect_nested_roots: bool,
) -> Option<&'a str> {
    if let Some(n) = ctx.nodes_by_id.get(id).copied() {
        if n.is_group && n.shape != "noteGroup" {
            return state_insertion_context_impl(ctx, id, respect_nested_roots);
        }
    }
    state_leaf_context_impl(ctx, id, respect_nested_roots)
}

fn state_context_chain_impl<'a>(
    ctx: &'a StateRenderCtx<'_>,
    mut c: Option<&'a str>,
    respect_nested_roots: bool,
) -> Vec<Option<&'a str>> {
    let mut out = Vec::new();
    loop {
        out.push(c);
        let Some(id) = c else {
            break;
        };
        c = state_insertion_context_impl(ctx, id, respect_nested_roots);
    }
    out
}

fn state_edge_context_impl<'a>(
    ctx: &'a StateRenderCtx<'_>,
    edge: &StateSvgEdge,
    respect_nested_roots: bool,
) -> Option<&'a str> {
    let a = state_endpoint_context_impl(ctx, edge.start.as_str(), respect_nested_roots);
    let b = state_endpoint_context_impl(ctx, edge.end.as_str(), respect_nested_roots);
    let ca = state_context_chain_impl(ctx, a, respect_nested_roots);
    let cb = state_context_chain_impl(ctx, b, respect_nested_roots);
    for anc in cb {
        if ca.contains(&anc) {
            return anc;
        }
    }
    None
}

pub(super) fn state_endpoint_context_raw<'a>(
    ctx: &'a StateRenderCtx<'_>,
    id: &str,
) -> Option<&'a str> {
    state_endpoint_context_impl(ctx, id, false)
}

pub(super) fn state_context_chain_raw<'a>(
    ctx: &'a StateRenderCtx<'_>,
    c: Option<&'a str>,
) -> Vec<Option<&'a str>> {
    state_context_chain_impl(ctx, c, false)
}

pub(super) fn state_edge_context_raw<'a>(
    ctx: &'a StateRenderCtx<'_>,
    edge: &StateSvgEdge,
) -> Option<&'a str> {
    state_edge_context_impl(ctx, edge, false)
}

pub(super) fn state_leaf_context<'a>(ctx: &'a StateRenderCtx<'_>, id: &str) -> Option<&'a str> {
    state_leaf_context_impl(ctx, id, true)
}

pub(super) fn state_insertion_context<'a>(
    ctx: &'a StateRenderCtx<'_>,
    cluster_id: &str,
) -> Option<&'a str> {
    state_insertion_context_impl(ctx, cluster_id, true)
}

pub(super) fn state_edge_context<'a>(
    ctx: &'a StateRenderCtx<'_>,
    edge: &StateSvgEdge,
) -> Option<&'a str> {
    state_edge_context_impl(ctx, edge, true)
}

pub(super) fn state_is_shadowed_self_loop_edge(
    ctx: &StateRenderCtx<'_>,
    edge_index: usize,
    edge: &StateSvgEdge,
    root: Option<&str>,
) -> bool {
    if edge.start != edge.end {
        return false;
    }
    if state_edge_context(ctx, edge) != root {
        return false;
    }

    for later in ctx.edges.iter().skip(edge_index + 1) {
        if later.start != later.end {
            continue;
        }
        if later.start != edge.start || later.end != edge.end {
            continue;
        }
        if state_is_hidden(ctx, later.start.as_str())
            || state_is_hidden(ctx, later.end.as_str())
            || state_is_hidden(ctx, later.id.as_str())
        {
            continue;
        }
        if state_edge_context(ctx, later) != root {
            continue;
        }
        return true;
    }

    false
}
