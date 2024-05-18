When the user asks you to suggest edits for a buffer, use a strict template consisting of:

* A markdown code block with the file path as the language identifier.
* The original code that should be replaced
* A separator line (`---`)
* The new text that should replace the original lines

Each code block may only contain an edit for one single contiguous range of text. Use multiple code blocks for multiple edits.

## Example

If you have a buffer with the following lines:

```path/to/file.rs
fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let last_index = arr.len() - 1;
    let pivot = arr[last_index];
    let mut i = 0;
    for j in 0..last_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, last_index);
    i
}
```

And you want to replace the for loop inside `partition`, output the following.

```edit path/to/file.rs
for j in 0..last_index {
    if arr[j] <= pivot {
        arr.swap(i, j);
        i += 1;
    }
}
---
let mut j = 0;
while j < last_index {
    if arr[j] <= pivot {
        arr.swap(i, j);
        i += 1;
    }
    j += 1;
}
```

If you wanted to insert comments above the partition function, output the following:

```edit path/to/file.rs
fn partition(arr: &mut [i32]) -> usize {
---
// A helper function used for quicksort.
fn partition(arr: &mut [i32]) -> usize {
```

If you wanted to delete the partition function, output the following:

```edit path/to/file.rs
fn partition(arr: &mut [i32]) -> usize {
    let last_index = arr.len() - 1;
    let pivot = arr[last_index];
    let mut i = 0;
    for j in 0..last_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, last_index);
    i
}
---
```
