# Adding New Languages to Zed

TBD: Explain all the steps necessary to add a language to Zed:

- Link to explanation of how extensions download and install
- Link to tree sitter in Zed


## LSP

Zed uses the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) to provide language support. This means, in theory, we can support any language that has an LSP server.

- TBD: Document what LSP is, ho

## Syntax Highlighting

TBD: Explain how syntax highlighting works in Zed.


### Defining syntax highlighting rules

We use tree-sitter queries to match certain properties to highlight.

#### Simple Example:

```scheme
(property_identifier) @property
```

```ts
const font: FontFamily = {
  weight: "normal",
  underline: false,
  italic: false,
};
```

Match a property identifier and highlight it using the identifier `@property`. In the above example, `weight`, `underline`, and `italic` would be highlighted.

#### Complex example:

```scheme
(_
  return_type: (type_annotation
    [
      (type_identifier) @type.return
      (generic_type
          name: (type_identifier) @type.return)
    ]))
```

```ts
function buildDefaultSyntax(colorScheme: Theme): Partial<Syntax> {
  // ...
}
```

Match a function return type, and highlight the type using the identifier `@type.return`. In the above example, `Partial` would be highlighted.

#### Example - Typescript

Here is an example portion of our `highlights.scm` for TypeScript:

```scheme
; crates/zed/src/languages/typescript/highlights.scm

; Variables

(identifier) @variable

; Properties

(property_identifier) @property

; Function and method calls

(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: (property_identifier) @function.method))

; Function and method definitions

(function
  name: (identifier) @function)
(function_declaration
  name: (identifier) @function)
(method_definition
  name: (property_identifier) @function.method)

; ...
```
