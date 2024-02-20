; Identifiers
(identifier) @variable

; ;; If you want type highlighting based on Julia naming conventions (this might collide with mathematical notation)
; ((identifier) @type
;   (match? @type "^[A-Z][^_]"))  ; exception: Highlight `A_foo` sort of identifiers as variables
(macro_identifier) @function.macro

(macro_identifier
  (identifier) @function.macro) ; for any one using the variable highlight

(macro_definition
  name: (identifier) @function.macro)

(quote_expression
  ":" @string.special.symbol
  [
    (identifier)
    (operator)
  ] @string.special.symbol)

(field_expression
  (identifier) @variable.member .)

; Function names
; Definitions
(function_definition
  name: (identifier) @function)

(short_function_definition
  name: (identifier) @function)

(function_definition
  name:
    (field_expression
      (identifier) @function .))

(short_function_definition
  name:
    (field_expression
      (identifier) @function .))

; calls
(call_expression
  (identifier) @function.call)

(call_expression
  (field_expression
    (identifier) @function.call .))

(broadcast_call_expression
  (identifier) @function.call)

(broadcast_call_expression
  (field_expression
    (identifier) @function.call .))

(binary_expression
  (_)
  (operator) @_pipe
  (identifier) @function.call
  (#eq? @_pipe "|>"))

; Builtins
((identifier) @function.builtin
  (#any-of? @function.builtin "_abstracttype" "_apply_iterate" "_apply_pure" "_call_in_world" "_call_in_world_total" "_call_latest" "_equiv_typedef" "_expr" "_primitivetype" "_setsuper!" "_structtype" "_typebody!" "_typevar" "applicable" "apply_type" "arrayref" "arrayset" "arraysize" "const_arrayref" "donotdelete" "fieldtype" "get_binding_type" "getfield" "ifelse" "invoke" "isa" "isdefined" "modifyfield!" "nfields" "replacefield!" "set_binding_type!" "setfield!" "sizeof" "svec" "swapfield!" "throw" "tuple" "typeassert" "typeof"))

; Parameters
(parameter_list
  (identifier) @variable.parameter)

(optional_parameter
  .
  (identifier) @variable.parameter)

(slurp_parameter
  (identifier) @variable.parameter)

(typed_parameter
  parameter: (identifier)? @variable.parameter
  type: (_) @type)

(function_expression
  .
  (identifier) @variable.parameter) ; Single parameter arrow functions

; Types
; Definitions
(abstract_definition
  name: (identifier) @type.definition) @keyword

(primitive_definition
  name: (identifier) @type.definition) @keyword

(struct_definition
  name: (identifier) @type)

(type_clause
  [
    (identifier) @type
    (field_expression
      (identifier) @type .)
  ])

; Annotations
(parametrized_type_expression
  (_) @type
  (curly_expression
    (_) @type))

(type_parameter_list
  (identifier) @type)

(typed_expression
  (identifier) @type .)

(function_definition
  return_type: (identifier) @type)

(short_function_definition
  return_type: (identifier) @type)

(where_clause
  (identifier) @type)

(where_clause
  (curly_expression
    (_) @type))

; Builtins
; This list was generated with:
;
;  istype(x) = typeof(x) === DataType || typeof(x) === UnionAll
;  get_types(m) = filter(x -> istype(Base.eval(m, x)), names(m))
;  type_names = sort(union(get_types(Core), get_types(Base)))
;
((identifier) @type.builtin
  ; format-ignore
  (#any-of? @type.builtin
    "AbstractArray"
    "AbstractChannel"
    "AbstractChar"
    "AbstractDict"
    "AbstractDisplay"
    "AbstractFloat"
    "AbstractIrrational"
    "AbstractLock"
    "AbstractMatch"
    "AbstractMatrix"
    "AbstractPattern"
    "AbstractRange"
    "AbstractSet"
    "AbstractSlices"
    "AbstractString"
    "AbstractUnitRange"
    "AbstractVecOrMat"
    "AbstractVector"
    "Any"
    "ArgumentError"
    "Array"
    "AssertionError"
    "Atomic"
    "BigFloat"
    "BigInt"
    "BitArray"
    "BitMatrix"
    "BitSet"
    "BitVector"
    "Bool"
    "BoundsError"
    "By"
    "CanonicalIndexError"
    "CapturedException"
    "CartesianIndex"
    "CartesianIndices"
    "Cchar"
    "Cdouble"
    "Cfloat"
    "Channel"
    "Char"
    "Cint"
    "Cintmax_t"
    "Clong"
    "Clonglong"
    "Cmd"
    "Colon"
    "ColumnSlices"
    "Complex"
    "ComplexF16"
    "ComplexF32"
    "ComplexF64"
    "ComposedFunction"
    "CompositeException"
    "ConcurrencyViolationError"
    "Condition"
    "Cptrdiff_t"
    "Cshort"
    "Csize_t"
    "Cssize_t"
    "Cstring"
    "Cuchar"
    "Cuint"
    "Cuintmax_t"
    "Culong"
    "Culonglong"
    "Cushort"
    "Cvoid"
    "Cwchar_t"
    "Cwstring"
    "DataType"
    "DenseArray"
    "DenseMatrix"
    "DenseVecOrMat"
    "DenseVector"
    "Dict"
    "DimensionMismatch"
    "Dims"
    "DivideError"
    "DomainError"
    "EOFError"
    "Enum"
    "ErrorException"
    "Exception"
    "ExponentialBackOff"
    "Expr"
    "Float16"
    "Float32"
    "Float64"
    "Function"
    "GlobalRef"
    "HTML"
    "IO"
    "IOBuffer"
    "IOContext"
    "IOStream"
    "IdDict"
    "IndexCartesian"
    "IndexLinear"
    "IndexStyle"
    "InexactError"
    "InitError"
    "Int"
    "Int128"
    "Int16"
    "Int32"
    "Int64"
    "Int8"
    "Integer"
    "InterruptException"
    "InvalidStateException"
    "Irrational"
    "KeyError"
    "LazyString"
    "LinRange"
    "LineNumberNode"
    "LinearIndices"
    "LoadError"
    "Lt"
    "MIME"
    "Matrix"
    "Method"
    "MethodError"
    "Missing"
    "MissingException"
    "Module"
    "NTuple"
    "NamedTuple"
    "Nothing"
    "Number"
    "Ordering"
    "OrdinalRange"
    "OutOfMemoryError"
    "OverflowError"
    "Pair"
    "ParseError"
    "PartialQuickSort"
    "Perm"
    "PermutedDimsArray"
    "Pipe"
    "ProcessFailedException"
    "Ptr"
    "QuoteNode"
    "Rational"
    "RawFD"
    "ReadOnlyMemoryError"
    "Real"
    "ReentrantLock"
    "Ref"
    "Regex"
    "RegexMatch"
    "Returns"
    "ReverseOrdering"
    "RoundingMode"
    "RowSlices"
    "SegmentationFault"
    "Set"
    "Signed"
    "Slices"
    "Some"
    "SpinLock"
    "StackFrame"
    "StackOverflowError"
    "StackTrace"
    "Stateful"
    "StepRange"
    "StepRangeLen"
    "StridedArray"
    "StridedMatrix"
    "StridedVecOrMat"
    "StridedVector"
    "String"
    "StringIndexError"
    "SubArray"
    "SubString"
    "SubstitutionString"
    "Symbol"
    "SystemError"
    "Task"
    "TaskFailedException"
    "Text"
    "TextDisplay"
    "Timer"
    "Tmstruct"
    "Tuple"
    "Type"
    "TypeError"
    "TypeVar"
    "UInt"
    "UInt128"
    "UInt16"
    "UInt32"
    "UInt64"
    "UInt8"
    "UndefInitializer"
    "UndefKeywordError"
    "UndefRefError"
    "UndefVarError"
    "Union"
    "UnionAll"
    "UnitRange"
    "Unsigned"
    "Val"
    "VecElement"
    "VecOrMat"
    "Vector"
    "VersionNumber"
    "WeakKeyDict"
    "WeakRef"))

((identifier) @variable.builtin
  (#any-of? @variable.builtin "begin" "end")
  (#has-ancestor? @variable.builtin index_expression))

((identifier) @variable.builtin
  (#any-of? @variable.builtin "begin" "end")
  (#has-ancestor? @variable.builtin range_expression))

; Keywords
[
  "global"
  "local"
] @keyword

(compound_statement
  [
    "begin"
    "end"
  ] @keyword)

(quote_statement
  [
    "quote"
    "end"
  ] @keyword)

(let_statement
  [
    "let"
    "end"
  ] @keyword)

(if_statement
  [
    "if"
    "end"
  ] @keyword.conditional)

(elseif_clause
  "elseif" @keyword.conditional)

(else_clause
  "else" @keyword.conditional)

(if_clause
  "if" @keyword.conditional) ; `if` clause in comprehensions

(ternary_expression
  [
    "?"
    ":"
  ] @keyword.conditional.ternary)

(try_statement
  [
    "try"
    "end"
  ] @keyword.exception)

(finally_clause
  "finally" @keyword.exception)

(catch_clause
  "catch" @keyword.exception)

(for_statement
  [
    "for"
    "end"
  ] @keyword.repeat)

(while_statement
  [
    "while"
    "end"
  ] @keyword.repeat)

(for_clause
  "for" @keyword.repeat)

[
  (break_statement)
  (continue_statement)
] @keyword.repeat

(module_definition
  [
    "module"
    "baremodule"
    "end"
  ] @keyword.import)

(import_statement
  [
    "import"
    "using"
  ] @keyword.import)

(import_alias
  "as" @keyword.import)

(export_statement
  "export" @keyword.import)

(selected_import
  ":" @punctuation.delimiter)

(struct_definition
  [
    "struct"
    "end"
  ] @keyword)

(macro_definition
  [
    "macro"
    "end"
  ] @keyword)

(function_definition
  [
    "function"
    "end"
  ] @keyword.function)

(do_clause
  [
    "do"
    "end"
  ] @keyword.function)

(return_statement
  "return" @keyword.return)

[
  "const"
  "mutable"
] @type.qualifier

; Operators & Punctuation
[
  "="
  "∈"
  (operator)
] @operator

(adjoint_expression
  "'" @operator)

(range_expression
  ":" @operator)

((operator) @keyword.operator
  (#any-of? @keyword.operator "in" "isa"))

(for_binding
  "in" @keyword.operator)

(where_clause
  "where" @keyword.operator)

(where_expression
  "where" @keyword.operator)

[
  ","
  "."
  ";"
  "::"
  "->"
] @punctuation.delimiter

"..." @punctuation.special

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Literals
(boolean_literal) @boolean

(integer_literal) @number

(float_literal) @number.float

((identifier) @number.float
  (#any-of? @number.float "NaN" "NaN16" "NaN32" "Inf" "Inf16" "Inf32"))

((identifier) @constant.builtin
  (#any-of? @constant.builtin "nothing" "missing"))

(character_literal) @character

(escape_sequence) @escape

(string_literal) @string

(prefixed_string_literal
  prefix: (identifier) @function.macro) @string

(command_literal) @string.special

(prefixed_command_literal
  prefix: (identifier) @function.macro) @string.special

((string_literal) @string.doc
  .
  [
    (module_definition)
    (abstract_definition)
    (struct_definition)
    (function_definition)
    (short_function_definition)
    (assignment)
    (const_statement)
  ])

[
  (line_comment)
  (block_comment)
] @comment
