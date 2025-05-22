Gives detailed information about code symbols in your project such as variables, functions, classes, interface, traits, and other programming constructs, using the editor's integrated Language Server Protocol (LSP) servers.

This tool is the preferred way to do things like:
* Find out where a code symbol is first declared (or first defined - that is, assigned)
* Find all the places where a code symbol is referenced
* Find the type definition for a code symbol
* Find a code symbol's implementation

This tool gives more reliable answers than things like regex searches, because it can account for relevant semantics like aliases. It should be used over textual search tools (e.g. regex) when searching for information about code symbols that this tool supports directly.

This tool should not be used when you need to search for something that is not a code symbol.
