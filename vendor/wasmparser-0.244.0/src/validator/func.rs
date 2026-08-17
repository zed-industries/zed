use super::operators::{Frame, OperatorValidator, OperatorValidatorAllocations};
use crate::{BinaryReader, Result, ValType, VisitOperator};
use crate::{FrameStack, FunctionBody, ModuleArity, Operator, WasmFeatures, WasmModuleResources};

/// Resources necessary to perform validation of a function.
///
/// This structure is created by
/// [`Validator::code_section_entry`](crate::Validator::code_section_entry) and
/// is created per-function in a WebAssembly module. This structure is suitable
/// for sending to other threads while the original
/// [`Validator`](crate::Validator) continues processing other functions.
#[derive(Debug)]
pub struct FuncToValidate<T> {
    /// Reusable, heap allocated resources to drive the Wasm validation.
    pub resources: T,
    /// The core Wasm function index being validated.
    pub index: u32,
    /// The core Wasm type index of the function being validated,
    /// defining the results and parameters to the function.
    pub ty: u32,
    /// The Wasm features enabled to validate the function.
    pub features: WasmFeatures,
}

impl<T: WasmModuleResources> FuncToValidate<T> {
    /// Converts this [`FuncToValidate`] into a [`FuncValidator`] using the
    /// `allocs` provided.
    ///
    /// This method, in conjunction with [`FuncValidator::into_allocations`],
    /// provides a means to reuse allocations across validation of each
    /// individual function. Note that it is also sufficient to call this
    /// method with `Default::default()` if no prior allocations are
    /// available.
    ///
    /// # Panics
    ///
    /// If a `FuncToValidate` was created with an invalid `ty` index then this
    /// function will panic.
    pub fn into_validator(self, allocs: FuncValidatorAllocations) -> FuncValidator<T> {
        let FuncToValidate {
            resources,
            index,
            ty,
            features,
        } = self;
        let validator =
            OperatorValidator::new_func(ty, 0, &features, &resources, allocs.0).unwrap();
        FuncValidator {
            validator,
            resources,
            index,
        }
    }
}

/// Validation context for a WebAssembly function.
///
/// This is a finalized validator which is ready to process a [`FunctionBody`].
/// This is created from the [`FuncToValidate::into_validator`] method.
pub struct FuncValidator<T> {
    validator: OperatorValidator,
    resources: T,
    index: u32,
}

impl<T: WasmModuleResources> ModuleArity for FuncValidator<T> {
    fn sub_type_at(&self, type_idx: u32) -> Option<&crate::SubType> {
        self.resources.sub_type_at(type_idx)
    }

    fn tag_type_arity(&self, at: u32) -> Option<(u32, u32)> {
        let ty = self.resources.tag_at(at)?;
        Some((
            u32::try_from(ty.params().len()).unwrap(),
            u32::try_from(ty.results().len()).unwrap(),
        ))
    }

    fn type_index_of_function(&self, func_idx: u32) -> Option<u32> {
        self.resources.type_index_of_function(func_idx)
    }

    fn func_type_of_cont_type(&self, cont_ty: &crate::ContType) -> Option<&crate::FuncType> {
        let id = cont_ty.0.as_core_type_id()?;
        Some(self.resources.sub_type_at_id(id).unwrap_func())
    }

    fn sub_type_of_ref_type(&self, rt: &crate::RefType) -> Option<&crate::SubType> {
        let id = rt.type_index()?.as_core_type_id()?;
        Some(self.resources.sub_type_at_id(id))
    }

    fn control_stack_height(&self) -> u32 {
        u32::try_from(self.validator.control_stack_height()).unwrap()
    }

    fn label_block(&self, depth: u32) -> Option<(crate::BlockType, crate::FrameKind)> {
        self.validator.jump(depth)
    }
}

/// External handle to the internal allocations used during function validation.
///
/// This is created with either the `Default` implementation or with
/// [`FuncValidator::into_allocations`]. It is then passed as an argument to
/// [`FuncToValidate::into_validator`] to provide a means of reusing allocations
/// between each function.
#[derive(Default)]
pub struct FuncValidatorAllocations(OperatorValidatorAllocations);

