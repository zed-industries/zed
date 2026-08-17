mod expressions;
mod functions;
mod handle_set_map;
mod statements;
mod types;

use alloc::vec::Vec;

use crate::{
    arena::{self, HandleSet},
    compact::functions::FunctionTracer,
    ir,
};
use handle_set_map::HandleMap;

#[cfg(test)]
use alloc::{format, string::ToString};

/// Configuration option for [`compact`]. See [`compact`] for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeepUnused {
    No,
    Yes,
}

impl From<KeepUnused> for bool {
    fn from(keep_unused: KeepUnused) -> Self {
        match keep_unused {
            KeepUnused::No => false,
            KeepUnused::Yes => true,
        }
    }
}

/// Remove most unused objects from `module`, which must be valid.
///
/// Always removes the following unused objects:
/// - anonymous types, overrides, and constants
/// - abstract-typed constants
/// - expressions
///
/// If `keep_unused` is `Yes`, the following are never considered unused,
/// otherwise, they will also be removed if unused:
/// - functions
/// - global variables
/// - named types and overrides
///
/// The following are never removed:
/// - named constants with a concrete type
/// - special types
/// - entry points
/// - within an entry point or a used function:
///     - arguments
///     - local variables
///     - named expressions
///
/// After removing items according to the rules above, all handles in the
/// remaining objects are adjusted as necessary. When `KeepUnused` is `Yes`, the
/// resulting module should have all the named objects (except abstract-typed
/// constants) present in the original, and those objects should be functionally
/// identical. When `KeepUnused` is `No`, the resulting module should have the
/// entry points present in the original, and those entry points should be
/// functionally identical.
///
/// # Panics
///
/// If `module` would not pass validation, this may panic.
pub fn compact(module: &mut crate::Module, keep_unused: KeepUnused) {
    // The trickiest part of compaction is determining what is used and what is
    // not. Once we have computed that correctly, it's easy enough to call
    // `retain_mut` on each arena, drop unused elements, and fix up the handles
    // in what's left.
    //
    // For every compactable arena in a `Module`, whether global to the `Module`
    // or local to a function or entry point, the `ModuleTracer` type holds a
    // bitmap indicating which elements of that arena are used. Our task is to
    // populate those bitmaps correctly.
    //
    // First, we mark everything that is considered used by definition, as
    // described in this function's documentation.
    //
    // Since functions and entry points are considered used by definition, we
    // traverse their statement trees, and mark the referents of all handles
    // appearing in those statements as used.
    //
    // Once we've marked which elements of an arena are referred to directly by
    // handles elsewhere (for example, which of a function's expressions are
    // referred to by handles in its body statements), we can mark all the other
    // arena elements that are used indirectly in a single pass, traversing the
    // arena from back to front. Since Naga allows arena elements to refer only
    // to prior elements, we know that by the time we reach an element, all
    // other elements that could possibly refer to it have already been visited.
    // Thus, if the present element has not been marked as used, then it is
    // definitely unused, and compaction can remove it. Otherwise, the element
    // is used and must be retained, so we must mark everything it refers to.
    //
    // The final step is to mark the global expressions and types, which must be
    // traversed simultaneously; see `ModuleTracer::type_expression_tandem`'s
    // documentation for details.
    //
    // # A definition and a rule of thumb
    //
    // In this module, to "trace" something is to mark everything else it refers
    // to as used, on the assumption that the thing itself is used. For example,
    // to trace an `Expression` is to mark its subexpressions as used, as well
    // as any types, constants, overrides, etc. that it refers to. This is what
    // `ExpressionTracer::trace_expression` does.
    //
    // Given that we we want to visit each thing only once (to keep compaction
    // linear in the size of the module), this definition of "trace" implies
    // that things that are not "used by definition" must be marked as used
    // *before* we trace them.
    //
    // Thus, whenever you are marking something as used, it's a good idea to ask
    // yourself how you know that thing will be traced in the future. If you're
    // not sure, then you could be marking it too late to be noticed. The thing
    // itself will be retained by compaction, but since it will not be traced,
    // anything it refers to could be compacted away.
    let mut module_tracer = ModuleTracer::new(module);

    // Observe what each entry point actually uses.
    log::trace!("tracing entry points");
    let entry_point_maps = module
        .entry_points
        .iter()
        .map(|e| {
            log::trace!("tracing entry point {:?}", e.function.name);

            if let Some(sizes) = e.workgroup_size_overrides {
                for size in sizes.iter().filter_map(|x| *x) {
                    module_tracer.global_expressions_used.insert(size);
                }
            }

            if let Some(task_payload) = e.task_payload {
                module_tracer.global_variables_used.insert(task_payload);
            }
            if let Some(ref mesh_info) = e.mesh_info {
                module_tracer
                    .global_variables_used
                    .insert(mesh_info.output_variable);
                module_tracer
                    .types_used
                    .insert(mesh_info.vertex_output_type);
                module_tracer
                    .types_used
                    .insert(mesh_info.primitive_output_type);
                if let Some(max_vertices_override) = mesh_info.max_vertices_override {
                    module_tracer
                        .global_expressions_used
                        .insert(max_vertices_override);
                }
                if let Some(max_primitives_override) = mesh_info.max_primitives_override {
                    module_tracer
                        .global_expressions_used
                        .insert(max_primitives_override);
                }
            }
            if e.stage == crate::ShaderStage::Task || e.stage == crate::ShaderStage::Mesh {
                // Mesh shaders always need a u32 type, as it is e.g. the type of some
                // expressions. We tolerate its absence here because compaction is
                // infallible, but the module will fail validation.
                if let Some(u32_type) = module.types.iter().find_map(|tuple| {
                    (tuple.1.inner == crate::TypeInner::Scalar(crate::Scalar::U32))
                        .then_some(tuple.0)
                }) {
                    module_tracer.types_used.insert(u32_type);
                }
            }

            let mut used = module_tracer.as_function(&e.function);
            used.trace();
            FunctionMap::from(used)
        })
        .collect::<Vec<_>>();

    // Observe which types, constant expressions, constants, and expressions
    // each function uses, and produce maps for each function from
    // pre-compaction to post-compaction expression handles.
    //
    // The function tracing logic here works in conjunction with
    // `FunctionTracer::trace_call`, which, when tracing a `Statement::Call`
    // to a function not already identified as used, adds the called function
    // to both `functions_used` and `functions_pending`.
    //
    // Called functions are required to appear before their callers in the
    // functions arena (recursion is disallowed). We have already traced the
    // entry point(s) and added any functions called directly by the entry
    // point(s) to `functions_pending`. We proceed by repeatedly tracing the
    // last function in `functions_pending`. By an inductive argument, any
    // functions after the last function in `functions_pending` must be unused.
    //
    // When `KeepUnused` is active, we simply mark all functions as pending,
    // and then trace all of them.
    log::trace!("tracing functions");
    let mut function_maps = HandleMap::with_capacity(module.functions.len());
    if keep_unused.into() {
        module_tracer.functions_used.add_all();
        module_tracer.functions_pending.add_all();
    }
    while let Some(handle) = module_tracer.functions_pending.pop() {
        let function = &module.functions[handle];
        log::trace!("tracing function {function:?}");
        let mut function_tracer = module_tracer.as_function(function);
        function_tracer.trace();
        function_maps.insert(handle, FunctionMap::from(function_tracer));
    }

    // We treat all special types as used by definition.
    log::trace!("tracing special types");
    module_tracer.trace_special_types(&module.special_types);

    log::trace!("tracing global variables");
    if keep_unused.into() {
        module_tracer.global_variables_used.add_all();
    }
    for global in module_tracer.global_variables_used.iter() {
        log::trace!("tracing global {:?}", module.global_variables[global].name);
        module_tracer
            .types_used
            .insert(module.global_variables[global].ty);
        if let Some(init) = module.global_variables[global].init {
            module_tracer.global_expressions_used.insert(init);
        }
    }

    // We treat all named constants as used by definition, unless they have an
    // abstract type as we do not want those reaching the validator.
    log::trace!("tracing named constants");
    for (handle, constant) in module.constants.iter() {
        if constant.name.is_none() || module.types[constant.ty].inner.is_abstract(&module.types) {
            continue;
        }

        log::trace!("tracing constant {:?}", constant.name.as_ref().unwrap());
        module_tracer.constants_used.insert(handle);
        module_tracer.types_used.insert(constant.ty);
        module_tracer.global_expressions_used.insert(constant.init);
    }

    if keep_unused.into() {
        // Treat all named overrides as used.
        for (handle, r#override) in module.overrides.iter() {
            if r#override.name.is_some() && module_tracer.overrides_used.insert(handle) {
                module_tracer.types_used.insert(r#override.ty);
                if let Some(init) = r#override.init {
                    module_tracer.global_expressions_used.insert(init);
                }
            }
        }

        // Treat all named types as used.
        for (handle, ty) in module.types.iter() {
            if ty.name.is_some() {
                module_tracer.types_used.insert(handle);
            }
        }
    }

    module_tracer.type_expression_tandem();

    // Now that we know what is used and what is never touched,
    // produce maps from the `Handle`s that appear in `module` now to
    // the corresponding `Handle`s that will refer to the same items
    // in the compacted module.
    let module_map = ModuleMap::from(module_tracer);

    // Drop unused types from the type arena.
    //
    // `FastIndexSet`s don't have an underlying Vec<T> that we can
    // steal, compact in place, and then rebuild the `FastIndexSet`
    // from. So we have to rebuild the type arena from scratch.
    log::trace!("compacting types");
    let mut new_types = arena::UniqueArena::new();
    for (old_handle, mut ty, span) in module.types.drain_all() {
        if let Some(expected_new_handle) = module_map.types.try_adjust(old_handle) {
            module_map.adjust_type(&mut ty);
            let actual_new_handle = new_types.insert(ty, span);
            assert_eq!(actual_new_handle, expected_new_handle);
        }
    }
    module.types = new_types;
    log::trace!("adjusting special types");
    module_map.adjust_special_types(&mut module.special_types);

    // Drop unused constant expressions, reusing existing storage.
    log::trace!("adjusting constant expressions");
    module.global_expressions.retain_mut(|handle, expr| {
        if module_map.global_expressions.used(handle) {
            module_map.adjust_expression(expr, &module_map.global_expressions);
            true
        } else {
            false
        }
    });

    // Drop unused constants in place, reusing existing storage.
    log::trace!("adjusting constants");
    module.constants.retain_mut(|handle, constant| {
        if module_map.constants.used(handle) {
            module_map.types.adjust(&mut constant.ty);
            module_map.global_expressions.adjust(&mut constant.init);
            true
        } else {
            false
        }
    });

    // Drop unused overrides in place, reusing existing storage.
    log::trace!("adjusting overrides");
    module.overrides.retain_mut(|handle, r#override| {
        if module_map.overrides.used(handle) {
            module_map.types.adjust(&mut r#override.ty);
            if let Some(ref mut init) = r#override.init {
                module_map.global_expressions.adjust(init);
            }
            true
        } else {
            false
        }
    });

    // Adjust workgroup_size_overrides
    log::trace!("adjusting workgroup_size_overrides");
    for e in module.entry_points.iter_mut() {
        if let Some(sizes) = e.workgroup_size_overrides.as_mut() {
            for size in sizes.iter_mut() {
                if let Some(expr) = size.as_mut() {
                    module_map.global_expressions.adjust(expr);
                }
            }
        }
    }

    // Drop unused global variables, reusing existing storage.
    // Adjust used global variables' types and initializers.
    log::trace!("adjusting global variables");
    module.global_variables.retain_mut(|handle, global| {
        if module_map.globals.used(handle) {
            log::trace!("retaining global variable {:?}", global.name);
            module_map.types.adjust(&mut global.ty);
            if let Some(ref mut init) = global.init {
                module_map.global_expressions.adjust(init);
            }
            true
        } else {
            log::trace!("dropping global variable {:?}", global.name);
            false
        }
    });

    // Adjust doc comments
    if let Some(ref mut doc_comments) = module.doc_comments {
        module_map.adjust_doc_comments(doc_comments.as_mut());
    }

    // Temporary storage to help us reuse allocations of existing
    // named expression tables.
    let mut reused_named_expressions = crate::NamedExpressions::default();

    // Drop unused functions. Compact and adjust used functions.
    module.functions.retain_mut(|handle, function| {
        if let Some(map) = function_maps.get(handle) {
            log::trace!("retaining and compacting function {:?}", function.name);
            map.compact(function, &module_map, &mut reused_named_expressions);
            true
        } else {
            log::trace!("dropping function {:?}", function.name);
            false
        }
    });

    // Compact each entry point.
    for (entry, map) in module.entry_points.iter_mut().zip(entry_point_maps.iter()) {
        log::trace!("compacting entry point {:?}", entry.function.name);
        map.compact(
            &mut entry.function,
            &module_map,
            &mut reused_named_expressions,
        );
        if let Some(ref mut task_payload) = entry.task_payload {
            module_map.globals.adjust(task_payload);
        }
        if let Some(ref mut mesh_info) = entry.mesh_info {
            module_map.globals.adjust(&mut mesh_info.output_variable);
            module_map.types.adjust(&mut mesh_info.vertex_output_type);
            module_map
                .types
                .adjust(&mut mesh_info.primitive_output_type);
            if let Some(ref mut max_vertices_override) = mesh_info.max_vertices_override {
                module_map.global_expressions.adjust(max_vertices_override);
            }
            if let Some(ref mut max_primitives_override) = mesh_info.max_primitives_override {
                module_map
                    .global_expressions
                    .adjust(max_primitives_override);
            }
        }
    }
}

