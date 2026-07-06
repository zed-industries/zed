use rustc_hir::def_id::DefId;
use rustc_hir::{ExprKind, HirId, Node};
use rustc_lint::LateContext;
use rustc_middle::ty::Ty;

/// Returns `true` when `hir_id` sits directly inside a `fn render` that
/// implements `gpui::Render` or `gpui::RenderOnce`, without an intervening
/// closure. If a closure sits between `hir_id` and the `render` method, the
/// expression executes later (e.g. in an event handler) and is not flagged.
pub(crate) fn is_directly_in_render_method(cx: &LateContext<'_>, hir_id: HirId) -> bool {
    for (parent_id, node) in cx.tcx.hir_parent_iter(hir_id) {
        match node {
            Node::Expr(expr) if matches!(expr.kind, ExprKind::Closure(_)) => {
                return false;
            }
            Node::ImplItem(impl_item) if impl_item.ident.name.as_str() == "render" => {
                return is_render_trait_impl(cx, parent_id);
            }
            _ => {}
        }
    }
    false
}

/// Returns `true` when the `impl` block that owns `impl_item_hir_id` is an
/// implementation of `gpui::Render` or `gpui::RenderOnce`.
fn is_render_trait_impl(cx: &LateContext<'_>, impl_item_hir_id: HirId) -> bool {
    let parent_owner = cx.tcx.hir_get_parent_item(impl_item_hir_id);
    let node = cx.tcx.hir_node(parent_owner.into());
    if let Node::Item(item) = node {
        if let rustc_hir::ItemKind::Impl(impl_block) = &item.kind {
            if let Some(trait_ref) = &impl_block.of_trait {
                if let rustc_hir::def::Res::Def(_, trait_def_id) = trait_ref.trait_ref.path.res {
                    return is_gpui_render_trait(cx, trait_def_id);
                }
            }
        }
    }
    false
}

fn is_gpui_render_trait(cx: &LateContext<'_>, trait_def_id: DefId) -> bool {
    let crate_name = cx.tcx.crate_name(trait_def_id.krate);
    if crate_name.as_str() != "gpui" {
        return false;
    }
    let name = cx.tcx.item_name(trait_def_id);
    name.as_str() == "Render" || name.as_str() == "RenderOnce"
}

/// Returns `true` when `ty` is `gpui::Entity<T>` or `gpui::WeakEntity<T>`.
pub(crate) fn is_gpui_entity_or_weak(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    let peeled = ty.peel_refs();
    let Some(adt) = peeled.ty_adt_def() else {
        return false;
    };
    let did = adt.did();
    let crate_name = cx.tcx.crate_name(did.krate);
    if crate_name.as_str() != "gpui" {
        return false;
    }
    let name = cx.tcx.item_name(did);
    name.as_str() == "Entity" || name.as_str() == "WeakEntity"
}

/// Returns `true` when `ty` is `gpui::Context<T>`.
pub(crate) fn is_gpui_context(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    let peeled = ty.peel_refs();
    let Some(adt) = peeled.ty_adt_def() else {
        return false;
    };
    let did = adt.did();
    let crate_name = cx.tcx.crate_name(did.krate);
    if crate_name.as_str() != "gpui" {
        return false;
    }
    cx.tcx.item_name(did).as_str() == "Context"
}

/// Returns `true` when the expression type indicates a unit-returning update
/// call — either `()` (from `Entity::update`) or `Result<(), _>` (from
/// `WeakEntity::update`).
pub(crate) fn is_unit_or_result_unit(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    if ty.is_unit() {
        return true;
    }
    if let Some(adt) = ty.ty_adt_def() {
        let path = cx.tcx.def_path_str(adt.did());
        if path == "core::result::Result" || path == "std::result::Result" {
            if let Some(substs) = ty.walk().nth(1) {
                if let Some(inner_ty) = substs.as_type() {
                    return inner_ty.is_unit();
                }
            }
        }
    }
    false
}
