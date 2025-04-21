We're trying to improve both performance and usability when working with large diffs in the editor. A few areas need attention:

First, the current diff animation applies updates line-by-line, which can feel slow and visually jarring for large edits. Could you revise the logic so that we update the editor in larger chunks instead? For smaller diffs, direct scrolling to the edited line is fine, but for larger changes, it would be great to implement a smooth scrolling animation that steps through the affected region before settling at the final line.

Second, the current error message when a SEARCH block doesn't match is a bit too vague. Let's make it clearer that the issue could be due to out-of-order or imprecise SEARCH/REPLACE blocks, especially when working with multiple blocks. It might also help to add a suggestion that users try only 1–3 changes at a time for large files before retrying.

Finally, in the file accordion UI, it would be useful to show how many edits a file contains. Could you parse the diff content and display a count of REPLACE blocks next to the file path, maybe with a small icon for clarity?