struct ModuleTracer<'module> {
    module: &'module crate::Module,

    /// The subset of functions in `functions_used` that have not yet been
    /// traced.
    functions_pending: HandleSet<crate::Function>,

    functions_used: HandleSet<crate::Function>,
    types_used: HandleSet<crate::Type>,
    global_variables_used: HandleSet<crate::GlobalVariable>,
    constants_used: HandleSet<crate::Constant>,
    overrides_used: HandleSet<crate::Override>,
    global_expressions_used: HandleSet<crate::Expression>,
}

impl<'module> ModuleTracer<'module> {
    fn new(module: &'module crate::Module) -> Self {
        Self {
            module,
            functions_pending: HandleSet::for_arena(&module.functions),
            functions_used: HandleSet::for_arena(&module.functions),
            types_used: HandleSet::for_arena(&module.types),
            global_variables_used: HandleSet::for_arena(&module.global_variables),
            constants_used: HandleSet::for_arena(&module.constants),
            overrides_used: HandleSet::for_arena(&module.overrides),
            global_expressions_used: HandleSet::for_arena(&module.global_expressions),
        }
    }

    fn trace_special_types(&mut self, special_types: &crate::SpecialTypes) {
        let crate::SpecialTypes {
            ref ray_desc,
            ref ray_intersection,
            ref ray_vertex_return,
            ref predeclared_types,
            ref external_texture_params,
            ref external_texture_transfer_function,
        } = *special_types;

        if let Some(ray_desc) = *ray_desc {
            self.types_used.insert(ray_desc);
        }
        if let Some(ray_intersection) = *ray_intersection {
            self.types_used.insert(ray_intersection);
        }
        if let Some(ray_vertex_return) = *ray_vertex_return {
            self.types_used.insert(ray_vertex_return);
        }
        // The `external_texture_params` type is generated purely as a
        // convenience to the backends. While it will never actually be used in
        // the IR, it must be marked as used so that it survives compaction.
        if let Some(external_texture_params) = *external_texture_params {
            self.types_used.insert(external_texture_params);
        }
        if let Some(external_texture_transfer_function) = *external_texture_transfer_function {
            self.types_used.insert(external_texture_transfer_function);
        }
        for (_, &handle) in predeclared_types {
            self.types_used.insert(handle);
        }
    }

