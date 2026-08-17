use crate::{Naming, Printer, State, name_map};
use anyhow::{Result, bail};
use wasmparser::*;

impl Printer<'_, '_> {
    pub(crate) fn register_component_names(
        &mut self,
        state: &mut State,
        names: ComponentNameSectionReader<'_>,
    ) -> Result<()> {
        for section in names {
            match section? {
                ComponentName::Component { name, .. } => {
                    let name = Naming::new(name, 0, "component", None);
                    state.name = Some(name);
                }
                ComponentName::CoreFuncs(n) => {
                    name_map(&mut state.core.func_names, n, "core-func")?
                }
                ComponentName::CoreTypes(n) => {
                    name_map(&mut state.core.type_names, n, "core-type")?
                }
                ComponentName::CoreTables(n) => {
                    name_map(&mut state.core.table_names, n, "core-table")?
                }
                ComponentName::CoreMemories(n) => {
                    name_map(&mut state.core.memory_names, n, "core-memory")?
                }
                ComponentName::CoreGlobals(n) => {
                    name_map(&mut state.core.global_names, n, "core-global")?
                }
                ComponentName::CoreTags(n) => name_map(&mut state.core.tag_names, n, "core-tag")?,
                ComponentName::CoreModules(n) => {
                    name_map(&mut state.core.module_names, n, "core-module")?
                }
                ComponentName::CoreInstances(n) => {
                    name_map(&mut state.core.instance_names, n, "core-instance")?
                }
                ComponentName::Types(n) => name_map(&mut state.component.type_names, n, "type")?,
                ComponentName::Instances(n) => {
                    name_map(&mut state.component.instance_names, n, "instance")?
                }
                ComponentName::Components(n) => {
                    name_map(&mut state.component.component_names, n, "component")?
                }
                ComponentName::Funcs(n) => name_map(&mut state.component.func_names, n, "func")?,
                ComponentName::Values(n) => name_map(&mut state.component.value_names, n, "value")?,
                ComponentName::Unknown { .. } => (),
            }
        }
        Ok(())
    }

    pub(crate) fn print_core_types(
        &mut self,
        states: &mut Vec<State>,
        parser: CoreTypeSectionReader<'_>,
    ) -> Result<()> {
        for ty in parser.into_iter_with_offsets() {
            let (offset, ty) = ty?;
            self.newline(offset)?;
            self.print_core_type(states, ty)?;
        }

        Ok(())
    }

    pub(crate) fn print_core_type(&mut self, states: &mut Vec<State>, ty: CoreType) -> Result<()> {
        match ty {
            CoreType::Rec(rec) => {
                self.print_rec(states.last_mut().unwrap(), None, rec, true)?;
            }
            CoreType::Module(decls) => {
                self.start_group("core type ")?;
                self.print_name(
                    &states.last().unwrap().core.type_names,
                    states.last().unwrap().core.types.len() as u32,
                )?;
                self.print_module_type(states, decls.into_vec())?;
                self.end_group()?; // `core type` itself
                states.last_mut().unwrap().core.types.push(None);
            }
        }
        Ok(())
    }

