(comment) @comment

[
  (addition)
  (new_file)
] @string

[
  (deletion)
  (old_file)
] @keyword

(commit) @constant

(location) @attribute

(command
  "diff" @function
  (argument) @variable.parameter)

(mode) @number

([
  ".."
  "+"
  "++"
  "+++"
  "++++"
  "-"
  "--"
  "---"
  "----"
] @punctuation.special)

[
  (binary_change)
  (similarity)
  (file_change)
] @label

(index
  "index" @keyword)

(similarity
  (score) @number
  "%" @number)