    /// Traverse types and global expressions in tandem to determine which are used.
    ///
    /// Assuming that all types and global expressions used by other parts of
    /// the module have been added to [`types_used`] and
    /// [`global_expressions_used`], expand those sets to include all types and
    /// global expressions reachable from those.
    ///
    /// [`types_used`]: ModuleTracer::types_used
    /// [`global_expressions_used`]: ModuleTracer::global_expressions_used
    fn type_expression_tandem(&mut self) {
        // For each type T, compute the latest global expression E that T and
        // its predecessors refer to. Given the ordering rules on types and
        // global expressions in valid modules, we can do this with a single
        // forward scan of the type arena. The rules further imply that T can
        // only be referred to by expressions after E.
        let mut max_dep = Vec::with_capacity(self.module.types.len());
        let mut previous = None;
        for (_handle, ty) in self.module.types.iter() {
            previous = core::cmp::max(
                previous,
                match ty.inner {
                    crate::TypeInner::Array { size, .. }
                    | crate::TypeInner::BindingArray { size, .. } => match size {
                        crate::ArraySize::Constant(_) | crate::ArraySize::Dynamic => None,
                        crate::ArraySize::Pending(handle) => self.module.overrides[handle].init,
                    },
                    _ => None,
                },
            );
            max_dep.push(previous);
        }

        // Visit types and global expressions from youngest to oldest.
        //
        // The outer loop visits types. Before visiting each type, the inner
        // loop ensures that all global expressions that could possibly refer to
        // it have been visited. And since the inner loop stop at the latest
        // expression that the type could possibly refer to, we know that we
        // have previously visited any types that might refer to each expression
        // we visit.
        //
        // This lets us assume that any type or expression that is *not* marked
        // as used by the time we visit it is genuinely unused, and can be
        // ignored.
        let mut exprs = self.module.global_expressions.iter().rev().peekable();

        for ((ty_handle, ty), dep) in self.module.types.iter().zip(max_dep).rev() {
            while let Some((expr_handle, expr)) = exprs.next_if(|&(h, _)| Some(h) > dep) {
                if self.global_expressions_used.contains(expr_handle) {
                    self.as_const_expression().trace_expression(expr);
                }
            }
            if self.types_used.contains(ty_handle) {
                self.as_type().trace_type(ty);
            }
        }
        // Visit any remaining expressions.
        for (expr_handle, expr) in exprs {
            if self.global_expressions_used.contains(expr_handle) {
                self.as_const_expression().trace_expression(expr);
            }
        }
    }

