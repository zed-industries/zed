---
title: Localization
description: How Zed's interface is translated, how to mark strings for localization, and how to add a language.
---

# Localization

Zed's interface can be displayed in languages other than English. Users select a
language with the `ui_language` setting, which takes a
[BCP 47](https://www.rfc-editor.org/info/bcp47) language tag:

```json
{
  "ui_language": "zh-CN"
}
```

Interface text that has no translation for the selected language is shown in
English, so an incomplete translation degrades gracefully rather than leaving
blank labels.

## How it works

Translations live in [Fluent](https://projectfluent.org) catalogs under
`assets/i18n/<locale>/*.ftl`, and are looked up by the [`i18n`](https://github.com/zed-industries/zed/tree/main/crates/i18n)
crate.

A user-facing string is marked for localization by wrapping its English literal
in `t!`:

```rust
use i18n::t;

MenuItem::action(t!("Zoom In"), zed_actions::IncreaseBufferFontSize::default())
```

`t!` does two things:

1. Derives the catalog key from the English text by lower-kebab-casing it, so
   `t!("Zoom In")` looks up `zoom-in`. The key is computed once per call site.
2. Returns a `LocalizedString`, which resolves against the active locale **when
   it is read**, not when it is constructed.

Because `LocalizedString` converts into `SharedString`, any call site that
already accepts `impl Into<SharedString>` takes `t!(..)` without a signature
change. This is what lets crates be migrated one at a time.

### Resolving late

Resolution happens on read so that changing `ui_language` takes effect on the
next render, without rebuilding view trees by hand. A view that stores a
`LocalizedString` and resolves it in `render` picks up the new locale
automatically.

The one exception is the application menu bar: it is owned by the platform rather
than rendered by GPUI, so it is rebuilt explicitly when the locale changes.

### Placeables

Values interpolated into a string are passed to `t!` as named arguments rather
than formatted into the literal beforehand, so that catalogs can position them
freely and select plural forms based on them:

```rust
Label::new(t!("Renaming {$count} files", count = files.len()))
```

The corresponding catalog entry may then use Fluent's selectors:

```ftl
renaming-count-files = 正在重命名 {$count} 个文件
```

Pass numbers and dates as numbers and dates, not as pre-rendered strings — a
formatted string cannot be re-formatted for another locale.

## Marking a string

1. Wrap the literal in `t!` and add `use i18n::t;` to the module.
2. Run `script/i18n-coverage` to see the derived key.
3. Add the key to each catalog you can translate.

Only mark text the user reads. Leave product names (Zed, Git), file names
(`tasks.json`), URLs, action identifiers, and log messages in English.

## Adding a language

1. Create `assets/i18n/<locale>/zed.ftl`, where `<locale>` is a BCP 47 tag.
2. Define `locale-display-name` as the name the language gives itself, written in
   that language. The interface language picker lists a catalog by that name, and
   falls back to the tag for a catalog that does not define it. Leave it
   untranslated: a language list reads as 简体中文 and English whichever locale is
   active.
3. Translate the keys reported by `script/i18n-coverage`.
4. Run the `locale selector: toggle` action to switch to it, or set
   `"ui_language": "<locale>"` directly.

Region-specific catalogs fall back to their base language, so `zh-CN` may define
only what differs from a `zh` catalog.

## Checking coverage

`script/i18n-coverage` compares the strings marked with `t!` against the bundled
catalogs and reports, per locale:

- **missing** — marked for localization but absent from the catalog
- **orphaned** — defined in the catalog but no longer marked in the sources
- **collision** — distinct English strings that derive the same key

A key the runtime reads directly instead of through a `t!` call site, such as
`locale-display-name`, has no English literal to derive from. Those are listed in
the script's `RESERVED_KEYS` and counted as neither missing nor orphaned.

```sh
script/i18n-coverage                    # report every locale
script/i18n-coverage --locale zh-CN     # one locale
script/i18n-coverage --require-complete # fail if anything is missing
```

Collisions always fail. They mean two different English strings would share one
translation, which is a problem when the strings translate differently even
though they look alike in English — reword one, or give it a dedicated key.
