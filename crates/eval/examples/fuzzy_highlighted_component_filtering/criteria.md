1. This change modifies the `PreviewEntry` enum to include an `Option<Vec<usize>>` alongside the `ComponentMetadata`, allowing the system to track where fuzzy-matched characters occur in the component name for highlighting purposes.
2. The `fuzzy_match` function is added to `ComponentPreview`, using a simple character-sequential approach to find and return byte positions of matched characters within component names.
3. The function now filters and groups components not only by basic containment but also by fuzzy match results, storing highlighted positions when matches are found.
4. When rendering a `PreviewEntry::Component`, if highlight positions exist, it uses a `HighlightedLabel` instead of a plain `Label`, allowing for visual indication of the matched characters in the UI.
5. `HighlightedLabel` is added to the `ui` import to support the new highlighted rendering in the component preview list.
6. Even when the fuzzy match is performed on a concatenated string (name, scope, description), only the matched positions within the component name are extracted and passed for highlighting.
7. This includes updates across list rendering, sorting, and interaction handling logic to destructure the new `(ComponentMetadata, Option<Vec<usize>>)` format.
8. If fuzzy matching returns `None`, the code falls back to traditional substring checks, and manually constructs highlight positions for basic matches.
