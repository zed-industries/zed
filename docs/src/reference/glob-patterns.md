# Glob Patterns

Glob patterns are Unix shell-style wildcards for matching file paths, like `*.md` or `docs/**/*.rs`. Zed uses globs in search filters, file exclusions, and various settings.

## Syntax Reference

| Pattern | Matches | Example |
|---------|---------|---------|
| `?` | Any single character | `?.md` matches `a.md`, not `ab.md` |
| `*` | Any sequence of characters (except `/`) | `*.rs` matches `main.rs`, `lib.rs` |
| `**` | Any directory depth (including zero) | `src/**/*.rs` matches `src/main.rs`, `src/lib/utils.rs` |
| `[abc]` | Any one character in brackets | `[abc].txt` matches `a.txt`, `b.txt`, `c.txt` |
| `[a-z]` | Any character in range | `[0-9].log` matches `1.log`, `9.log` |
| `[!abc]` | Any character not in brackets | `[!0-9].txt` matches `a.txt`, not `1.txt` |

## Common Examples

```/dev/null/examples.txt#L1-12
# File extensions
*.rs                    # All Rust files
*.{rs,toml}             # NOT supported - use multiple patterns

# Directory matching
docs/**/*.md            # All Markdown files under docs/
**/test_*.py            # Test files in any directory

# Case-insensitive matching (globs are case-sensitive)
*.[cC]                  # Matches .c and .C files
```

## Where Globs Are Used

| Feature | Setting/Location | Notes |
|---------|------------------|-------|
| Project search | Include/Exclude filters | Filter search results by path |
| File excludes | `file_scan_exclusions` | Hide files from project panel |
| Search excludes | `search.exclude` | Exclude from search results |
| Formatter overrides | `languages.*.format_on_save` | Match files for formatting rules |

## Notes

- Globs in Zed are **case-sensitive**. On macOS (case-insensitive filesystem), `*.c` won't match `Main.C`.
- Brace expansion (`{a,b,c}`) is **not supported**. Use separate patterns instead.
- Patterns are matched against the full path from the project root.
- To match a literal `-` in brackets, place it first or last: `[-abc]` or `[abc-]`.
- To match a literal `[` or `]`, use `[[]` or `[]]`.

## See Also

- [Configuring Zed](../configuring-zed.md) for settings that accept glob patterns
- [gitignore patterns](https://git-scm.com/docs/gitignore#_pattern_format) use similar but not identical syntax