    const fn as_type(&mut self) -> types::TypeTracer<'_> {
        types::TypeTracer {
            overrides: &self.module.overrides,
            types_used: &mut self.types_used,
            expressions_used: &mut self.global_expressions_used,
            overrides_used: &mut self.overrides_used,
        }
    }

    const fn as_const_expression(&mut self) -> expressions::ExpressionTracer<'_> {
        expressions::ExpressionTracer {
            constants: &self.module.constants,
            overrides: &self.module.overrides,
            expressions: &self.module.global_expressions,
            types_used: &mut self.types_used,
            global_variables_used: &mut self.global_variables_used,
            constants_used: &mut self.constants_used,
            expressions_used: &mut self.global_expressions_used,
            overrides_used: &mut self.overrides_used,
            global_expressions_used: None,
        }
    }

    pub fn as_function<'tracer>(
        &'tracer mut self,
        function: &'tracer crate::Function,
    ) -> FunctionTracer<'tracer> {
        FunctionTracer {
            function,
            constants: &self.module.constants,
            overrides: &self.module.overrides,
            functions_pending: &mut self.functions_pending,
            functions_used: &mut self.functions_used,
            types_used: &mut self.types_used,
            global_variables_used: &mut self.global_variables_used,
            constants_used: &mut self.constants_used,
            overrides_used: &mut self.overrides_used,
            global_expressions_used: &mut self.global_expressions_used,
            expressions_used: HandleSet::for_arena(&function.expressions),
        }
    }
}

