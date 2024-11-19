Your task is to grade similarity of code snippets.

You'll be given two snippets: actual and expected. You need to give the "actual" text a score between 0.0 and 1.0

0.0 means the pair is not similar at all.

1.0 means the pair is very similar.

## Example 1

Actual:

```
fn fibonacci(n: usize) -> usize{
}
```

Expected:

```
fn fibonacci(n: usize) -> usize{
}
```

Grade: 1.0

## Example 2

Actual:

```
fn fibonacci(n: usize) -> usize{
}
```

Expected:

```
hello
```

Grade: 0.0

## Example 3

Actual:

```
func quicksort(arr []int) []int {
	if len(arr) < 2 {
		return arr
	}

	left, right := 0, len(arr)-1

	pivotIndex := len(arr) / 2

	arr[pivotIndex], arr[right] = arr[right], arr[pivotIndex]

	for i := range arr {
		if arr[i] < arr[right] {
			arr[i], arr[left] = arr[left], arr[i]
			left++
		}
	}

	arr[left], arr[right] = arr[right], arr[left]

	quicksort(arr[:left])
	quicksort(arr[left+1:])

	return arr
}
```

Expected:

```
// quicksort sorts the given arr. It does not sort it if the array has a single
// element.
func quicksort(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	pivot := arr[len(arr)/2]
	left := []int{}
	right := []int{}

	for _, v := range arr {
		if v < pivot {
			left = append(left, v)
		} else if v > pivot {
			right = append(right, v)
		}
	}

	left = quicksortVariant(left)
	right = quicksortVariant(right)

	return append(append(left, pivot), right...)
}
```

Grade: 0.6

## Example 4

Actual:

```
function calculateArea(length, width) {
    return length * width;
}
```

Expected:

```
function getArea(l, w) {
    return l * w;
}
```

Grade: 0.6

## Example 5

Actual:

```
class Player {
    constructor(name, score) {
        this.name = name;
        this.score = score;
    }

    getScore() {
        return this.score;
    }
}
```

Expected:

```
class Player {
    constructor(name, score) {
        this.name = name;
        this.score = score;
    }

    getScore() {
        return this.score;
    }
}
```

Grade: 1.0

## Example 6

Actual:

```
class BinarySearchTree {
    constructor() {
        this.root = null;
    }

    insert(value) {
        const newNode = new Node(value);
        if (!this.root) {
            this.root = newNode;
            return;
        }
        let current = this.root;
        while(true) {
            if (value < current.value) {
                if (!current.left) {
                    current.left = newNode;
                    break;
                }
                current = current.left;
            } else {
                if (!current.right) {
                    current.right = newNode;
                    break;
                }
                current = current.right;
            }
        }
    }
}
```

Expected:

```
class BST {
    constructor() {
        this.root = null;
    }

    add(data) {
        const node = new Node(data);
        if (!this.root) {
            this.root = node;
            return;
        }
        let current = this.root;
        while(true) {
            if (data < current.data) {
                if (!current.left) {
                    current.left = node;
                    return;
                }
                current = current.left;
            } else {
                if (!current.right) {
                    current.right = node;
                    return;
                }
                current = current.right;
            }
        }
    }
}
```

Grade: 0.8

## Example 7

Actual:

```
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)
```

Expected:

```
def quick_sort(array):
    if len(array) < 2:
        return array
    pivot = array[len(array) // 2]
    less = [i for i in array if i < pivot]
    equal = [i for i in array if i == pivot]
    greater = [i for i in array if i > pivot]
    return quick_sort(less) + equal + quick_sort(greater)
```

Grade: 0.9

## Example 8

Actual:

```
async function fetchUserData(userId) {
    try {
        const response = await fetch(`/api/users/${userId}`);
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const userData = await response.json();
        return {
            success: true,
            data: userData,
            error: null
        };
    } catch (error) {
        console.error('Error fetching user data:', error);
        return {
            success: false,
            data: null,
            error: error.message
        };
    }
}
```

Expected:

```
async function getUser(id) {
    try {
        const res = await fetch(`/api/users/${id}`);
        if (!res.ok) {
            throw new Error('Failed to fetch user');
        }
        const user = await res.json();
        return {
            success: true,
            user,
            error: null
        };
    } catch (err) {
        console.log('Error:', err);
        return {
            success: false,
            user: null,
            error: err.message
        };
    }
}
```

Grade: 0.7

## Example 9

Actual:

```
function process_data(input) {
    var x = 0;
    for(var i = 0; i < input.length; i++) {
        x = x + input[i] * 2;
    }
    return x;
}
```

Expected:

```
function calculate_sum(arr) {
    let total = 0;
    arr.forEach(num => {
        total += num;
    });
    return total;
}
```

Grade: 0.3

## Example 10

Actual:

```
class Rectangle {
    setWidth(w) {
        this.width = w;
    }
    setHeight(h) {
        this.height = h;
    }
}
```

Expected:

```
class Circle {
    constructor(radius) {
        this.radius = radius;
    }

    getArea() {
        return Math.PI * this.radius * this.radius;
    }
}
```

Grade: 0.2
