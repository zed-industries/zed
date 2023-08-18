Given a snippet as the input, you must produce an array of edits. An edit has the following structure:

{ skip: "skip", delete: "delete", insert: "insert" }

`skip` is a string in the input that should be left unchanged. `delete` is a string in the input located right after the skipped text that should be deleted. `insert` is a new string that should be inserted after the end of the text in `skip`. It's crucial that a string in the input can only be skipped or deleted once and only once.

Your task is to produce an array of edits. `delete` and `insert` can be empty if nothing changed. When `skip`, `delete` or `insert` are longer than 20 characters, split them into multiple edits.

Check your reasoning by concatenating all the strings in `skip` and `delete`. If the text is the same as the input snippet then the edits are valid.

It's crucial that you reply only with edits. No prose or remarks.