impl<T: WasmModuleResources> FuncValidator<T> {
    /// Convenience function to validate an entire function's body.
    ///
    /// You may not end up using this in final implementations because you'll
    /// often want to interleave validation with parsing.
    pub fn validate(&mut self, body: &FunctionBody<'_>) -> Result<()> {
        let mut reader = body.get_binary_reader();
        self.read_locals(&mut reader)?;
        #[cfg(feature = "features")]
        {
            reader.set_features(self.validator.features);
        }
        while !reader.eof() {
            // In a debug build, verify that the validator's pops and pushes to and from
            // the operand stack match the operator's arity.
            #[cfg(debug_assertions)]
            let (ops_before, arity) = {
                let op = reader.peek_operator(&self.visitor(reader.original_position()))?;
                let arity = op.operator_arity(&self.visitor(reader.original_position()));
                (reader.clone(), arity)
            };

            reader.visit_operator(&mut self.visitor(reader.original_position()))??;

            #[cfg(debug_assertions)]
            {
                let (params, results) = arity.ok_or(format_err!(
                    reader.original_position(),
                    "could not calculate operator arity"
                ))?;

                // Analyze the log to determine the actual, externally visible
                // pop/push count. This allows us to hide the fact that we might
                // push and then pop a temporary while validating an
                // instruction, which shouldn't be visible from the outside.
                let mut pop_count = 0;
                let mut push_count = 0;
                for op in self.validator.pop_push_log.drain(..) {
                    match op {
                        true => push_count += 1,
                        false if push_count > 0 => push_count -= 1,
                        false => pop_count += 1,
                    }
                }

                if pop_count != params || push_count != results {
                    panic!(
                        "\
arity mismatch in validation
    operator: {:?}
    expected: {params} -> {results}
    got       {pop_count} -> {push_count}",
                        ops_before.peek_operator(&self.visitor(ops_before.original_position()))?,
                    );
                }
            }
        }
        reader.finish_expression(&self.visitor(reader.original_position()))
    }

    /// Reads the local definitions from the given `BinaryReader`, often sourced
    /// from a `FunctionBody`.
    ///
    /// This function will automatically advance the `BinaryReader` forward,
    /// leaving reading operators up to the caller afterwards.
    pub fn read_locals(&mut self, reader: &mut BinaryReader<'_>) -> Result<()> {
        for _ in 0..reader.read_var_u32()? {
            let offset = reader.original_position();
            let cnt = reader.read()?;
            let ty = reader.read()?;
            self.define_locals(offset, cnt, ty)?;
        }
        Ok(())
    }

    /// Defines locals into this validator.
    ///
    /// This should be used if the application is already reading local
    /// definitions and there's no need to re-parse the function again.
    pub fn define_locals(&mut self, offset: usize, count: u32, ty: ValType) -> Result<()> {
        self.validator
            .define_locals(offset, count, ty, &self.resources)
    }

    /// Validates the next operator in a function.
    ///
    /// This functions is expected to be called once-per-operator in a
    /// WebAssembly function. Each operator's offset in the original binary and
    /// the operator itself are passed to this function to provide more useful
    /// error messages.
    pub fn op(&mut self, offset: usize, operator: &Operator<'_>) -> Result<()> {
        self.visitor(offset).visit_operator(operator)
    }

