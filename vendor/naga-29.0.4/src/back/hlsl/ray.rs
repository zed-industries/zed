use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt::Write;

use crate::{
    back::{hlsl::BackendResult, Baked, Level},
    Handle,
};
use crate::{RayQueryIntersection, TypeInner};

impl<W: Write> super::Writer<'_, W> {
    // https://sakibsaikia.github.io/graphics/2022/01/04/Nan-Checks-In-HLSL.html suggests that isnan may not work, unsure if this has changed.
    fn write_not_finite(&mut self, expr: &str) -> BackendResult {
        self.write_contains_flags(&format!("asuint({expr})"), 0x7f800000)
    }

    fn write_nan(&mut self, expr: &str) -> BackendResult {
        write!(self.out, "(")?;
        self.write_not_finite(expr)?;
        write!(self.out, " && ((asuint({expr}) & 0x7fffff) != 0))")?;
        Ok(())
    }

    fn write_contains_flags(&mut self, expr: &str, flags: u32) -> BackendResult {
        write!(self.out, "(({expr} & {flags}) == {flags})")?;
        Ok(())
    }

    // constructs hlsl RayDesc from wgsl RayDesc
    pub(super) fn write_ray_desc_from_ray_desc_constructor_function(
        &mut self,
        module: &crate::Module,
    ) -> BackendResult {
        write!(self.out, "RayDesc RayDescFromRayDesc_(")?;
        self.write_type(module, module.special_types.ray_desc.unwrap())?;
        writeln!(self.out, " arg0) {{")?;
        writeln!(self.out, "    RayDesc ret = (RayDesc)0;")?;
        writeln!(self.out, "    ret.Origin = arg0.origin;")?;
        writeln!(self.out, "    ret.TMin = arg0.tmin;")?;
        writeln!(self.out, "    ret.Direction = arg0.dir;")?;
        writeln!(self.out, "    ret.TMax = arg0.tmax;")?;
        writeln!(self.out, "    return ret;")?;
        writeln!(self.out, "}}")?;
        writeln!(self.out)?;
        Ok(())
    }
    pub(super) fn write_committed_intersection_function(
        &mut self,
        module: &crate::Module,
    ) -> BackendResult {
        self.write_type(module, module.special_types.ray_intersection.unwrap())?;
        write!(self.out, " GetCommittedIntersection(")?;
        self.write_value_type(
            module,
            &TypeInner::RayQuery {
                vertex_return: false,
            },
        )?;
        write!(self.out, " rq, ")?;
        self.write_value_type(module, &TypeInner::Scalar(crate::Scalar::U32))?;
        writeln!(self.out, " rq_tracker) {{")?;
        write!(self.out, "    ")?;
        self.write_type(module, module.special_types.ray_intersection.unwrap())?;
        write!(self.out, " ret = (")?;
        self.write_type(module, module.special_types.ray_intersection.unwrap())?;
        writeln!(self.out, ")0;")?;
        let mut extra_level = Level(0);
        if self.options.ray_query_initialization_tracking {
            // *Technically*, `CommittedStatus` is valid as long as the ray query is initialized, but the metal backend
            // doesn't support this function unless it has finished traversal, so to encourage portable behaviour we
            // disallow it here too.
            write!(self.out, "    if (")?;
            self.write_contains_flags(
                "rq_tracker",
                crate::back::RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            )?;
            writeln!(self.out, ") {{")?;
            extra_level = extra_level.next();
        }
        writeln!(
            self.out,
            "    {extra_level}ret.kind = rq.CommittedStatus();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}if( rq.CommittedStatus() == COMMITTED_NOTHING) {{}} else {{"
        )?;
        writeln!(self.out, "        {extra_level}ret.t = rq.CommittedRayT();")?;
        writeln!(
            self.out,
            "        {extra_level}ret.instance_custom_data = rq.CommittedInstanceID();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.instance_index = rq.CommittedInstanceIndex();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.sbt_record_offset = rq.CommittedInstanceContributionToHitGroupIndex();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.geometry_index = rq.CommittedGeometryIndex();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.primitive_index = rq.CommittedPrimitiveIndex();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}if( rq.CommittedStatus() == COMMITTED_TRIANGLE_HIT ) {{"
        )?;
        writeln!(
            self.out,
            "            {extra_level}ret.barycentrics = rq.CommittedTriangleBarycentrics();"
        )?;
        writeln!(
            self.out,
            "            {extra_level}ret.front_face = rq.CommittedTriangleFrontFace();"
        )?;
        writeln!(self.out, "        {extra_level}}}")?;
        writeln!(
            self.out,
            "        {extra_level}ret.object_to_world = rq.CommittedObjectToWorld4x3();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.world_to_object = rq.CommittedWorldToObject4x3();"
        )?;
        writeln!(self.out, "    {extra_level}}}")?;
        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "    }}")?;
        }
        writeln!(self.out, "    return ret;")?;
        writeln!(self.out, "}}")?;
        writeln!(self.out)?;
        Ok(())
    }
    pub(super) fn write_candidate_intersection_function(
        &mut self,
        module: &crate::Module,
    ) -> BackendResult {
        self.write_type(module, module.special_types.ray_intersection.unwrap())?;
        write!(self.out, " GetCandidateIntersection(")?;
        self.write_value_type(
            module,
            &TypeInner::RayQuery {
                vertex_return: false,
            },
        )?;
        write!(self.out, " rq, ")?;
        self.write_value_type(module, &TypeInner::Scalar(crate::Scalar::U32))?;
        writeln!(self.out, " rq_tracker) {{")?;
        write!(self.out, "    ")?;
        self.write_type(module, module.special_types.ray_intersection.unwrap())?;
        write!(self.out, " ret = (")?;
        self.write_type(module, module.special_types.ray_intersection.unwrap())?;
        writeln!(self.out, ")0;")?;
        let mut extra_level = Level(0);
        if self.options.ray_query_initialization_tracking {
            write!(self.out, "    if (")?;
            self.write_contains_flags("rq_tracker", crate::back::RayQueryPoint::PROCEED.bits())?;
            write!(self.out, " && !")?;
            self.write_contains_flags(
                "rq_tracker",
                crate::back::RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            )?;
            writeln!(self.out, ") {{")?;
            extra_level = extra_level.next();
        }
        writeln!(
            self.out,
            "    {extra_level}CANDIDATE_TYPE kind = rq.CandidateType();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}if (kind == CANDIDATE_NON_OPAQUE_TRIANGLE) {{"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.kind = {};",
            RayQueryIntersection::Triangle as u32
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.t = rq.CandidateTriangleRayT();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.barycentrics = rq.CandidateTriangleBarycentrics();"
        )?;
        writeln!(
            self.out,
            "        {extra_level}ret.front_face = rq.CandidateTriangleFrontFace();"
        )?;
        writeln!(self.out, "    {extra_level}}} else {{")?;
        writeln!(
            self.out,
            "        {extra_level}ret.kind = {};",
            RayQueryIntersection::Aabb as u32
        )?;
        writeln!(self.out, "    {extra_level}}}")?;

        writeln!(
            self.out,
            "    {extra_level}ret.instance_custom_data = rq.CandidateInstanceID();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}ret.instance_index = rq.CandidateInstanceIndex();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}ret.sbt_record_offset = rq.CandidateInstanceContributionToHitGroupIndex();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}ret.geometry_index = rq.CandidateGeometryIndex();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}ret.primitive_index = rq.CandidatePrimitiveIndex();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}ret.object_to_world = rq.CandidateObjectToWorld4x3();"
        )?;
        writeln!(
            self.out,
            "    {extra_level}ret.world_to_object = rq.CandidateWorldToObject4x3();"
        )?;
        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "    }}")?;
        }
        writeln!(self.out, "    return ret;")?;
        writeln!(self.out, "}}")?;
        writeln!(self.out)?;
        Ok(())
    }

    #[expect(clippy::too_many_arguments)]
    pub(super) fn write_initialize_function(
        &mut self,
        module: &crate::Module,
        mut level: Level,
        query: Handle<crate::Expression>,
        acceleration_structure: Handle<crate::Expression>,
        descriptor: Handle<crate::Expression>,
        rq_tracker: &str,
        func_ctx: &crate::back::FunctionCtx<'_>,
    ) -> BackendResult {
        let base_level = level;

        // This prevents variables flowing down a level and causing compile errors.
        writeln!(self.out, "{level}{{")?;
        level = level.next();
        write!(self.out, "{level}")?;
        self.write_type(
            module,
            module
                .special_types
                .ray_desc
                .expect("should have been generated"),
        )?;
        write!(self.out, " naga_desc = ")?;
        self.write_expr(module, descriptor, func_ctx)?;
        writeln!(self.out, ";")?;

        if self.options.ray_query_initialization_tracking {
            // Validate ray extents https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html#ray-extents

            // just for convenience
            writeln!(self.out, "{level}float naga_tmin = naga_desc.tmin;")?;
            writeln!(self.out, "{level}float naga_tmax = naga_desc.tmax;")?;
            writeln!(self.out, "{level}float3 naga_origin = naga_desc.origin;")?;
            writeln!(self.out, "{level}float3 naga_dir = naga_desc.dir;")?;
            writeln!(self.out, "{level}uint naga_flags = naga_desc.flags;")?;
            write!(
                self.out,
                "{level}bool naga_tmin_valid = (naga_tmin >= 0.0) && (naga_tmin <= naga_tmax) && !"
            )?;
            self.write_nan("naga_tmin")?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_tmax_valid = !")?;
            self.write_nan("naga_tmax")?;
            writeln!(self.out, ";")?;
            // Unlike Vulkan it seems that for DX12, it seems only NaN components of the origin and direction are invalid
            write!(self.out, "{level}bool naga_origin_valid = !any(")?;
            self.write_nan("naga_origin")?;
            writeln!(self.out, ");")?;
            write!(self.out, "{level}bool naga_dir_valid = !any(")?;
            self.write_nan("naga_dir")?;
            writeln!(self.out, ");")?;
            write!(self.out, "{level}bool naga_contains_opaque = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::FORCE_OPAQUE.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_no_opaque = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::FORCE_NO_OPAQUE.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_cull_opaque = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::CULL_OPAQUE.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_cull_no_opaque = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::CULL_NO_OPAQUE.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_cull_front = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::CULL_FRONT_FACING.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_cull_back = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::CULL_BACK_FACING.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_skip_triangles = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::SKIP_TRIANGLES.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_contains_skip_aabbs = ")?;
            self.write_contains_flags("naga_flags", crate::RayFlag::SKIP_AABBS.bits())?;
            writeln!(self.out, ";")?;
            // A textified version of the same in the spirv writer
            fn less_than_two_true(mut bools: Vec<&str>) -> Result<String, super::Error> {
                assert!(bools.len() > 1, "Must have multiple booleans!");
                let mut final_expr = String::new();
                while let Some(last_bool) = bools.pop() {
                    for &bool in &bools {
                        if !final_expr.is_empty() {
                            final_expr.push_str("||");
                        }
                        write!(final_expr, " ({last_bool} && {bool}) ")?;
                    }
                }
                Ok(final_expr)
            }
            writeln!(
                self.out,
                "{level}bool naga_contains_skip_triangles_aabbs = {};",
                less_than_two_true(vec![
                    "naga_contains_skip_triangles",
                    "naga_contains_skip_aabbs"
                ])?
            )?;
            writeln!(
                self.out,
                "{level}bool naga_contains_skip_triangles_cull = {};",
                less_than_two_true(vec![
                    "naga_contains_skip_triangles",
                    "naga_contains_cull_back",
                    "naga_contains_cull_front"
                ])?
            )?;
            writeln!(
                self.out,
                "{level}bool naga_contains_multiple_opaque = {};",
                less_than_two_true(vec![
                    "naga_contains_opaque",
                    "naga_contains_no_opaque",
                    "naga_contains_cull_opaque",
                    "naga_contains_cull_no_opaque"
                ])?
            )?;
            writeln!(
                self.out,
                "{level}if (naga_tmin_valid && naga_tmax_valid && naga_origin_valid && naga_dir_valid && !(naga_contains_skip_triangles_aabbs || naga_contains_skip_triangles_cull || naga_contains_multiple_opaque)) {{"
            )?;
            level = level.next();
            writeln!(
                self.out,
                "{level}{rq_tracker} = {rq_tracker} | {};",
                crate::back::RayQueryPoint::INITIALIZED.bits()
            )?;
        }
        write!(self.out, "{level}")?;
        self.write_expr(module, query, func_ctx)?;
        write!(self.out, ".TraceRayInline(")?;
        self.write_expr(module, acceleration_structure, func_ctx)?;
        writeln!(
            self.out,
            ", naga_desc.flags, naga_desc.cull_mask, RayDescFromRayDesc_(naga_desc));"
        )?;
        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "{base_level}    }}")?;
        }
        writeln!(self.out, "{base_level}}}")?;
        Ok(())
    }

    pub(super) fn write_proceed(
        &mut self,
        module: &crate::Module,
        mut level: Level,
        query: Handle<crate::Expression>,
        result: Handle<crate::Expression>,
        rq_tracker: &str,
        func_ctx: &crate::back::FunctionCtx<'_>,
    ) -> BackendResult {
        let base_level = level;
        write!(self.out, "{level}")?;
        let name = Baked(result).to_string();
        writeln!(self.out, "bool {name} = false;")?;
        // This prevents variables flowing down a level and causing compile errors.
        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "{level}{{")?;
            level = level.next();
            write!(self.out, "{level}bool naga_has_initialized = ")?;
            self.write_contains_flags(rq_tracker, crate::back::RayQueryPoint::INITIALIZED.bits())?;
            writeln!(self.out, ";")?;
            write!(self.out, "{level}bool naga_has_finished = ")?;
            self.write_contains_flags(
                rq_tracker,
                crate::back::RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            )?;
            writeln!(self.out, ";")?;
            writeln!(
                self.out,
                "{level}if (naga_has_initialized && !naga_has_finished) {{"
            )?;
            level = level.next();
        }

        write!(self.out, "{level}{name} = ")?;
        self.write_expr(module, query, func_ctx)?;
        writeln!(self.out, ".Proceed();")?;

        if self.options.ray_query_initialization_tracking {
            writeln!(
                self.out,
                "{level}{rq_tracker} = {rq_tracker} | {};",
                crate::back::RayQueryPoint::PROCEED.bits()
            )?;
            writeln!(
                self.out,
                "{level}if (!{name}) {{ {rq_tracker} = {rq_tracker} | {}; }}",
                crate::back::RayQueryPoint::FINISHED_TRAVERSAL.bits()
            )?;
            writeln!(self.out, "{base_level}}}}}")?;
        }

        self.named_expressions.insert(result, name);

        Ok(())
    }

    pub(super) fn write_generate_intersection(
        &mut self,
        module: &crate::Module,
        mut level: Level,
        query: Handle<crate::Expression>,
        hit_t: Handle<crate::Expression>,
        rq_tracker: &str,
        func_ctx: &crate::back::FunctionCtx<'_>,
    ) -> BackendResult {
        let base_level = level;
        if self.options.ray_query_initialization_tracking {
            write!(self.out, "{level}if (")?;
            self.write_contains_flags(rq_tracker, crate::back::RayQueryPoint::PROCEED.bits())?;
            write!(self.out, " && !")?;
            self.write_contains_flags(
                rq_tracker,
                crate::back::RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            )?;
            writeln!(self.out, ") {{")?;
            level = level.next();
            write!(self.out, "{level}CANDIDATE_TYPE naga_kind = ")?;
            self.write_expr(module, query, func_ctx)?;
            writeln!(self.out, ".CandidateType();")?;
            write!(self.out, "{level}float naga_tmin = ")?;
            self.write_expr(module, query, func_ctx)?;
            writeln!(self.out, ".RayTMin();")?;
            write!(self.out, "{level}float naga_tcurrentmax = ")?;
            self.write_expr(module, query, func_ctx)?;
            // This gets initialized to tmax and is updated after each intersection is committed so is valid to call.
            // Note: there is a bug in DXC's spirv backend that makes this technically UB in spirv, but HLSL backend
            // is intended for DXIL, so it should be fine (hopefully).
            writeln!(self.out, ".CommittedRayT();")?;
            write!(
                self.out,
                "{level}if ((naga_kind == CANDIDATE_PROCEDURAL_PRIMITIVE) && (naga_tmin <="
            )?;
            self.write_expr(module, hit_t, func_ctx)?;
            write!(self.out, ") && (")?;
            self.write_expr(module, hit_t, func_ctx)?;
            writeln!(self.out, " <= naga_tcurrentmax)) {{")?;
            level = level.next();
        }

        write!(self.out, "{level}")?;
        self.write_expr(module, query, func_ctx)?;
        write!(self.out, ".CommitProceduralPrimitiveHit(")?;
        self.write_expr(module, hit_t, func_ctx)?;
        writeln!(self.out, ");")?;
        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "{base_level}}}}}")?;
        }
        Ok(())
    }
    pub(super) fn write_confirm_intersection(
        &mut self,
        module: &crate::Module,
        mut level: Level,
        query: Handle<crate::Expression>,
        rq_tracker: &str,
        func_ctx: &crate::back::FunctionCtx<'_>,
    ) -> BackendResult {
        let base_level = level;
        if self.options.ray_query_initialization_tracking {
            write!(self.out, "{level}if (")?;
            self.write_contains_flags(rq_tracker, crate::back::RayQueryPoint::PROCEED.bits())?;
            write!(self.out, " && !")?;
            self.write_contains_flags(
                rq_tracker,
                crate::back::RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            )?;
            writeln!(self.out, ") {{")?;
            level = level.next();
            write!(self.out, "{level}CANDIDATE_TYPE naga_kind = ")?;
            self.write_expr(module, query, func_ctx)?;
            writeln!(self.out, ".CandidateType();")?;
            writeln!(
                self.out,
                "{level}if (naga_kind == CANDIDATE_NON_OPAQUE_TRIANGLE) {{"
            )?;
            level = level.next();
        }

        write!(self.out, "{level}")?;
        self.write_expr(module, query, func_ctx)?;
        writeln!(self.out, ".CommitNonOpaqueTriangleHit();")?;
        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "{base_level}}}}}")?;
        }
        Ok(())
    }

    pub(super) fn write_terminate(
        &mut self,
        module: &crate::Module,
        mut level: Level,
        query: Handle<crate::Expression>,
        rq_tracker: &str,
        func_ctx: &crate::back::FunctionCtx<'_>,
    ) -> BackendResult {
        let base_level = level;
        if self.options.ray_query_initialization_tracking {
            write!(self.out, "{level}if (")?;
            // RayQuery::Abort() can be called any time after RayQuery::TraceRayInline() has been called.
            // from https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html#rayquery-abort
            self.write_contains_flags(rq_tracker, crate::back::RayQueryPoint::INITIALIZED.bits())?;
            writeln!(self.out, ") {{")?;
            level = level.next();
        }

        write!(self.out, "{level}")?;
        self.write_expr(module, query, func_ctx)?;
        writeln!(self.out, ".Abort();")?;

        if self.options.ray_query_initialization_tracking {
            writeln!(self.out, "{base_level}}}")?;
        }

        Ok(())
    }
}
