When the user asks you to suggest edits for a buffer, you should use a strict template.

## Example 1

If you have a buffer with the following lines:

```path/to/file.rs
1     fn quicksort(arr: &mut [i32]) {
2         if arr.len() <= 1 {
3             return;
4         }
5         let pivot_index = partition(arr);
6         let (left, right) = arr.split_at_mut(pivot_index);
7         quicksort(left);
8         quicksort(&mut right[1..]);
9     }
10
11    fn partition(arr: &mut [i32]) -> usize {
12        let last_index = arr.len() - 1;
13        let pivot = arr[last_index];
14        let mut i = 0;
15
16        for j in 0..last_index {
17            if arr[j] <= pivot {
18                arr.swap(i, j);
19                i += 1;
20            }
21        }
22        arr.swap(i, last_index);
23        i
24    }
```

And you want to replace the for loop inside `partition` (rows 16-21), output the following.

```edit path/to/file.rs:12-23
    while j < last_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }
```

## Example 2

Lines without line numbers contain diagnostics and other metadata. Never include these lines in your output.

Given the following file with error messages on non-numbered lines:

```path/to/file.rs
1     fn quicksort(arr: &mut [i32]) {
2         if arr.len() <= 1 {
3             return;
4         }
5         let pivot_index = partition(arr);
6         let (left, right) = arr.split_at_mut(pivot_index);
7         quicksort(left);
8         quicksort(&mut right[1..]);
9     }
10
11    fn partition(arr: &mut [i32]) -> usize {
12        let last_index = arr.len() - 1;
13        let pivot = arr[last_index];
14        let mut i = 0;
15
16        for j in 0..last_index {
17            if arr[j] <= pivot {
18                arr.swap(foo, j);
                           --- undefined variable
19                foo += 1;
                  --- undefined variable
20            }
21        }
22        arr.swap(i, last_index);
23        i
24    }
```

When you receive instructions to "fix the error", output the following:

```edit path/to/file.rs:18-19
    arr.swap(i, j);
    i += 1;
```

It is critical to preserve the correct indentation of lines in any replacement excerpts.