struct ModuleMap {
    functions: HandleMap<crate::Function>,
    types: HandleMap<crate::Type>,
    globals: HandleMap<crate::GlobalVariable>,
    constants: HandleMap<crate::Constant>,
    overrides: HandleMap<crate::Override>,
    global_expressions: HandleMap<crate::Expression>,
}

impl From<ModuleTracer<'_>> for ModuleMap {
    fn from(used: ModuleTracer) -> Self {
        ModuleMap {
            functions: HandleMap::from_set(used.functions_used),
            types: HandleMap::from_set(used.types_used),
            globals: HandleMap::from_set(used.global_variables_used),
            constants: HandleMap::from_set(used.constants_used),
            overrides: HandleMap::from_set(used.overrides_used),
            global_expressions: HandleMap::from_set(used.global_expressions_used),
        }
    }
}

impl ModuleMap {
    fn adjust_special_types(&self, special: &mut crate::SpecialTypes) {
        let crate::SpecialTypes {
            ref mut ray_desc,
            ref mut ray_intersection,
            ref mut ray_vertex_return,
            ref mut predeclared_types,
            ref mut external_texture_params,
            ref mut external_texture_transfer_function,
        } = *special;

        if let Some(ref mut ray_desc) = *ray_desc {
            self.types.adjust(ray_desc);
        }
        if let Some(ref mut ray_intersection) = *ray_intersection {
            self.types.adjust(ray_intersection);
        }

        if let Some(ref mut ray_vertex_return) = *ray_vertex_return {
            self.types.adjust(ray_vertex_return);
        }

        if let Some(ref mut external_texture_params) = *external_texture_params {
            self.types.adjust(external_texture_params);
        }

        if let Some(ref mut external_texture_transfer_function) =
            *external_texture_transfer_function
        {
            self.types.adjust(external_texture_transfer_function);
        }

        for handle in predeclared_types.values_mut() {
            self.types.adjust(handle);
        }
    }

    fn adjust_doc_comments(&self, doc_comments: &mut ir::DocComments) {
        let crate::DocComments {
            module: _,
            types: ref mut doc_types,
            struct_members: ref mut doc_struct_members,
            entry_points: _,
            functions: ref mut doc_functions,
            constants: ref mut doc_constants,
            global_variables: ref mut doc_globals,
        } = *doc_comments;
        log::trace!("adjusting doc comments for types");
        for (mut ty, doc_comment) in core::mem::take(doc_types) {
            if !self.types.used(ty) {
                continue;
            }
            self.types.adjust(&mut ty);
            doc_types.insert(ty, doc_comment);
        }
        log::trace!("adjusting doc comments for struct members");
        for ((mut ty, index), doc_comment) in core::mem::take(doc_struct_members) {
            if !self.types.used(ty) {
                continue;
            }
            self.types.adjust(&mut ty);
            doc_struct_members.insert((ty, index), doc_comment);
        }
        log::trace!("adjusting doc comments for functions");
        for (mut handle, doc_comment) in core::mem::take(doc_functions) {
            if !self.functions.used(handle) {
                continue;
            }
            self.functions.adjust(&mut handle);
            doc_functions.insert(handle, doc_comment);
        }
        log::trace!("adjusting doc comments for constants");
        for (mut constant, doc_comment) in core::mem::take(doc_constants) {
            if !self.constants.used(constant) {
                continue;
            }
            self.constants.adjust(&mut constant);
            doc_constants.insert(constant, doc_comment);
        }
        log::trace!("adjusting doc comments for globals");
        for (mut handle, doc_comment) in core::mem::take(doc_globals) {
            if !self.globals.used(handle) {
                continue;
            }
            self.globals.adjust(&mut handle);
            doc_globals.insert(handle, doc_comment);
        }
    }
}

