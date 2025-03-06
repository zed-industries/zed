alter table extension_versions
add column provides_themes bool not null default false,
add column provides_icon_themes bool not null default false,
add column provides_languages bool not null default false,
add column provides_grammars bool not null default false,
add column provides_language_servers bool not null default false,
add column provides_context_servers bool not null default false,
add column provides_slash_commands bool not null default false,
add column provides_indexed_docs_providers bool not null default false,
add column provides_snippets bool not null default false;
