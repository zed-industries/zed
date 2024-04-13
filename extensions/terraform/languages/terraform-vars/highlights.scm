; https://github.com/nvim-treesitter/nvim-treesitter/blob/cb79d2446196d25607eb1d982c96939abdf67b8e/queries/hcl/highlights.scm
; highlights.scm
[
  "!"
  "\*"
  "/"
  "%"
  "\+"
  "-"
  ">"
  ">="
  "<"
  "<="
  "=="
  "!="
  "&&"
  "||"
] @operator

[
  "{"
  "}"
  "["
  "]"
  "("
  ")"
] @punctuation.bracket

[
  "."
  ".*"
  ","
  "[*]"
] @punctuation.delimiter

[
  (ellipsis)
  "\?"
  "=>"
] @punctuation.special

[
  ":"
  "="
] @punctuation

[
  "for"
  "endfor"
  "in"
  "if"
  "else"
  "endif"
] @keyword

[
  (quoted_template_start) ; "
  (quoted_template_end) ; "
  (template_literal) ; non-interpolation/directive content
] @string

[
  (heredoc_identifier) ; END
  (heredoc_start) ; << or <<-
] @punctuation.delimiter

[
  (template_interpolation_start) ; ${
  (template_interpolation_end) ; }
  (template_directive_start) ; %{
  (template_directive_end) ; }
  (strip_marker) ; ~
] @punctuation.special

(numeric_lit) @number

(bool_lit) @boolean

(null_lit) @constant

(comment) @comment

(identifier) @variable

(body
  (block
    (identifier) @keyword))

(body
  (block
    (body
      (block
        (identifier) @type))))

(function_call
  (identifier) @function)

(attribute
  (identifier) @variable)

; { key: val }
;
; highlight identifier keys as though they were block attributes
(object_elem
  key:
    (expression
      (variable_expr
        (identifier) @variable)))

; var.foo, data.bar
;
; first element in get_attr is a variable.builtin or a reference to a variable.builtin
(expression
  (variable_expr
    (identifier) @variable)
  (get_attr
    (identifier) @variable))

; https://github.com/nvim-treesitter/nvim-treesitter/blob/cb79d2446196d25607eb1d982c96939abdf67b8e/queries/terraform/highlights.scm
; Terraform specific references
;
;
; local/module/data/var/output
(expression
  (variable_expr
    (identifier) @variable
    (#any-of? @variable "data" "var" "local" "module" "output"))
  (get_attr
    (identifier) @variable))

; path.root/cwd/module
(expression
  (variable_expr
    (identifier) @type
    (#eq? @type "path"))
  (get_attr
    (identifier) @variable
    (#any-of? @variable "root" "cwd" "module")))

; terraform.workspace
(expression
  (variable_expr
    (identifier) @type
    (#eq? @type "terraform"))
  (get_attr
    (identifier) @variable
    (#any-of? @variable "workspace")))

; Terraform specific keywords
; FIXME: ideally only for identifiers under a `variable` block to minimize false positives
((identifier) @type
  (#any-of? @type "bool" "string" "number" "object" "tuple" "list" "map" "set" "any"))

(object_elem
  val:
    (expression
      (variable_expr
        (identifier) @type
        (#any-of? @type "bool" "string" "number" "object" "tuple" "list" "map" "set" "any"))))
