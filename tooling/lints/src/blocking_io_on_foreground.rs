use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::Res;
use rustc_hir::{Expr, ExprKind, HirId, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;

use crate::render_helpers::is_directly_in_render_method;

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags calls to known blocking IO functions from the standard library
    /// (`std::fs`, `std::thread::sleep`, `std::process::Command`, `std::net`)
    /// when they appear inside a function that receives a synchronous GPUI
    /// context parameter (`&App`, `&mut App`, `&Context<T>`,
    /// `&mut Context<T>`, `&mut Window`) or directly inside a
    /// `Render::render` / `RenderOnce::render` method.
    ///
    /// ### Why is this bad?
    ///
    /// In GPUI, code that receives a synchronous context type runs on the
    /// foreground (UI) thread. A blocking IO call on this thread freezes the
    /// application until the syscall returns.
    pub BLOCKING_IO_ON_FOREGROUND,
    Warn,
    "blocking IO call on the GPUI foreground thread"
}

pub(crate) struct BlockingIoOnForeground;

rustc_session::impl_lint_pass!(BlockingIoOnForeground => [BLOCKING_IO_ON_FOREGROUND]);

const BLOCKING_FN_PATHS: &[&str] = &[
    // std::fs free functions
    "std::fs::read",
    "std::fs::read_to_string",
    "std::fs::write",
    "std::fs::read_dir",
    "std::fs::read_link",
    "std::fs::metadata",
    "std::fs::symlink_metadata",
    "std::fs::set_permissions",
    "std::fs::canonicalize",
    "std::fs::create_dir",
    "std::fs::create_dir_all",
    "std::fs::remove_file",
    "std::fs::remove_dir",
    "std::fs::remove_dir_all",
    "std::fs::copy",
    "std::fs::rename",
    "std::fs::hard_link",
    // std::fs::File associated functions
    "std::fs::File::open",
    "std::fs::File::create",
    "std::fs::File::create_new",
    // std::thread
    "std::thread::sleep",
    // std::path::Path methods (resolved via method call def_id)
    "std::path::Path::metadata",
    "std::path::Path::symlink_metadata",
    "std::path::Path::read_link",
    "std::path::Path::read_dir",
    "std::path::Path::exists",
    "std::path::Path::try_exists",
    "std::path::Path::is_file",
    "std::path::Path::is_dir",
    "std::path::Path::is_symlink",
    "std::path::Path::canonicalize",
    // std::net associated functions
    "std::net::TcpStream::connect",
    "std::net::TcpStream::connect_timeout",
    "std::net::TcpListener::bind",
    "std::net::UdpSocket::bind",
];

const BLOCKING_METHODS: &[(&str, &str)] = &[
    // std::process
    ("Command", "output"),
    ("Command", "status"),
    ("Command", "spawn"),
    ("Child", "wait"),
    ("Child", "wait_with_output"),
    // std::fs::File instance methods
    ("File", "sync_all"),
    ("File", "sync_data"),
    ("File", "set_len"),
    ("File", "metadata"),
    ("File", "try_clone"),
    ("File", "set_permissions"),
    // std::net — TCP
    ("TcpStream", "connect"),
    ("TcpStream", "peek"),
    ("TcpListener", "bind"),
    ("TcpListener", "accept"),
    ("TcpListener", "incoming"),
    // std::net — UDP
    ("UdpSocket", "send"),
    ("UdpSocket", "send_to"),
    ("UdpSocket", "recv"),
    ("UdpSocket", "recv_from"),
    ("UdpSocket", "peek"),
    ("UdpSocket", "peek_from"),
    // std::sync
    ("Mutex", "lock"),
    ("RwLock", "read"),
    ("RwLock", "write"),
    ("Condvar", "wait"),
    ("Condvar", "wait_timeout"),
    ("Condvar", "wait_while"),
    ("Barrier", "wait"),
    // std::sync::mpsc
    ("Receiver", "recv"),
    ("Receiver", "recv_timeout"),
    ("SyncSender", "send"),
];