struct FunctionMap {
    expressions: HandleMap<crate::Expression>,
}

impl From<FunctionTracer<'_>> for FunctionMap {
    fn from(used: FunctionTracer) -> Self {
        FunctionMap {
            expressions: HandleMap::from_set(used.expressions_used),
        }
    }
}

#[test]
fn type_expression_interdependence() {
    let mut module: crate::Module = Default::default();
    let u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Uint,
                width: 4,
            }),
        },
        crate::Span::default(),
    );
    let expr = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(0)),
        crate::Span::default(),
    );
    let type_needs_expression = |module: &mut crate::Module, handle| {
        let override_handle = module.overrides.append(
            crate::Override {
                name: None,
                id: None,
                ty: u32,
                init: Some(handle),
            },
            crate::Span::default(),
        );
        module.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Array {
                    base: u32,
                    size: crate::ArraySize::Pending(override_handle),
                    stride: 4,
                },
            },
            crate::Span::default(),
        )
    };
    let expression_needs_type = |module: &mut crate::Module, handle| {
        module
            .global_expressions
            .append(crate::Expression::ZeroValue(handle), crate::Span::default())
    };
    let expression_needs_expression = |module: &mut crate::Module, handle| {
        module.global_expressions.append(
            crate::Expression::Load { pointer: handle },
            crate::Span::default(),
        )
    };
    let type_needs_type = |module: &mut crate::Module, handle| {
        module.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Array {
                    base: handle,
                    size: crate::ArraySize::Dynamic,
                    stride: 0,
                },
            },
            crate::Span::default(),
        )
    };
    let mut type_name_counter = 0;
    let mut type_needed = |module: &mut crate::Module, handle| {
        let name = Some(format!("type{type_name_counter}"));
        type_name_counter += 1;
        module.types.insert(
            crate::Type {
                name,
                inner: crate::TypeInner::Array {
                    base: handle,
                    size: crate::ArraySize::Dynamic,
                    stride: 0,
                },
            },
            crate::Span::default(),
        )
    };
    let mut override_name_counter = 0;
    let mut expression_needed = |module: &mut crate::Module, handle| {
        let name = Some(format!("override{override_name_counter}"));
        override_name_counter += 1;
        module.overrides.append(
            crate::Override {
                name,
                id: None,
                ty: u32,
                init: Some(handle),
            },
            crate::Span::default(),
        )
    };
    let cmp_modules = |mod0: &crate::Module, mod1: &crate::Module| {
        (mod0.types.iter().collect::<Vec<_>>() == mod1.types.iter().collect::<Vec<_>>())
            && (mod0.global_expressions.iter().collect::<Vec<_>>()
                == mod1.global_expressions.iter().collect::<Vec<_>>())
    };
    // borrow checker breaks without the tmp variables as of Rust 1.83.0
    let expr_end = type_needs_expression(&mut module, expr);
    let ty_trace = type_needs_type(&mut module, expr_end);
    let expr_init = expression_needs_type(&mut module, ty_trace);
    expression_needed(&mut module, expr_init);
    let ty_end = expression_needs_type(&mut module, u32);
    let expr_trace = expression_needs_expression(&mut module, ty_end);
    let ty_init = type_needs_expression(&mut module, expr_trace);
    type_needed(&mut module, ty_init);
    let untouched = module.clone();
    compact(&mut module, KeepUnused::Yes);
    assert!(cmp_modules(&module, &untouched));
    let unused_expr = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(1)),
        crate::Span::default(),
    );
    type_needs_expression(&mut module, unused_expr);
    assert!(!cmp_modules(&module, &untouched));
    compact(&mut module, KeepUnused::Yes);
    assert!(cmp_modules(&module, &untouched));
}

