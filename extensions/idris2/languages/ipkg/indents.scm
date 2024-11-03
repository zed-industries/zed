; Indentation rules for Idris .ipkg files
[
  (package_declaration)
  (field_declaration)
  (main_declaration)
  (executable_declaration)
  (version_declaration)
  (langversion_declaration)
] @indent.begin

(dependency_declaration
  "=" @indent.begin
  (dependency_list
    "," @indent.begin))

(module_declaration
  "=" @indent.begin
  (module_list
    "," @indent.begin))

(field_declaration
  "=" @indent.begin)

"=" @indent.branch

(string_value) @indent.begin

(comment) @indent.ignore

(ERROR) @indent.auto