fn is_blocking_call(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    match &expr.kind {
        ExprKind::Call(callee, _) => {
            if let ExprKind::Path(qpath) = &callee.kind {
                if let Res::Def(_, def_id) = cx.qpath_res(qpath, callee.hir_id) {
                    let path = cx.tcx.def_path_str(def_id);
                    return BLOCKING_FN_PATHS.iter().any(|blocked| path == *blocked);
                }
            }
            false
        }
        ExprKind::MethodCall(segment, receiver, _args, _span) => {
            if let Some(def_id) = cx.typeck_results().type_dependent_def_id(expr.hir_id) {
                let path = cx.tcx.def_path_str(def_id);
                if BLOCKING_FN_PATHS.iter().any(|blocked| path == *blocked) {
                    return true;
                }
            }
            let method_name = segment.ident.name.as_str();
            if !BLOCKING_METHODS
                .iter()
                .any(|(_, name)| *name == method_name)
            {
                return false;
            }
            let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();
            if let Some(adt) = receiver_ty.ty_adt_def() {
                let type_name = cx.tcx.item_name(adt.did());
                return BLOCKING_METHODS
                    .iter()
                    .any(|(ty, name)| *name == method_name && type_name.as_str() == *ty);
            }
            false
        }
        _ => false,
    }
}

/// Returns `true` if `ty` (after peeling references) is a synchronous GPUI
/// foreground type: `App`, `Context`, or `Window`.
fn is_gpui_foreground_type<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
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
    matches!(name.as_str(), "App" | "Context" | "Window")
}

/// Walks up the HIR parent chain from `hir_id` to find the enclosing
/// function. Returns `true` if that function has a parameter whose type is a
/// synchronous GPUI context type. Returns `false` if a closure boundary is
/// crossed first (the closure might run on a background thread).
fn is_in_foreground_fn(cx: &LateContext<'_>, hir_id: HirId) -> bool {
    for (_parent_id, node) in cx.tcx.hir_parent_iter(hir_id) {
        match node {
            Node::Expr(expr) if matches!(expr.kind, ExprKind::Closure(_)) => {
                return false;
            }
            Node::Item(item) => {
                if let rustc_hir::ItemKind::Fn { .. } = &item.kind {
                    let owner_id = item.owner_id.def_id;
                    return owner_has_foreground_param(cx, owner_id);
                }
                return false;
            }
            Node::ImplItem(impl_item) => {
                if let rustc_hir::ImplItemKind::Fn(_, _) = &impl_item.kind {
                    let owner_id = impl_item.owner_id.def_id;
                    return owner_has_foreground_param(cx, owner_id);
                }
                return false;
            }
            Node::TraitItem(trait_item) => {
                if let rustc_hir::TraitItemKind::Fn(_, _) = &trait_item.kind {
                    let owner_id = trait_item.owner_id.def_id;
                    return owner_has_foreground_param(cx, owner_id);
                }
                return false;
            }
            _ => {}
        }
    }
    false
}

/// Checks whether the function identified by `local_def_id` has any parameter
/// whose type is a synchronous GPUI foreground type.
fn owner_has_foreground_param(
    cx: &LateContext<'_>,
    local_def_id: rustc_hir::def_id::LocalDefId,
) -> bool {
    let def_id = local_def_id.to_def_id();
    let sig = cx.tcx.fn_sig(def_id).instantiate_identity();
    sig.inputs()
        .skip_binder()
        .iter()
        .any(|ty| is_gpui_foreground_type(cx, *ty))
}

impl<'tcx> LateLintPass<'tcx> for BlockingIoOnForeground {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        if !is_blocking_call(cx, expr) {
            return;
        }

        let in_render = is_directly_in_render_method(cx, expr.hir_id);
        let in_foreground_fn = is_in_foreground_fn(cx, expr.hir_id);

        if !in_render && !in_foreground_fn {
            return;
        }

        span_lint(
            cx,
            BLOCKING_IO_ON_FOREGROUND,
            expr.span,
            "blocking IO call on the GPUI foreground thread",
        );
    }
}
