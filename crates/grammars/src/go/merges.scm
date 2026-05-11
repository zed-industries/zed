; Auto-Resolve structural merge rules for Go.

(source_file) @merge.set

(interface_type) @merge.set

(field_declaration_list) @merge.set

(function_declaration name: (identifier) @merge.key)

(method_declaration name: (field_identifier) @merge.key)

(type_spec name: (type_identifier) @merge.key)

(const_spec name: (identifier) @merge.key)

(var_spec name: (identifier) @merge.key)

(field_declaration name: (field_identifier) @merge.key)
