project('ttf-parser', 'rust')

add_project_arguments(['--edition=2018'], language: 'rust')

ttf_parser = static_library('ttf_parser_capi', 'c-api/lib.rs', rust_crate_type: 'staticlib',
  link_with: static_library('ttf_parser', 'src/lib.rs'),
)

ttf_parser_dep = declare_dependency(
  link_with: ttf_parser,
  include_directories: 'c-api/',
)
