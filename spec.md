# Project Search Presets - Feature Specification

## Overview

This feature adds named presets for project search filters (include/exclude paths). Users can define presets in their project settings and quickly apply them via the command palette.

## Motivation

When working on large codebases, developers often need to search within specific subsets of files repeatedly. For example:
- Searching only in source code (excluding tests, generated files, vendor code)
- Searching only in test files
- Searching within a specific module or feature area

Currently, users must manually type include/exclude patterns each time. Presets allow saving these patterns for quick reuse.

## Settings Schema

Presets are stored in `.zed/settings.json` (project settings).

```json
{
  "project_search_presets": {
    "source_only": {
      "include": "src/**",
      "exclude": "**/*.test.ts, **/*.spec.ts"
    },
    "tests_only": {
      "include": "**/*.test.ts, **/*.spec.ts, tests/**"
    },
    "frontend": {
      "include": "src/ui/**, src/components/**",
      "exclude": "**/*.test.tsx"
    },
    "backend": {
      "include": "src/server/**, src/api/**"
    }
  }
}
```

### Design Notes

- Preset names are user-defined strings (keys in the object)
- Both `include` and `exclude` are optional - omitted fields leave that filter empty
- Pattern format matches existing project search behavior (comma-separated globs)
- The schema uses an object structure to allow future extension with additional fields (e.g., `regex`, `case_sensitive`, `whole_word`)

## User Experience

### Applying a Preset

1. User opens command palette (`Cmd+Shift+P`)
2. User types "search preset" to filter commands
3. User selects "Project Search: Search With Preset"
4. A picker appears listing available preset names
5. User selects a preset
6. A new project search tab opens with the preset's include/exclude values pre-filled
7. The filters panel auto-expands if the preset has include/exclude values
8. User types their search query and executes the search

### Command Palette Actions

| Command | Description |
|---------|-------------|
| `Project Search: Search With Preset` | Opens preset picker, then opens new search with selected preset applied |

### Preset Picker UI

The picker displays preset names only:
```
source_only
tests_only
frontend
backend
```

### Edge Cases

- **No presets defined**: Show informative message "No search presets defined. Add presets to .zed/settings.json"
- **Empty preset**: If a preset has neither include nor exclude, it opens a blank search (same as normal new search)
- **Invalid patterns**: Invalid glob patterns in presets should be handled gracefully - show the preset but surface parsing errors when applied (consistent with current search behavior)

## Future Extensions

The design accommodates future enhancements:

1. **Additional search options**: Extend presets with `regex`, `case_sensitive`, `whole_word` fields
2. **Query templates**: Presets could optionally include a default query pattern
3. **Keyboard shortcuts per preset**: Allow binding specific presets to shortcuts
4. **Save current as preset**: UI action to save current search configuration as a new preset
