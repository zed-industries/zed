
# Crate Features

| Cargo feature | Description |
| --- | --- |
| `proc_macro` |  Crate feature `proc_macro` (enables the `#[section]` attribute shim). |

# Macro Attributes

<table><tr><th>Attribute</th><th>Description</th></tr>
<tr><td><code>aux(main = path::to::MAIN_SECTION)</code></td><td>

 Auxiliary sections are stored in a section near the main section. The
 aux path must be a valid reference to the main section.


</td></tr>
<tr><td><code>crate_path = ::path::to::link_section</code></td><td>

 Specify a custom crate path for the `link-section` crate. Used when
 re-exporting the section macro.


</td></tr>
<tr><td><code>name = my_crate::SECTION_NAME</code></td><td>

 Specify a custom section name to allow the section to be used without a
 direct reference. If not specified, the section name will be generated
 using the item name and a path to the section.

 It is valid to specify multiple sections with the same name, and the linker
 will ensure that both sections contain the same items. The multiple sections
 must contain the same type, otherwise the section will `panic!` at runtime.

 While `name` accepts a path, this path does not refer to a specific Rust
 item path.


</td></tr>
<tr><td><code>untyped | typed | mutable | movable | reference</code></td><td>

 The type of the section.


</td></tr>
<tr><td><code>unsafe</code></td><td>

 Allow the section to be used without a direct reference.


</td></tr>
</table>

# Defaults
