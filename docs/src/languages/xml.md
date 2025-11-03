# XML

XML support is available through the [XML extension](https://github.com/sweetppro/zed-xml/).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-xml](https://github.com/tree-sitter-grammars/tree-sitter-xml)

## Configuration

If you have additional file extensions that are not being automatically recognized as XML just add them to [file_types](../configuring-zed.md#file-types) in your Zed settings:

```json [settings]
  "file_types": {
    "XML": ["rdf", "gpx", "kml"]
  }
```
