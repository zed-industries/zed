Searches the entire project for the given regular expression.

Returns a list of paths that matched the query. For each path, it returns some excerpts of the matched text.

Results are paginated with 20 matches per page. Use the optional 'offset' parameter to request subsequent pages.

This tool is not aware of semantics and does not use any information from language servers, so it should only be used when no available semantic tool (e.g. one that uses language servers) could fit a particular use case instead.
