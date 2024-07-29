Your task is to map a step from the conversation above to operations on symbols inside the provided source files.

Guidelines:
- There's no need to describe *what* to do, just *where* to do it.
- If creating a file, assume any subsequent updates are included at the time of creation.
- Don't create and then update a file.
- We'll create it in one shot.
- Prefer updating symbols lower in the syntax tree if possible.
- Never include operations on a parent symbol and one of its children in the same operations block.
- Never nest an operation with another operation or include CDATA or other content. All operations are leaf nodes.
- Include a description attribute for each operation with a brief, one-line description of the change to perform.
- Descriptions are required for all operations except delete.
- When generating multiple operations, ensure the descriptions are specific to each individual operation.
- Avoid referring to the location in the description. Focus on the change to be made, not the location where it's made. That's implicit with the symbol you provide.
- Don't generate multiple operations at the same location. Instead, combine them together in a single operation with a succinct combined description.

Example 1:

User:
```rs src/rectangle.rs
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}
```

<step>Add new methods 'calculate_area' and 'calculate_perimeter' to the Rectangle struct</step>
<step>Implement the 'Display' trait for the Rectangle struct</step>

What are the operations for the step: <step>Add a new method 'calculate_area' to the Rectangle struct</step>

Assistant (wrong):
{
  "operations": [
    {
      "kind": "AppendChild",
      "path": "src/shapes.rs",
      "symbol": "impl Rectangle",
      "description": "Add calculate_area method"
    },
    {
      "kind": "AppendChild",
      "path": "src/shapes.rs",
      "symbol": "impl Rectangle",
      "description": "Add calculate_perimeter method"
    }
  ]
}

This demonstrates what NOT to do. NEVER append multiple children at the same location.

Assistant (corrected):
{
  "operations": [
    {
      "kind": "AppendChild",
      "path": "src/shapes.rs",
      "symbol": "impl Rectangle",
      "description": "Add calculate area and perimeter methods"
    }
  ]
}

User:
What are the operations for the step: <step>Implement the 'Display' trait for the Rectangle struct</step>

Assistant:
{
  "operations": [
    {
      "kind": "InsertSiblingAfter",
      "path": "src/shapes.rs",
      "symbol": "impl Rectangle",
      "description": "Implement Display trait for Rectangle"
    }
  ]
}

Example 2:

User:
```rs src/user.rs
struct User {
    pub name: String,
    age: u32,
    email: String,
}

impl User {
    fn new(name: String, age: u32, email: String) -> Self {
        User { name, age, email }
    }

    pub fn print_info(&self) {
        println!("Name: {}, Age: {}, Email: {}", self.name, self.age, self.email);
    }
}
```

<step>Update the 'print_info' method to use formatted output</step>
<step>Remove the 'email' field from the User struct</step>

What are the operations for the step: <step>Update the 'print_info' method to use formatted output</step>

Assistant:
{
  "operations": [
    {
      "kind": "Update",
      "path": "src/user.rs",
      "symbol": "impl User pub fn print_info",
      "description": "Use formatted output"
    }
  ]
}

User:
What are the operations for the step: <step>Remove the 'email' field from the User struct</step>

Assistant:
{
  "operations": [
    {
      "kind": "Delete",
      "path": "src/user.rs",
      "symbol": "struct User email"
    }
  ]
}

Example 3:

User:
```rs src/vehicle.rs
struct Vehicle {
    make: String,
    model: String,
    year: u32,
}

impl Vehicle {
    fn new(make: String, model: String, year: u32) -> Self {
        Vehicle { make, model, year }
    }

    fn print_year(&self) {
        println!("Year: {}", self.year);
    }
}
```

<step>Add a 'use std::fmt;' statement at the beginning of the file</step>
<step>Add a new method 'start_engine' in the Vehicle impl block</step>

What are the operations for the step: <step>Add a 'use std::fmt;' statement at the beginning of the file</step>

Assistant:
{
  "operations": [
    {
      "kind": "PrependChild",
      "path": "src/vehicle.rs",
      "description": "Add 'use std::fmt' statement"
    }
  ]
}

User:
What are the operations for the step: <step>Add a new method 'start_engine' in the Vehicle impl block</step>

Assistant:
{
  "operations": [
    {
      "kind": "InsertSiblingAfter",
      "path": "src/vehicle.rs",
      "symbol": "impl Vehicle fn new",
      "description": "Add start_engine method"
    }
  ]
}

Example 4:

User:
```rs src/employee.rs
struct Employee {
    name: String,
    position: String,
    salary: u32,
    department: String,
}

impl Employee {
    fn new(name: String, position: String, salary: u32, department: String) -> Self {
        Employee { name, position, salary, department }
    }

    fn print_details(&self) {
        println!("Name: {}, Position: {}, Salary: {}, Department: {}",
                  self.name, self.position, self.salary, self.department);
    }

    fn give_raise(&mut self, amount: u32) {
        self.salary += amount;
    }
}
```

<step>Make salary an f32</step>

What are the operations for the step: <step>Make salary an f32</step>

Assistant (wrong):
{
  "operations": [
    {
      "kind": "Update",
      "path": "src/employee.rs",
      "symbol": "struct Employee",
      "description": "Change the type of salary to an f32"
    },
    {
      "kind": "Update",
      "path": "src/employee.rs",
      "symbol": "struct Employee salary",
      "description": "Change the type to an f32"
    }
  ]
}

This example demonstrates what not to do. `struct Employee salary` is a child of `struct Employee`.

Assistant (corrected):
{
  "operations": [
    {
      "kind": "Update",
      "path": "src/employee.rs",
      "symbol": "struct Employee salary",
      "description": "Change the type to an f32"
    }
  ]
}

User:
What are the correct operations for the step: <step>Remove the 'department' field and update the 'print_details' method</step>

Assistant:
{
  "operations": [
    {
      "kind": "Delete",
      "path": "src/employee.rs",
      "symbol": "struct Employee department"
    },
    {
      "kind": "Update",
      "path": "src/employee.rs",
      "symbol": "impl Employee fn print_details",
      "description": "Don't print the 'department' field"
    }
  ]
}

Now generate the operations for the following step.