    /// Get the operator visitor for the next operator in the function.
    ///
    /// The returned visitor is intended to visit just one instruction at the `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmparser::{WasmModuleResources, FuncValidator, FunctionBody, Result};
    /// pub fn validate<R>(validator: &mut FuncValidator<R>, body: &FunctionBody<'_>) -> Result<()>
    /// where R: WasmModuleResources
    /// {
    ///     let mut operator_reader = body.get_binary_reader_for_operators()?;
    ///     while !operator_reader.eof() {
    ///         let mut visitor = validator.visitor(operator_reader.original_position());
    ///         operator_reader.visit_operator(&mut visitor)??;
    ///     }
    ///     operator_reader.finish_expression(&validator.visitor(operator_reader.original_position()))
    /// }
    /// ```
    pub fn visitor<'this, 'a: 'this>(
        &'this mut self,
        offset: usize,
    ) -> impl VisitOperator<'a, Output = Result<()>> + ModuleArity + FrameStack + 'this {
        self.validator.with_resources(&self.resources, offset)
    }

    /// Same as [`FuncValidator::visitor`] except that the returned type
    /// implements the [`VisitSimdOperator`](crate::VisitSimdOperator) trait as
    /// well.
    #[cfg(feature = "simd")]
    pub fn simd_visitor<'this, 'a: 'this>(
        &'this mut self,
        offset: usize,
    ) -> impl crate::VisitSimdOperator<'a, Output = Result<()>> + ModuleArity + 'this {
        self.validator.with_resources_simd(&self.resources, offset)
    }

    /// Returns the Wasm features enabled for this validator.
    pub fn features(&self) -> &WasmFeatures {
        &self.validator.features
    }

    /// Returns the underlying module resources that this validator is using.
    pub fn resources(&self) -> &T {
        &self.resources
    }

    /// The index of the function within the module's function index space that
    /// is being validated.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Returns the number of defined local variables in the function.
    pub fn len_locals(&self) -> u32 {
        self.validator.locals.len_locals()
    }

    /// Returns the type of the local variable at the given `index` if any.
    pub fn get_local_type(&self, index: u32) -> Option<ValType> {
        self.validator.locals.get(index)
    }

    /// Get the current height of the operand stack.
    ///
    /// This returns the height of the whole operand stack for this function,
    /// not just for the current control frame.
    pub fn operand_stack_height(&self) -> u32 {
        self.validator.operand_stack_height() as u32
    }

    /// Returns the optional value type of the value operand at the given
    /// `depth` from the top of the operand stack.
    ///
    /// - Returns `None` if the `depth` is out of bounds.
    /// - Returns `Some(None)` if there is a value with unknown type
    /// at the given `depth`.
    ///
    /// # Note
    ///
    /// A `depth` of 0 will refer to the last operand on the stack.
    pub fn get_operand_type(&self, depth: usize) -> Option<Option<ValType>> {
        self.validator.peek_operand_at(depth)
    }

    /// Returns the number of frames on the control flow stack.
    ///
    /// This returns the height of the whole control stack for this function,
    /// not just for the current control frame.
    pub fn control_stack_height(&self) -> u32 {
        self.validator.control_stack_height() as u32
    }

    /// Returns a shared reference to the control flow [`Frame`] of the
    /// control flow stack at the given `depth` if any.
    ///
    /// Returns `None` if the `depth` is out of bounds.
    ///
    /// # Note
    ///
    /// A `depth` of 0 will refer to the last frame on the stack.
    pub fn get_control_frame(&self, depth: usize) -> Option<&Frame> {
        self.validator.get_frame(depth)
    }

    /// Consumes this validator and returns the underlying allocations that
    /// were used during the validation process.
    ///
    /// The returned value here can be paired with
    /// [`FuncToValidate::into_validator`] to reuse the allocations already
    /// created by this validator.
    pub fn into_allocations(self) -> FuncValidatorAllocations {
        FuncValidatorAllocations(self.validator.into_allocations())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CoreTypeId;
    use crate::{HeapType, Parser, RefType, Validator};
    use alloc::vec::Vec;

    struct EmptyResources(crate::SubType);

    impl Default for EmptyResources {
        fn default() -> Self {
            EmptyResources(crate::SubType {
                supertype_idx: None,
                is_final: true,
                composite_type: crate::CompositeType {
                    inner: crate::CompositeInnerType::Func(crate::FuncType::new([], [])),
                    shared: false,
                    descriptor_idx: None,
                    describes_idx: None,
                },
            })
        }
    }

    impl WasmModuleResources for EmptyResources {
        fn table_at(&self, _at: u32) -> Option<crate::TableType> {
            todo!()
        }
        fn memory_at(&self, _at: u32) -> Option<crate::MemoryType> {
            todo!()
        }
        fn tag_at(&self, _at: u32) -> Option<&crate::FuncType> {
            todo!()
        }
        fn global_at(&self, _at: u32) -> Option<crate::GlobalType> {
            todo!()
        }
        fn sub_type_at(&self, _type_idx: u32) -> Option<&crate::SubType> {
            Some(&self.0)
        }
        fn sub_type_at_id(&self, _id: CoreTypeId) -> &crate::SubType {
            todo!()
        }
        fn type_id_of_function(&self, _at: u32) -> Option<CoreTypeId> {
            todo!()
        }
        fn type_index_of_function(&self, _at: u32) -> Option<u32> {
            todo!()
        }
        fn check_heap_type(&self, _t: &mut HeapType, _offset: usize) -> Result<()> {
            Ok(())
        }
        fn top_type(&self, _heap_type: &HeapType) -> HeapType {
            todo!()
        }
        fn element_type_at(&self, _at: u32) -> Option<crate::RefType> {
            todo!()
        }
        fn is_subtype(&self, _t1: ValType, _t2: ValType) -> bool {
            todo!()
        }
        fn is_shared(&self, _ty: RefType) -> bool {
            todo!()
        }
        fn element_count(&self) -> u32 {
            todo!()
        }
        fn data_count(&self) -> Option<u32> {
            todo!()
        }
        fn is_function_referenced(&self, _idx: u32) -> bool {
            todo!()
        }
        fn has_function_exact_type(&self, _idx: u32) -> bool {
            todo!()
        }
    }

    #[test]
    fn operand_stack_height() {
        let mut v = FuncToValidate {
            index: 0,
            ty: 0,
            resources: EmptyResources::default(),
            features: Default::default(),
        }
        .into_validator(Default::default());

        // Initially zero values on the stack.
        assert_eq!(v.operand_stack_height(), 0);

        // Pushing a constant value makes use have one value on the stack.
        assert!(v.op(0, &Operator::I32Const { value: 0 }).is_ok());
        assert_eq!(v.operand_stack_height(), 1);

        // Entering a new control block does not affect the stack height.
        assert!(
            v.op(
                1,
                &Operator::Block {
                    blockty: crate::BlockType::Empty
                }
            )
            .is_ok()
        );
        assert_eq!(v.operand_stack_height(), 1);

        // Pushing another constant value makes use have two values on the stack.
        assert!(v.op(2, &Operator::I32Const { value: 99 }).is_ok());
        assert_eq!(v.operand_stack_height(), 2);
    }

    fn assert_arity(wat: &str, expected: Vec<Vec<(u32, u32)>>) {
        let wasm = wat::parse_str(wat).unwrap();
        assert!(Validator::new().validate_all(&wasm).is_ok());

        let parser = Parser::new(0);
        let mut validator = Validator::new();

        let mut actual = vec![];

        for payload in parser.parse_all(&wasm) {
            let payload = payload.unwrap();
            match payload {
                crate::Payload::CodeSectionEntry(body) => {
                    let mut arity = vec![];
                    let mut func_validator = validator
                        .code_section_entry(&body)
                        .unwrap()
                        .into_validator(FuncValidatorAllocations::default());
                    let ops = body.get_operators_reader().unwrap();
                    for op in ops.into_iter() {
                        let op = op.unwrap();
                        arity.push(
                            op.operator_arity(&func_validator)
                                .expect("valid operators should have arity"),
                        );
                        func_validator.op(usize::MAX, &op).expect("should be valid");
                    }
                    actual.push(arity);
                }
                p => {
                    validator.payload(&p).unwrap();
                }
            }
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn arity_smoke_test() {
        let wasm = r#"
            (module
                (type $pair (struct (field i32) (field i32)))

                (func $add (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add
                )

                (func $f (param i32 i32) (result (ref null $pair))
                    local.get 0
                    local.get 1
                    call $add
                    if (result (ref null $pair))
                    local.get 0
                    local.get 1
                      struct.new $pair
                    else
                      unreachable
                      i32.add
                      unreachable
                    end
                )
            )
        "#;

        assert_arity(
            wasm,
            vec![
                // $add
                vec![
                    // local.get 0
                    (0, 1),
                    // local.get 1
                    (0, 1),
                    // i32.add
                    (2, 1),
                    // end
                    (1, 1),
                ],
                // $f
                vec![
                    // local.get 0
                    (0, 1),
                    // local.get 1
                    (0, 1),
                    // call $add
                    (2, 1),
                    // if
                    (1, 0),
                    // local.get 0
                    (0, 1),
                    // local.get 1
                    (0, 1),
                    // struct.new $pair
                    (2, 1),
                    // else
                    (1, 0),
                    // unreachable,
                    (0, 0),
                    // i32.add
                    (2, 1),
                    // unreachable
                    (0, 0),
                    // end
                    (1, 1),
                    // implicit end
                    (1, 1),
                ],
            ],
        );
    }

    #[test]
    fn arity_if_no_else_same_params_and_results() {
        let wasm = r#"
            (module
                (func (export "f") (param i64 i32) (result i64)
                    (local.get 0)
                    (local.get 1)
                    ;; If with no else. Same number of params and results.
                    if (param i64) (result i64)
                        drop
                        i64.const -1
                    end
                )
            )
        "#;

        assert_arity(
            wasm,
            vec![vec![
                // local.get 0
                (0, 1),
                // local.get 1
                (0, 1),
                // if
                (2, 1),
                // drop
                (1, 0),
                // i64.const -1
                (0, 1),
                // end
                (1, 1),
                // implicit end
                (1, 1),
            ]],
        );
    }

    #[test]
    fn arity_br_table() {
        let wasm = r#"
            (module
                (func (export "f") (result i32 i32)
                    i32.const 0
                    i32.const 1
                    i32.const 2
                    br_table 0 0
                )
            )
        "#;

        assert_arity(
            wasm,
            vec![vec![
                // i32.const 0
                (0, 1),
                // i32.const 1
                (0, 1),
                // i32.const 2
                (0, 1),
                // br_table
                (3, 0),
                // implicit end
                (2, 2),
            ]],
        );
    }
}