#[test]
fn array_length_override() {
    let mut module: crate::Module = Default::default();
    let ty_bool = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::BOOL),
        },
        crate::Span::default(),
    );
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::U32),
        },
        crate::Span::default(),
    );
    let one = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(1)),
        crate::Span::default(),
    );
    let _unused_override = module.overrides.append(
        crate::Override {
            name: None,
            id: Some(40),
            ty: ty_u32,
            init: None,
        },
        crate::Span::default(),
    );
    let o = module.overrides.append(
        crate::Override {
            name: None,
            id: Some(42),
            ty: ty_u32,
            init: Some(one),
        },
        crate::Span::default(),
    );
    let _ty_array = module.types.insert(
        crate::Type {
            name: Some("array<bool, o>".to_string()),
            inner: crate::TypeInner::Array {
                base: ty_bool,
                size: crate::ArraySize::Pending(o),
                stride: 4,
            },
        },
        crate::Span::default(),
    );

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}

/// Test mutual references between types and expressions via override
/// lengths.
#[test]
fn array_length_override_mutual() {
    use crate::Expression as Ex;
    use crate::Scalar as Sc;
    use crate::TypeInner as Ti;

    let nowhere = crate::Span::default();
    let mut module = crate::Module::default();
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: Ti::Scalar(Sc::U32),
        },
        nowhere,
    );

    // This type is only referred to by the override's init
    // expression, so if we visit that too early, this type will be
    // removed incorrectly.
    let ty_i32 = module.types.insert(
        crate::Type {
            name: None,
            inner: Ti::Scalar(Sc::I32),
        },
        nowhere,
    );

    // An override that the other override's init can refer to.
    let first_override = module.overrides.append(
        crate::Override {
            name: None, // so it is not considered used by definition
            id: Some(41),
            ty: ty_i32,
            init: None,
        },
        nowhere,
    );

    // Initializer expression for the override:
    //
    //     (first_override + 0) as u32
    //
    // The `first_override` makes it an override expression; the `0`
    // gets a use of `ty_i32` in there; and the `as` makes it match
    // the type of `second_override` without actually making
    // `second_override` point at `ty_i32` directly.
    let first_override_expr = module
        .global_expressions
        .append(Ex::Override(first_override), nowhere);
    let zero = module
        .global_expressions
        .append(Ex::ZeroValue(ty_i32), nowhere);
    let sum = module.global_expressions.append(
        Ex::Binary {
            op: crate::BinaryOperator::Add,
            left: first_override_expr,
            right: zero,
        },
        nowhere,
    );
    let init = module.global_expressions.append(
        Ex::As {
            expr: sum,
            kind: crate::ScalarKind::Uint,
            convert: None,
        },
        nowhere,
    );

    // Override that serves as the array's length.
    let second_override = module.overrides.append(
        crate::Override {
            name: None, // so it is not considered used by definition
            id: Some(42),
            ty: ty_u32,
            init: Some(init),
        },
        nowhere,
    );

    // Array type that uses the overload as its length.
    // Since this is named, it is considered used by definition.
    let _ty_array = module.types.insert(
        crate::Type {
            name: Some("delicious_array".to_string()),
            inner: Ti::Array {
                base: ty_u32,
                size: crate::ArraySize::Pending(second_override),
                stride: 4,
            },
        },
        nowhere,
    );

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}

#[test]
fn array_length_expression() {
    let mut module: crate::Module = Default::default();
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::U32),
        },
        crate::Span::default(),
    );
    let _unused_zero = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(0)),
        crate::Span::default(),
    );
    let one = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(1)),
        crate::Span::default(),
    );
    let override_one = module.overrides.append(
        crate::Override {
            name: None,
            id: None,
            ty: ty_u32,
            init: Some(one),
        },
        crate::Span::default(),
    );
    let _ty_array = module.types.insert(
        crate::Type {
            name: Some("array<u32, 1>".to_string()),
            inner: crate::TypeInner::Array {
                base: ty_u32,
                size: crate::ArraySize::Pending(override_one),
                stride: 4,
            },
        },
        crate::Span::default(),
    );

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}

#[test]
fn global_expression_override() {
    let mut module: crate::Module = Default::default();
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::U32),
        },
        crate::Span::default(),
    );

    // This will only be retained if we trace the initializers
    // of overrides referred to by `Expression::Override`
    // in global expressions.
    let expr1 = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(1)),
        crate::Span::default(),
    );

    // This will only be traced via a global `Expression::Override`.
    let o = module.overrides.append(
        crate::Override {
            name: None,
            id: Some(42),
            ty: ty_u32,
            init: Some(expr1),
        },
        crate::Span::default(),
    );

    // This is retained by _p.
    let expr2 = module
        .global_expressions
        .append(crate::Expression::Override(o), crate::Span::default());

    // Since this is named, it will be retained.
    let _p = module.overrides.append(
        crate::Override {
            name: Some("p".to_string()),
            id: None,
            ty: ty_u32,
            init: Some(expr2),
        },
        crate::Span::default(),
    );

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}

