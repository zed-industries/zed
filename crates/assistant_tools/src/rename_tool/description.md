Renames a symbol across your codebase using the language server's semantic knowledge.

This tool performs a rename refactoring operation on a specified symbol. It uses the project's language server to analyze the code and perform the rename correctly across all files where the symbol is referenced.

Unlike a simple find and replace, this tool understands the semantic meaning of the code, so it only renames the specific symbol you specify and not unrelated text that happens to have the same name.

Examples of symbols you can rename:
- Variables
- Functions
- Classes/structs
- Fields/properties
- Methods
- Interfaces/traits

The language server handles updating all references to the renamed symbol throughout the codebase.