    pub(crate) fn print_component_type_ref(&mut self, state: &State, idx: u32) -> Result<()> {
        self.start_group("type ")?;
        self.print_idx(&state.component.type_names, idx)?;
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_primitive_val_type(&mut self, ty: &PrimitiveValType) -> Result<()> {
        match ty {
            PrimitiveValType::Bool => self.print_type_keyword("bool")?,
            PrimitiveValType::S8 => self.print_type_keyword("s8")?,
            PrimitiveValType::U8 => self.print_type_keyword("u8")?,
            PrimitiveValType::S16 => self.print_type_keyword("s16")?,
            PrimitiveValType::U16 => self.print_type_keyword("u16")?,
            PrimitiveValType::S32 => self.print_type_keyword("s32")?,
            PrimitiveValType::U32 => self.print_type_keyword("u32")?,
            PrimitiveValType::S64 => self.print_type_keyword("s64")?,
            PrimitiveValType::U64 => self.print_type_keyword("u64")?,
            PrimitiveValType::F32 => self.print_type_keyword("f32")?,
            PrimitiveValType::F64 => self.print_type_keyword("f64")?,
            PrimitiveValType::Char => self.print_type_keyword("char")?,
            PrimitiveValType::String => self.print_type_keyword("string")?,
            PrimitiveValType::ErrorContext => self.print_type_keyword("error-context")?,
        }
        Ok(())
    }

    pub(crate) fn print_record_type(
        &mut self,
        state: &State,
        fields: &[(&str, ComponentValType)],
    ) -> Result<()> {
        self.start_group("record")?;
        for (name, ty) in fields.iter() {
            self.result.write_str(" ")?;
            self.start_group("field ")?;
            self.print_str(name)?;
            self.result.write_str(" ")?;
            self.print_component_val_type(state, ty)?;
            self.end_group()?;
        }
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_variant_type(
        &mut self,
        state: &State,
        cases: &[VariantCase],
    ) -> Result<()> {
        self.start_group("variant")?;
        for case in cases {
            self.result.write_str(" ")?;
            self.start_group("case ")?;
            self.print_str(case.name)?;

            if let Some(ty) = case.ty {
                self.result.write_str(" ")?;
                self.print_component_val_type(state, &ty)?;
            }

            if let Some(refines) = case.refines {
                self.result.write_str(" ")?;
                self.start_group("refines ")?;
                write!(&mut self.result, "{refines}")?;
                self.end_group()?;
            }
            self.end_group()?;
        }
        self.end_group()?;

        Ok(())
    }

    pub(crate) fn print_list_type(
        &mut self,
        state: &State,
        element_ty: &ComponentValType,
    ) -> Result<()> {
        self.start_group("list ")?;
        self.print_component_val_type(state, element_ty)?;
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_fixed_size_list_type(
        &mut self,
        state: &State,
        element_ty: &ComponentValType,
        elements: u32,
    ) -> Result<()> {
        self.start_group("list ")?;
        self.print_component_val_type(state, element_ty)?;
        self.result.write_str(&format!(" {elements}"))?;
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_tuple_type(
        &mut self,
        state: &State,
        tys: &[ComponentValType],
    ) -> Result<()> {
        self.start_group("tuple")?;
        for ty in tys {
            self.result.write_str(" ")?;
            self.print_component_val_type(state, ty)?;
        }
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_flag_or_enum_type(&mut self, ty: &str, names: &[&str]) -> Result<()> {
        self.start_group(ty)?;
        for name in names {
            self.result.write_str(" ")?;
            self.print_str(name)?;
        }
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_option_type(
        &mut self,
        state: &State,
        inner: &ComponentValType,
    ) -> Result<()> {
        self.start_group("option ")?;
        self.print_component_val_type(state, inner)?;
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_result_type(
        &mut self,
        state: &State,
        ok: Option<ComponentValType>,
        err: Option<ComponentValType>,
    ) -> Result<()> {
        self.start_group("result")?;

        if let Some(ok) = ok {
            self.result.write_str(" ")?;
            self.print_component_val_type(state, &ok)?;
        }

        if let Some(err) = err {
            self.result.write_str(" ")?;
            self.start_group("error ")?;
            self.print_component_val_type(state, &err)?;
            self.end_group()?;
        }

        self.end_group()?;

        Ok(())
    }

    fn print_future_type(&mut self, state: &State, ty: Option<ComponentValType>) -> Result<()> {
        self.start_group("future")?;

        if let Some(ty) = ty {
            self.result.write_str(" ")?;
            self.print_component_val_type(state, &ty)?;
        }

        self.end_group()?;

        Ok(())
    }

    fn print_stream_type(&mut self, state: &State, ty: Option<ComponentValType>) -> Result<()> {
        self.start_group("stream")?;

        if let Some(ty) = ty {
            self.result.write_str(" ")?;
            self.print_component_val_type(state, &ty)?;
        }

        self.end_group()?;

        Ok(())
    }

    pub(crate) fn print_defined_type(
        &mut self,
        state: &State,
        ty: &ComponentDefinedType,
    ) -> Result<()> {
        match ty {
            ComponentDefinedType::Primitive(ty) => self.print_primitive_val_type(ty)?,
            ComponentDefinedType::Record(fields) => self.print_record_type(state, fields)?,
            ComponentDefinedType::Variant(cases) => self.print_variant_type(state, cases)?,
            ComponentDefinedType::List(ty) => self.print_list_type(state, ty)?,
            ComponentDefinedType::FixedSizeList(ty, elements) => {
                self.print_fixed_size_list_type(state, ty, *elements)?
            }
            ComponentDefinedType::Tuple(tys) => self.print_tuple_type(state, tys)?,
            ComponentDefinedType::Flags(names) => self.print_flag_or_enum_type("flags", names)?,
            ComponentDefinedType::Enum(cases) => self.print_flag_or_enum_type("enum", cases)?,
            ComponentDefinedType::Option(ty) => self.print_option_type(state, ty)?,
            ComponentDefinedType::Result { ok, err } => self.print_result_type(state, *ok, *err)?,
            ComponentDefinedType::Own(idx) => {
                self.start_group("own ")?;
                self.print_idx(&state.component.type_names, *idx)?;
                self.end_group()?;
            }
            ComponentDefinedType::Borrow(idx) => {
                self.start_group("borrow ")?;
                self.print_idx(&state.component.type_names, *idx)?;
                self.end_group()?;
            }
            ComponentDefinedType::Future(ty) => self.print_future_type(state, *ty)?,
            ComponentDefinedType::Stream(ty) => self.print_stream_type(state, *ty)?,
        }

        Ok(())
    }

    pub(crate) fn print_component_val_type(
        &mut self,
        state: &State,
        ty: &ComponentValType,
    ) -> Result<()> {
        match ty {
            ComponentValType::Primitive(ty) => self.print_primitive_val_type(ty),
            ComponentValType::Type(idx) => self.print_idx(&state.component.type_names, *idx),
        }
    }

    pub(crate) fn print_module_type(
        &mut self,
        states: &mut Vec<State>,
        decls: Vec<ModuleTypeDeclaration>,
    ) -> Result<()> {
        states.push(State::new(Encoding::Module));
        self.newline_unknown_pos()?;
        self.start_group("module")?;
        for decl in decls {
            self.newline_unknown_pos()?;
            match decl {
                ModuleTypeDeclaration::Type(rec) => {
                    self.print_rec(states.last_mut().unwrap(), None, rec, false)?
                }
                ModuleTypeDeclaration::OuterAlias { kind, count, index } => {
                    self.print_outer_alias(states, kind, count, index)?;
                }
                ModuleTypeDeclaration::Export { name, ty } => {
                    self.start_group("export ")?;
                    self.print_str(name)?;
                    self.result.write_str(" ")?;
                    self.print_import_ty(states.last_mut().unwrap(), &ty, false)?;
                    self.end_group()?;
                }
                ModuleTypeDeclaration::Import(import) => {
                    self.print_import(states.last_mut().unwrap(), &import, false)?;
                }
            }
        }
        self.end_group()?;
        states.pop();
        Ok(())
    }

    pub(crate) fn print_component_type<'a>(
        &mut self,
        states: &mut Vec<State>,
        decls: Vec<ComponentTypeDeclaration<'a>>,
    ) -> Result<()> {
        states.push(State::new(Encoding::Component));
        self.newline_unknown_pos()?;
        self.start_group("component")?;
        for decl in decls {
            self.newline_unknown_pos()?;
            match decl {
                ComponentTypeDeclaration::CoreType(ty) => self.print_core_type(states, ty)?,
                ComponentTypeDeclaration::Type(ty) => self.print_component_type_def(states, ty)?,
                ComponentTypeDeclaration::Alias(alias) => {
                    self.print_component_alias(states, alias)?;
                }
                ComponentTypeDeclaration::Export { name, ty } => {
                    self.start_group("export ")?;
                    self.print_component_kind_name(states.last_mut().unwrap(), ty.kind())?;
                    self.result.write_str(" ")?;
                    self.print_str(name.0)?;
                    self.result.write_str(" ")?;
                    self.print_component_import_ty(states.last_mut().unwrap(), &ty, false)?;
                    self.end_group()?;
                }
                ComponentTypeDeclaration::Import(import) => {
                    self.print_component_import(states.last_mut().unwrap(), &import, true)?
                }
            }
        }
        self.end_group()?;
        states.pop().unwrap();
        Ok(())
    }

    pub(crate) fn print_instance_type<'a>(
        &mut self,
        states: &mut Vec<State>,
        decls: Vec<InstanceTypeDeclaration<'a>>,
    ) -> Result<()> {
        states.push(State::new(Encoding::Component));
        self.newline_unknown_pos()?;
        self.start_group("instance")?;
        for decl in decls {
            self.newline_unknown_pos()?;
            match decl {
                InstanceTypeDeclaration::CoreType(ty) => self.print_core_type(states, ty)?,
                InstanceTypeDeclaration::Type(ty) => self.print_component_type_def(states, ty)?,
                InstanceTypeDeclaration::Alias(alias) => {
                    self.print_component_alias(states, alias)?;
                }
                InstanceTypeDeclaration::Export { name, ty } => {
                    self.start_group("export ")?;
                    self.print_component_kind_name(states.last_mut().unwrap(), ty.kind())?;
                    self.result.write_str(" ")?;
                    self.print_str(name.0)?;
                    self.result.write_str(" ")?;
                    self.print_component_import_ty(states.last_mut().unwrap(), &ty, false)?;
                    self.end_group()?;
                }
            }
        }
        self.end_group()?;
        states.pop().unwrap();
        Ok(())
    }

    pub(crate) fn outer_state(states: &[State], count: u32) -> Result<&State> {
        if count as usize >= states.len() {
            bail!("invalid outer alias count {}", count);
        }

        let count: usize = std::cmp::min(count as usize, states.len() - 1);
        Ok(&states[states.len() - count - 1])
    }

    pub(crate) fn print_outer_alias(
        &mut self,
        states: &mut [State],
        kind: OuterAliasKind,
        count: u32,
        index: u32,
    ) -> Result<()> {
        let state = states.last().unwrap();
        let default_state = State::new(Encoding::Component);
        let outer = match Self::outer_state(states, count) {
            Ok(o) => o,
            Err(e) => {
                write!(self.result, "(; {e} ;) ")?;
                &default_state
            }
        };
        self.start_group("alias outer ")?;
        if let Some(name) = outer.name.as_ref() {
            name.write(self)?;
        } else {
            write!(self.result, "{count}")?;
        }
        self.result.write_str(" ")?;
        match kind {
            OuterAliasKind::Type => {
                self.print_idx(&outer.core.type_names, index)?;
                self.result.write_str(" ")?;
                self.start_group("type ")?;
                self.print_name(&state.core.type_names, state.core.types.len() as u32)?;
            }
        }
        self.end_group()?; // kind
        self.end_group()?; // alias

        let state = states.last_mut().unwrap();
        match kind {
            OuterAliasKind::Type => state.core.types.push(None),
        }

        Ok(())
    }

    pub(crate) fn print_component_func_type(
        &mut self,
        state: &State,
        ty: &ComponentFuncType,
    ) -> Result<()> {
        self.start_group("func")?;
        for (name, ty) in ty.params.iter() {
            self.result.write_str(" ")?;
            self.start_group("param ")?;
            self.print_str(name)?;
            self.result.write_str(" ")?;
            self.print_component_val_type(state, ty)?;
            self.end_group()?;
        }

        if let Some(ty) = &ty.result {
            self.result.write_str(" ")?;
            self.start_group("result ")?;
            self.print_component_val_type(state, ty)?;
            self.end_group()?;
        }

        self.end_group()?;

        Ok(())
    }

    pub(crate) fn print_component_type_def<'a>(
        &mut self,
        states: &mut Vec<State>,
        ty: ComponentType<'a>,
    ) -> Result<()> {
        self.start_group("type ")?;
        {
            let state = states.last_mut().unwrap();
            self.print_name(&state.component.type_names, state.component.types)?;
        }
        match ty {
            ComponentType::Defined(ty) => {
                self.result.write_str(" ")?;
                self.print_defined_type(states.last_mut().unwrap(), &ty)?;
            }
            ComponentType::Func(ty) => {
                self.result.write_str(" ")?;
                self.print_component_func_type(states.last_mut().unwrap(), &ty)?;
            }
            ComponentType::Component(decls) => {
                self.print_component_type(states, decls.into_vec())?;
            }
            ComponentType::Instance(decls) => {
                self.print_instance_type(states, decls.into_vec())?;
            }
            ComponentType::Resource { rep, dtor } => {
                self.result.write_str(" ")?;
                self.start_group("resource ")?;
                self.start_group("rep ")?;
                self.print_valtype(states.last().unwrap(), rep)?;
                self.end_group()?;
                if let Some(dtor) = dtor {
                    self.result.write_str(" ")?;
                    self.start_group("dtor ")?;
                    self.start_group("func ")?;
                    self.print_idx(&states.last().unwrap().core.func_names, dtor)?;
                    self.end_group()?;
                    self.end_group()?;
                }
                self.end_group()?;
            }
        }
        self.end_group()?;

        states.last_mut().unwrap().component.types += 1;

        Ok(())
    }

    pub(crate) fn print_component_types<'a>(
        &mut self,
        states: &mut Vec<State>,
        parser: ComponentTypeSectionReader<'a>,
    ) -> Result<()> {
        for ty in parser.into_iter_with_offsets() {
            let (offset, ty) = ty?;
            self.newline(offset)?;
            self.print_component_type_def(states, ty)?;
        }

        Ok(())
    }

    pub(crate) fn print_component_imports(
        &mut self,
        state: &mut State,
        parser: ComponentImportSectionReader,
    ) -> Result<()> {
        for import in parser.into_iter_with_offsets() {
            let (offset, import) = import?;
            self.newline(offset)?;
            self.print_component_import(state, &import, true)?;
        }

        Ok(())
    }

    pub(crate) fn print_component_import(
        &mut self,
        state: &mut State,
        import: &ComponentImport,
        index: bool,
    ) -> Result<()> {
        self.start_group("import ")?;
        self.print_str(import.name.0)?;
        self.result.write_str(" ")?;
        self.print_component_import_ty(state, &import.ty, index)?;
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_component_import_ty(
        &mut self,
        state: &mut State,
        ty: &ComponentTypeRef,
        index: bool,
    ) -> Result<()> {
        match ty {
            ComponentTypeRef::Module(idx) => {
                self.start_group("core module ")?;
                if index {
                    self.print_name(&state.core.module_names, state.core.modules)?;
                    self.result.write_str(" ")?;
                    state.core.modules += 1;
                }
                self.print_core_type_ref(state, *idx)?;
                self.end_group()?;
            }
            ComponentTypeRef::Func(idx) => {
                self.start_group("func ")?;
                if index {
                    self.print_name(&state.component.func_names, state.component.funcs)?;
                    self.result.write_str(" ")?;
                    state.component.funcs += 1;
                }
                self.print_component_type_ref(state, *idx)?;
                self.end_group()?;
            }
            ComponentTypeRef::Value(ty) => {
                self.start_group("value ")?;
                if index {
                    self.print_name(&state.component.value_names, state.component.values)?;
                    self.result.write_str(" ")?;
                    state.component.values += 1;
                }
                match ty {
                    ComponentValType::Primitive(ty) => self.print_primitive_val_type(ty)?,
                    ComponentValType::Type(idx) => self.print_component_type_ref(state, *idx)?,
                }
                self.end_group()?;
            }
            ComponentTypeRef::Type(bounds) => {
                self.start_group("type ")?;
                if index {
                    self.print_name(&state.component.type_names, state.component.types)?;
                    self.result.write_str(" ")?;
                    state.component.types += 1;
                }
                match bounds {
                    TypeBounds::Eq(idx) => {
                        self.start_group("eq ")?;
                        self.print_idx(&state.component.type_names, *idx)?;
                        self.end_group()?;
                    }
                    TypeBounds::SubResource => {
                        self.start_group("sub ")?;
                        self.print_type_keyword("resource")?;
                        self.end_group()?;
                    }
                };
                self.end_group()?;
            }
            ComponentTypeRef::Instance(idx) => {
                self.start_group("instance ")?;
                if index {
                    self.print_name(&state.component.instance_names, state.component.instances)?;
                    self.result.write_str(" ")?;
                    state.component.instances += 1;
                }
                self.print_component_type_ref(state, *idx)?;
                self.end_group()?;
            }
            ComponentTypeRef::Component(idx) => {
                self.start_group("component ")?;
                if index {
                    self.print_name(&state.component.component_names, state.component.components)?;
                    self.result.write_str(" ")?;
                    state.component.components += 1;
                }
                self.print_component_type_ref(state, *idx)?;
                self.end_group()?;
            }
        }
        Ok(())
    }

    pub(crate) fn print_component_exports(
        &mut self,
        state: &mut State,
        parser: ComponentExportSectionReader,
    ) -> Result<()> {
        for export in parser.into_iter_with_offsets() {
            let (offset, export) = export?;
            self.newline(offset)?;
            self.print_component_export(state, &export, true)?;
        }
        Ok(())
    }

    pub(crate) fn print_component_export(
        &mut self,
        state: &mut State,
        export: &ComponentExport,
        named: bool,
    ) -> Result<()> {
        self.start_group("export ")?;
        if named {
            self.print_component_kind_name(state, export.kind)?;
            self.result.write_str(" ")?;
        }
        self.print_str(export.name.0)?;
        self.result.write_str(" ")?;
        self.print_component_external_kind(state, export.kind, export.index)?;
        if let Some(ty) = &export.ty {
            self.result.write_str(" ")?;
            self.print_component_import_ty(state, &ty, false)?;
        }
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_component_kind_name(
        &mut self,
        state: &mut State,
        kind: ComponentExternalKind,
    ) -> Result<()> {
        match kind {
            ComponentExternalKind::Func => {
                self.print_name(&state.component.func_names, state.component.funcs)?;
                state.component.funcs += 1;
            }
            ComponentExternalKind::Module => {
                self.print_name(&state.core.module_names, state.core.modules)?;
                state.core.modules += 1;
            }
            ComponentExternalKind::Value => {
                self.print_name(&state.component.value_names, state.component.values)?;
                state.component.values += 1;
            }
            ComponentExternalKind::Type => {
                self.print_name(&state.component.type_names, state.component.types)?;
                state.component.types += 1;
            }
            ComponentExternalKind::Instance => {
                self.print_name(&state.component.instance_names, state.component.instances)?;
                state.component.instances += 1;
            }
            ComponentExternalKind::Component => {
                self.print_name(&state.component.component_names, state.component.components)?;
                state.component.components += 1;
            }
        }
        Ok(())
    }

    pub(crate) fn start_component_external_kind_group(
        &mut self,
        kind: ComponentExternalKind,
    ) -> Result<()> {
        match kind {
            ComponentExternalKind::Module => {
                self.start_group("core module ")?;
            }
            ComponentExternalKind::Component => {
                self.start_group("component ")?;
            }
            ComponentExternalKind::Instance => {
                self.start_group("instance ")?;
            }
            ComponentExternalKind::Func => {
                self.start_group("func ")?;
            }
            ComponentExternalKind::Value => {
                self.start_group("value ")?;
            }
            ComponentExternalKind::Type => {
                self.start_group("type ")?;
            }
        }
        Ok(())
    }

    pub(crate) fn print_component_external_kind(
        &mut self,
        state: &State,
        kind: ComponentExternalKind,
        index: u32,
    ) -> Result<()> {
        self.start_component_external_kind_group(kind)?;
        match kind {
            ComponentExternalKind::Module => {
                self.print_idx(&state.core.module_names, index)?;
            }
            ComponentExternalKind::Component => {
                self.print_idx(&state.component.component_names, index)?;
            }
            ComponentExternalKind::Instance => {
                self.print_idx(&state.component.instance_names, index)?;
            }
            ComponentExternalKind::Func => {
                self.print_idx(&state.component.func_names, index)?;
            }
            ComponentExternalKind::Value => {
                self.print_idx(&state.component.value_names, index)?;
            }
            ComponentExternalKind::Type => {
                self.print_idx(&state.component.type_names, index)?;
            }
        }
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_canonical_options(
        &mut self,
        state: &State,
        options: &[CanonicalOption],
    ) -> Result<()> {
        for option in options {
            self.result.write_str(" ")?;
            match option {
                CanonicalOption::UTF8 => self.result.write_str("string-encoding=utf8")?,
                CanonicalOption::UTF16 => self.result.write_str("string-encoding=utf16")?,
                CanonicalOption::CompactUTF16 => {
                    self.result.write_str("string-encoding=latin1+utf16")?
                }
                CanonicalOption::Memory(idx) => {
                    self.start_group("memory ")?;
                    self.print_idx(&state.core.memory_names, *idx)?;
                    self.end_group()?;
                }
                CanonicalOption::Realloc(idx) => {
                    self.start_group("realloc ")?;
                    self.print_idx(&state.core.func_names, *idx)?;
                    self.end_group()?;
                }
                CanonicalOption::PostReturn(idx) => {
                    self.start_group("post-return ")?;
                    self.print_idx(&state.core.func_names, *idx)?;
                    self.end_group()?;
                }
                CanonicalOption::Async => self.print_type_keyword("async")?,
                CanonicalOption::Callback(idx) => {
                    self.start_group("callback ")?;
                    self.print_idx(&state.core.func_names, *idx)?;
                    self.end_group()?;
                }
                CanonicalOption::CoreType(idx) => {
                    self.start_group("core-type ")?;
                    self.print_idx(&state.core.type_names, *idx)?;
                    self.end_group()?;
                }
                CanonicalOption::Gc => self.result.write_str("gc")?,
            }
        }
        Ok(())
    }

    fn print_intrinsic(
        &mut self,
        state: &mut State,
        group: &str,
        body: &dyn Fn(&mut Self, &mut State) -> Result<()>,
    ) -> Result<()> {
        self.start_group("core func ")?;
        self.print_name(&state.core.func_names, state.core.funcs)?;
        self.result.write_str(" ")?;
        self.start_group(group)?;
        body(self, state)?;
        self.end_group()?;
        self.end_group()?;
        debug_assert_eq!(state.core.func_to_type.len(), state.core.funcs as usize);
        state.core.funcs += 1;
        state.core.func_to_type.push(None);
        Ok(())
    }

    pub(crate) fn print_canonical_functions(
        &mut self,
        state: &mut State,
        parser: ComponentCanonicalSectionReader,
    ) -> Result<()> {
        for func in parser.into_iter_with_offsets() {
            let (offset, func) = func?;
            self.newline(offset)?;
            match func {
                CanonicalFunction::Lift {
                    core_func_index,
                    type_index,
                    options,
                } => {
                    self.start_group("func ")?;
                    self.print_name(&state.component.func_names, state.component.funcs)?;
                    self.result.write_str(" ")?;
                    self.start_group("type ")?;
                    self.print_idx(&state.component.type_names, type_index)?;
                    self.end_group()?;
                    self.result.write_str(" ")?;
                    self.start_group("canon lift ")?;
                    self.start_group("core func ")?;
                    self.print_idx(&state.core.func_names, core_func_index)?;
                    self.end_group()?;
                    self.print_canonical_options(state, &options)?;
                    self.end_group()?;
                    self.end_group()?;
                    state.component.funcs += 1;
                }
                CanonicalFunction::Lower {
                    func_index,
                    options,
                } => {
                    self.print_intrinsic(state, "canon lower ", &|me, state| {
                        me.start_group("func ")?;
                        me.print_idx(&state.component.func_names, func_index)?;
                        me.end_group()?;
                        me.print_canonical_options(state, &options)
                    })?;
                }
                CanonicalFunction::ResourceNew { resource } => {
                    self.print_intrinsic(state, "canon resource.new ", &|me, state| {
                        me.print_idx(&state.component.type_names, resource)
                    })?;
                }
                CanonicalFunction::ResourceDrop { resource } => {
                    self.print_intrinsic(state, "canon resource.drop ", &|me, state| {
                        me.print_idx(&state.component.type_names, resource)
                    })?;
                }
                CanonicalFunction::ResourceDropAsync { resource } => {
                    self.print_intrinsic(state, "canon resource.drop ", &|me, state| {
                        me.print_idx(&state.component.type_names, resource)?;
                        me.print_type_keyword(" async")
                    })?;
                }
                CanonicalFunction::ResourceRep { resource } => {
                    self.print_intrinsic(state, "canon resource.rep ", &|me, state| {
                        me.print_idx(&state.component.type_names, resource)
                    })?;
                }
                CanonicalFunction::ThreadSpawnRef { func_ty_index } => {
                    self.print_intrinsic(state, "canon thread.spawn_ref ", &|me, state| {
                        me.print_idx(&state.core.type_names, func_ty_index)
                    })?;
                }
                CanonicalFunction::ThreadSpawnIndirect {
                    func_ty_index,
                    table_index,
                } => {
                    self.print_intrinsic(state, "canon thread.spawn_indirect ", &|me, state| {
                        me.print_idx(&state.core.type_names, func_ty_index)?;
                        me.result.write_str(" ")?;
                        me.start_group("table ")?;
                        me.print_idx(&state.core.table_names, table_index)?;
                        me.end_group()
                    })?;
                }
                CanonicalFunction::ThreadAvailableParallelism => {
                    self.print_intrinsic(state, "canon thread.available_parallelism", &|_, _| {
                        Ok(())
                    })?;
                }
                CanonicalFunction::BackpressureSet => {
                    self.print_intrinsic(state, "canon backpressure.set", &|_, _| Ok(()))?;
                }
                CanonicalFunction::TaskReturn { result, options } => {
                    self.print_intrinsic(state, "canon task.return", &|me, state| {
                        if let Some(ty) = result {
                            me.result.write_str(" ")?;
                            me.start_group("result ")?;
                            me.print_component_val_type(state, &ty)?;
                            me.end_group()?;
                        }
                        me.print_canonical_options(state, &options)?;
                        Ok(())
                    })?;
                }
                CanonicalFunction::TaskCancel => {
                    self.print_intrinsic(state, "canon task.cancel", &|_, _| Ok(()))?;
                }
                CanonicalFunction::ContextGet(i) => {
                    self.print_intrinsic(state, "canon context.get", &|me, _state| {
                        write!(me.result, " i32 {i}")?;
                        Ok(())
                    })?;
                }
                CanonicalFunction::ContextSet(i) => {
                    self.print_intrinsic(state, "canon context.set", &|me, _state| {
                        write!(me.result, " i32 {i}")?;
                        Ok(())
                    })?;
                }
                CanonicalFunction::Yield { async_ } => {
                    self.print_intrinsic(state, "canon yield", &|me, _| {
                        if async_ {
                            me.print_type_keyword(" async")?;
                        }
                        Ok(())
                    })?;
                }
                CanonicalFunction::SubtaskDrop => {
                    self.print_intrinsic(state, "canon subtask.drop", &|_, _| Ok(()))?;
                }
                CanonicalFunction::SubtaskCancel { async_ } => {
                    self.print_intrinsic(state, "canon subtask.cancel", &|me, _| {
                        if async_ {
                            me.print_type_keyword(" async")?;
                        }
                        Ok(())
                    })?;
                }
                CanonicalFunction::StreamNew { ty } => {
                    self.print_intrinsic(state, "canon stream.new ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)
                    })?;
                }
                CanonicalFunction::StreamRead { ty, options } => {
                    self.print_intrinsic(state, "canon stream.read ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        me.print_canonical_options(state, &options)
                    })?;
                }
                CanonicalFunction::StreamWrite { ty, options } => {
                    self.print_intrinsic(state, "canon stream.write ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        me.print_canonical_options(state, &options)
                    })?;
                }
                CanonicalFunction::StreamCancelRead { ty, async_ } => {
                    self.print_intrinsic(state, "canon stream.cancel-read ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        if async_ {
                            me.print_type_keyword(" async")?;
                        }
                        Ok(())
                    })?;
                }
                CanonicalFunction::StreamCancelWrite { ty, async_ } => {
                    self.print_intrinsic(state, "canon stream.cancel-write ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        if async_ {
                            me.print_type_keyword(" async")?;
                        }
                        Ok(())
                    })?;
                }
                CanonicalFunction::StreamDropReadable { ty } => {
                    self.print_intrinsic(state, "canon stream.drop-readable ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)
                    })?;
                }
                CanonicalFunction::StreamDropWritable { ty } => {
                    self.print_intrinsic(state, "canon stream.drop-writable ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)
                    })?;
                }
                CanonicalFunction::FutureNew { ty } => {
                    self.print_intrinsic(state, "canon future.new ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)
                    })?;
                }
                CanonicalFunction::FutureRead { ty, options } => {
                    self.print_intrinsic(state, "canon future.read ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        me.print_canonical_options(state, &options)
                    })?;
                }
                CanonicalFunction::FutureWrite { ty, options } => {
                    self.print_intrinsic(state, "canon future.write ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        me.print_canonical_options(state, &options)
                    })?;
                }
                CanonicalFunction::FutureCancelRead { ty, async_ } => {
                    self.print_intrinsic(state, "canon future.cancel-read ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        if async_ {
                            me.print_type_keyword(" async")?;
                        }
                        Ok(())
                    })?;
                }
                CanonicalFunction::FutureCancelWrite { ty, async_ } => {
                    self.print_intrinsic(state, "canon future.cancel-write ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)?;
                        if async_ {
                            me.print_type_keyword(" async")?;
                        }
                        Ok(())
                    })?;
                }
                CanonicalFunction::FutureDropReadable { ty } => {
                    self.print_intrinsic(state, "canon future.drop-readable ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)
                    })?;
                }
                CanonicalFunction::FutureDropWritable { ty } => {
                    self.print_intrinsic(state, "canon future.drop-writable ", &|me, state| {
                        me.print_idx(&state.component.type_names, ty)
                    })?;
                }
                CanonicalFunction::ErrorContextNew { options } => {
                    self.print_intrinsic(state, "canon error-context.new", &|me, state| {
                        me.print_canonical_options(state, &options)
                    })?;
                }
                CanonicalFunction::ErrorContextDebugMessage { options } => {
                    self.print_intrinsic(
                        state,
                        "canon error-context.debug-message",
                        &|me, state| me.print_canonical_options(state, &options),
                    )?;
                }
                CanonicalFunction::ErrorContextDrop => {
                    self.print_intrinsic(state, "canon error-context.drop", &|_, _| Ok(()))?;
                }
                CanonicalFunction::WaitableSetNew => {
                    self.print_intrinsic(state, "canon waitable-set.new", &|_, _| Ok(()))?;
                }
                CanonicalFunction::WaitableSetWait { async_, memory } => {
                    self.print_intrinsic(state, "canon waitable-set.wait ", &|me, state| {
                        if async_ {
                            me.result.write_str("async ")?;
                        }
                        me.start_group("memory ")?;
                        me.print_idx(&state.core.memory_names, memory)?;
                        me.end_group()
                    })?;
                }
                CanonicalFunction::WaitableSetPoll { async_, memory } => {
                    self.print_intrinsic(state, "canon waitable-set.poll ", &|me, state| {
                        if async_ {
                            me.result.write_str("async ")?;
                        }
                        me.start_group("memory ")?;
                        me.print_idx(&state.core.memory_names, memory)?;
                        me.end_group()
                    })?;
                }
                CanonicalFunction::WaitableSetDrop => {
                    self.print_intrinsic(state, "canon waitable-set.drop", &|_, _| Ok(()))?;
                }
                CanonicalFunction::WaitableJoin => {
                    self.print_intrinsic(state, "canon waitable.join", &|_, _| Ok(()))?;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn print_instances(
        &mut self,
        state: &mut State,
        parser: InstanceSectionReader,
    ) -> Result<()> {
        for instance in parser.into_iter_with_offsets() {
            let (offset, instance) = instance?;
            self.newline(offset)?;
            self.start_group("core instance ")?;
            self.print_name(&state.core.instance_names, state.core.instances)?;
            match instance {
                Instance::Instantiate { module_index, args } => {
                    self.result.write_str(" ")?;
                    self.start_group("instantiate ")?;
                    self.print_idx(&state.core.module_names, module_index)?;
                    for arg in args.iter() {
                        self.newline(offset)?;
                        self.print_instantiation_arg(state, arg)?;
                    }
                    self.end_group()?;
                    state.core.instances += 1;
                }
                Instance::FromExports(exports) => {
                    for export in exports.iter() {
                        self.newline(offset)?;
                        self.print_export(state, export)?;
                    }
                    state.core.instances += 1;
                }
            }
            self.end_group()?;
        }
        Ok(())
    }

    pub(crate) fn print_component_instances(
        &mut self,
        state: &mut State,
        parser: ComponentInstanceSectionReader,
    ) -> Result<()> {
        for instance in parser.into_iter_with_offsets() {
            let (offset, instance) = instance?;
            self.newline(offset)?;
            self.start_group("instance ")?;
            self.print_name(&state.component.instance_names, state.component.instances)?;
            state.component.instances += 1;
            match instance {
                ComponentInstance::Instantiate {
                    component_index,
                    args,
                } => {
                    self.result.write_str(" ")?;
                    self.start_group("instantiate ")?;
                    self.print_idx(&state.component.component_names, component_index)?;
                    for arg in args.iter() {
                        self.newline(offset)?;
                        self.print_component_instantiation_arg(state, arg)?;
                    }
                    self.end_group()?;
                }
                ComponentInstance::FromExports(exports) => {
                    for export in exports.iter() {
                        self.newline(offset)?;
                        self.print_component_export(state, export, false)?;
                    }
                }
            }
            self.end_group()?;
        }
        Ok(())
    }

    pub(crate) fn print_instantiation_arg(
        &mut self,
        state: &State,
        arg: &InstantiationArg,
    ) -> Result<()> {
        self.start_group("with ")?;
        self.print_str(arg.name)?;
        self.result.write_str(" ")?;
        match arg.kind {
            InstantiationArgKind::Instance => {
                self.start_group("instance ")?;
                self.print_idx(&state.core.instance_names, arg.index)?;
                self.end_group()?;
            }
        }
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_component_instantiation_arg(
        &mut self,
        state: &State,
        arg: &ComponentInstantiationArg,
    ) -> Result<()> {
        self.start_group("with ")?;
        self.print_str(arg.name)?;
        self.result.write_str(" ")?;
        self.print_component_external_kind(state, arg.kind, arg.index)?;
        self.end_group()?;
        Ok(())
    }

    pub(crate) fn print_component_start(
        &mut self,
        state: &mut State,
        pos: usize,
        start: ComponentStartFunction,
    ) -> Result<()> {
        self.newline(pos)?;
        self.start_group("start ")?;
        self.print_idx(&state.component.func_names, start.func_index)?;

        for arg in start.arguments.iter() {
            self.result.write_str(" ")?;
            self.start_group("value ")?;
            self.print_idx(&state.component.value_names, *arg)?;
            self.end_group()?;
        }

        for _ in 0..start.results {
            self.result.write_str(" ")?;
            self.start_group("result ")?;
            self.start_group("value ")?;
            self.print_name(&state.component.value_names, state.component.values)?;
            self.end_group()?;
            self.end_group()?;
            state.component.values += 1;
        }

        self.end_group()?; // start

        Ok(())
    }

    pub(crate) fn print_component_aliases(
        &mut self,
        states: &mut [State],
        parser: ComponentAliasSectionReader,
    ) -> Result<()> {
        for alias in parser.into_iter_with_offsets() {
            let (offset, alias) = alias?;
            self.newline(offset)?;
            self.print_component_alias(states, alias)?;
        }
        Ok(())
    }

    pub(crate) fn print_component_alias(
        &mut self,
        states: &mut [State],
        alias: ComponentAlias<'_>,
    ) -> Result<()> {
        match alias {
            ComponentAlias::InstanceExport {
                kind,
                instance_index,
                name,
            } => {
                let state = states.last_mut().unwrap();
                self.start_group("alias export ")?;
                self.print_idx(&state.component.instance_names, instance_index)?;
                self.result.write_str(" ")?;
                self.print_str(name)?;
                self.result.write_str(" ")?;
                self.start_component_external_kind_group(kind)?;
                self.print_component_kind_name(state, kind)?;
                self.end_group()?;

                self.end_group()?; // alias export
            }
            ComponentAlias::CoreInstanceExport {
                instance_index,
                kind,
                name,
            } => {
                let state = states.last_mut().unwrap();
                self.start_group("alias core export ")?;
                self.print_idx(&state.core.instance_names, instance_index)?;
                self.result.write_str(" ")?;
                self.print_str(name)?;
                self.result.write_str(" ")?;
                match kind {
                    ExternalKind::Func => {
                        self.start_group("core func ")?;
                        self.print_name(&state.core.func_names, state.core.funcs)?;
                        self.end_group()?;
                        debug_assert_eq!(state.core.func_to_type.len(), state.core.funcs as usize);
                        state.core.funcs += 1;
                        state.core.func_to_type.push(None)
                    }
                    ExternalKind::Table => {
                        self.start_group("core table ")?;
                        self.print_name(&state.core.table_names, state.core.tables)?;
                        self.end_group()?;
                        state.core.tables += 1;
                    }
                    ExternalKind::Memory => {
                        self.start_group("core memory ")?;
                        self.print_name(&state.core.memory_names, state.core.memories)?;
                        self.end_group()?;
                        state.core.memories += 1;
                    }
                    ExternalKind::Global => {
                        self.start_group("core global ")?;
                        self.print_name(&state.core.global_names, state.core.globals)?;
                        self.end_group()?;
                        state.core.globals += 1;
                    }
                    ExternalKind::Tag => {
                        self.start_group("core tag ")?;
                        self.print_name(&state.core.tag_names, state.core.tags)?;
                        self.end_group()?;
                        debug_assert_eq!(state.core.tag_to_type.len(), state.core.tags as usize);
                        state.core.tags += 1;
                        state.core.tag_to_type.push(None)
                    }
                }
                self.end_group()?; // alias export
            }

            ComponentAlias::Outer { kind, count, index } => {
                let state = states.last().unwrap();
                let default_state = State::new(Encoding::Component);
                let outer = match Self::outer_state(states, count) {
                    Ok(o) => o,
                    Err(e) => {
                        write!(self.result, "(; {e} ;) ")?;
                        &default_state
                    }
                };
                self.start_group("alias outer ")?;
                if let Some(name) = outer.name.as_ref() {
                    name.write(self)?;
                } else {
                    write!(self.result, "{count}")?;
                }
                self.result.write_str(" ")?;
                match kind {
                    ComponentOuterAliasKind::CoreModule => {
                        self.print_idx(&outer.core.module_names, index)?;
                        self.result.write_str(" ")?;
                        self.start_group("core module ")?;
                        self.print_name(&state.core.module_names, state.core.modules)?;
                    }
                    ComponentOuterAliasKind::CoreType => {
                        self.print_idx(&outer.core.type_names, index)?;
                        self.result.write_str(" ")?;
                        self.start_group("core type ")?;
                        self.print_name(&state.core.type_names, state.core.types.len() as u32)?;
                    }
                    ComponentOuterAliasKind::Type => {
                        self.print_idx(&outer.component.type_names, index)?;
                        self.result.write_str(" ")?;
                        self.start_group("type ")?;
                        self.print_name(&state.component.type_names, state.component.types)?;
                    }
                    ComponentOuterAliasKind::Component => {
                        self.print_idx(&outer.component.component_names, index)?;
                        self.result.write_str(" ")?;
                        self.start_group("component ")?;
                        self.print_name(
                            &state.component.component_names,
                            state.component.components,
                        )?;
                    }
                }
                self.end_group()?; // kind
                self.end_group()?; // alias

                let state = states.last_mut().unwrap();
                match kind {
                    ComponentOuterAliasKind::CoreModule => state.core.modules += 1,
                    ComponentOuterAliasKind::CoreType => state.core.types.push(None),
                    ComponentOuterAliasKind::Type => state.component.types += 1,
                    ComponentOuterAliasKind::Component => state.component.components += 1,
                }
            }
        }
        Ok(())
    }
}