#[test]
fn local_expression_override() {
    let mut module: crate::Module = Default::default();
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::U32),
        },
        crate::Span::default(),
    );

    // This will only be retained if we trace the initializers
    // of overrides referred to by `Expression::Override` in a function.
    let expr1 = module.global_expressions.append(
        crate::Expression::Literal(crate::Literal::U32(1)),
        crate::Span::default(),
    );

    // This will be removed by compaction.
    let _unused_override = module.overrides.append(
        crate::Override {
            name: None,
            id: Some(41),
            ty: ty_u32,
            init: None,
        },
        crate::Span::default(),
    );

    // This will only be traced via an `Expression::Override` in a function.
    let o = module.overrides.append(
        crate::Override {
            name: None,
            id: Some(42),
            ty: ty_u32,
            init: Some(expr1),
        },
        crate::Span::default(),
    );

    let mut fun = crate::Function {
        result: Some(crate::FunctionResult {
            ty: ty_u32,
            binding: None,
        }),
        ..crate::Function::default()
    };

    // This is used by the `Return` statement.
    let o_expr = fun
        .expressions
        .append(crate::Expression::Override(o), crate::Span::default());
    fun.body.push(
        crate::Statement::Return {
            value: Some(o_expr),
        },
        crate::Span::default(),
    );

    module.functions.append(fun, crate::Span::default());

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}

#[test]
fn unnamed_constant_type() {
    let mut module = crate::Module::default();
    let nowhere = crate::Span::default();

    // This type is used only by the unnamed constant.
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::U32),
        },
        nowhere,
    );

    // This type is used by the named constant.
    let ty_vec_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Vector {
                size: crate::VectorSize::Bi,
                scalar: crate::Scalar::U32,
            },
        },
        nowhere,
    );

    let unnamed_init = module
        .global_expressions
        .append(crate::Expression::Literal(crate::Literal::U32(0)), nowhere);

    let unnamed_constant = module.constants.append(
        crate::Constant {
            name: None,
            ty: ty_u32,
            init: unnamed_init,
        },
        nowhere,
    );

    // The named constant is initialized using a Splat expression, to
    // give the named constant a type distinct from the unnamed
    // constant's.
    let unnamed_constant_expr = module
        .global_expressions
        .append(crate::Expression::Constant(unnamed_constant), nowhere);
    let named_init = module.global_expressions.append(
        crate::Expression::Splat {
            size: crate::VectorSize::Bi,
            value: unnamed_constant_expr,
        },
        nowhere,
    );

    let _named_constant = module.constants.append(
        crate::Constant {
            name: Some("totally_named".to_string()),
            ty: ty_vec_u32,
            init: named_init,
        },
        nowhere,
    );

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}

#[test]
fn unnamed_override_type() {
    let mut module = crate::Module::default();
    let nowhere = crate::Span::default();

    // This type is used only by the unnamed override.
    let ty_u32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::U32),
        },
        nowhere,
    );

    // This type is used by the named override.
    let ty_i32 = module.types.insert(
        crate::Type {
            name: None,
            inner: crate::TypeInner::Scalar(crate::Scalar::I32),
        },
        nowhere,
    );

    let unnamed_init = module
        .global_expressions
        .append(crate::Expression::Literal(crate::Literal::U32(0)), nowhere);

    let unnamed_override = module.overrides.append(
        crate::Override {
            name: None,
            id: Some(42),
            ty: ty_u32,
            init: Some(unnamed_init),
        },
        nowhere,
    );

    // The named override is initialized using a Splat expression, to
    // give the named override a type distinct from the unnamed
    // override's.
    let unnamed_override_expr = module
        .global_expressions
        .append(crate::Expression::Override(unnamed_override), nowhere);
    let named_init = module.global_expressions.append(
        crate::Expression::As {
            expr: unnamed_override_expr,
            kind: crate::ScalarKind::Sint,
            convert: None,
        },
        nowhere,
    );

    let _named_override = module.overrides.append(
        crate::Override {
            name: Some("totally_named".to_string()),
            id: None,
            ty: ty_i32,
            init: Some(named_init),
        },
        nowhere,
    );

    let mut validator = super::valid::Validator::new(
        super::valid::ValidationFlags::all(),
        super::valid::Capabilities::all(),
    );

    assert!(validator.validate(&module).is_ok());
    compact(&mut module, KeepUnused::Yes);
    assert!(validator.validate(&module).is_ok());
}
