; Refer to https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/go/injections.scm#L4C1-L16C41
(call_expression
  (selector_expression) @_function
  (#any-of? @_function
    "regexp.Match" "regexp.MatchReader" "regexp.MatchString" "regexp.Compile" "regexp.CompilePOSIX"
    "regexp.MustCompile" "regexp.MustCompilePOSIX")
  (argument_list
    .
    [
      (raw_string_literal)
      (interpreted_string_literal)
    ] @injection.content
    (#set! injection.language "regex")
    ))

; INJECT SQL
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*sql\\s*\\*\\/") ; /* sql */ or /*sql*/
    (#set! injection.language "sql")
)

; INJECT JSON
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*json\\s*\\*\\/") ; /* json */ or /*json*/
    (#set! injection.language "json")
)

; INJECT YAML
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*yaml\\s*\\*\\/") ; /* yaml */ or /*yaml*/
    (#set! injection.language "yaml")
)

; INJECT XML
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*xml\\s*\\*\\/") ; /* xml */ or /*xml*/
    (#set! injection.language "xml")
)

; INJECT HTML
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*html\\s*\\*\\/") ; /* html */ or /*html*/
    (#set! injection.language "html")
)

; INJECT JS
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*js\\s*\\*\\/") ; /* js */ or /*js*/
    (#set! injection.language "javascript")
)

; INJECT CSS
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*css\\s*\\*\\/") ; /* css */ or /*css*/
    (#set! injection.language "css")
)

; INJECT LUA
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*lua\\s*\\*\\/") ; /* lua */ or /*lua*/
    (#set! injection.language "lua")
)

; INJECT BASH
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*bash\\s*\\*\\/") ; /* bash */ or /*bash*/
    (#set! injection.language "bash")
)

; INJECT CSV
(
	[
		; var, const or short declaration of raw or interpreted string literal
		((comment) @comment
  		.
    	(expression_list
     	[
      		(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a literal element (to struct field eg.)
		((comment) @comment
        .
        (literal_element
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content
        ))

        ; when passing as a function parameter
        ((comment) @comment
        .
        [
        	(interpreted_string_literal)
        	(raw_string_literal)
        ] @injection.content)
    ]

    (#match? @comment "^\\/\\*\\s*csv\\s*\\*\\/") ; /* csv */ or /*csv*/
    (#set! injection.language "csv")
)
