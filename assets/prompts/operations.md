Your task is to map a step from the conversation above to operations on symbols inside the provided source files.

Guidelines:
- There's no need to describe *what* to do, just *where* to do it.
- If creating a file, assume any subsequent updates are included at the time of creation.
- Don't create and then update a file.
- We'll create it in one shot.
- Prefer updating symbols lower in the syntax tree if possible.
- Never include operations on a parent symbol and one of its children in the same <operations> block.
- Never nest an operation with another operation or include CDATA or other content. All operations are leaf nodes.
- Include a description attribute for each operation with a brief, one-line description of the change to perform.
- Descriptions are required for all operations except delete.
- When generating multiple operations, ensure the descriptions are specific to each individual operation.
- Avoid referring to the location in the description. Focus on the change to be made, not the location where it's made. That's implicit with the symbol you provide.
- Don't generate multiple operations at the same location. Instead, combine them together in a single operation with a succinct combined description.

The available operation types are:

1. <update>: Modify an existing symbol in a file.
2. <create_file>: Create a new file.
3. <insert_sibling_after>: Add a new symbol as sibling after an existing symbol in a file.
4. <append_child>: Add a new symbol as the last child of an existing symbol in a file.
5. <prepend_child>: Add a new symbol as the first child of an existing symbol in a file.
6. <delete>: Remove an existing symbol from a file. The `description` attribute is invalid for delete, but required for other ops.

All operations *require* a path.
Operations that *require* a symbol: <update>, <insert_sibling_after>, <delete>
Operations that don't allow a symbol: <create>
Operations that have an *optional* symbol: <prepend_child>, <append_child>

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

  Symbols for src/rectangle.rs:
  - struct Rectangle
  - impl Rectangle
  - impl Rectangle fn new

  <step>Add new methods 'calculate_area' and 'calculate_perimeter' to the Rectangle struct</step>
  <step>Implement the 'Display' trait for the Rectangle struct</step>

  What are the operations for the step: <step>Add a new method 'calculate_area' to the Rectangle struct</step>

Assistant (wrong):
<operations>
    <append_child path="src/shapes.rs" symbol="impl Rectangle" description="Add calculate_area method" />
    <append_child path="src/shapes.rs" symbol="impl Rectangle" description="Add calculate_perimeter method" />
</operations>

This demonstrates what NOT to do. NEVER append multiple children at the same location.

Assistant (corrected):
<operations>
    <append_child path="src/shapes.rs" symbol="impl Rectangle" description="Add calculate area and perimeter methods" />
</operations>

User:
What are the operations for the step: <step>Implement the 'Display' trait for the Rectangle struct</step>

Assistant:
<operations>
    <insert_sibling_after path="src/shapes.rs" symbol="impl Rectangle" description="Implement Display trait for Rectangle"/>
</operations>

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

Symbols for src/user.rs:
- struct User
- struct User pub name
- struct User age
- struct User email
- impl User
- impl User fn new
- impl User pub fn print_info

<step>Update the 'print_info' method to use formatted output</step>
<step>Remove the 'email' field from the User struct</step>

What are the operations for the step: <step>Update the 'print_info' method to use formatted output</step>

Assistant:
<operations>
    <update path="src/user.rs" symbol="impl User fn print_info" description="Use formatted output" />
</operations>

User:
What are the operations for the step: <step>Remove the 'email' field from the User struct</step>

Assistant:
<operations>
    <delete path="src/user.rs" symbol="struct User email" description="Remove the email field" />
</operations>

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

Symbols for src/vehicle.rs:
- struct Vehicle
- struct Vehicle make
- struct Vehicle model
- struct Vehicle year
- impl Vehicle
- impl Vehicle fn new
- impl Vehicle fn print_year

<step>Add a 'use std::fmt;' statement at the beginning of the file</step>
<step>Add a new method 'start_engine' in the Vehicle impl block</step>

What are the operations for the step: <step>Add a 'use std::fmt;' statement at the beginning of the file</step>

Assistant:
<operations>
    <prepend_child path="src/vehicle.rs" description="Add 'use std::fmt' statement" />
</operations>

User:
What are the operations for the step: <step>Add a new method 'start_engine' in the Vehicle impl block</step>

Assistant:
<operations>
    <insert_sibling_after path="src/vehicle.rs" symbol="impl Vehicle fn new" description="Add start_engine method"/>
</operations>

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

Symbols for src/employee.rs:
- struct Employee
- struct Employee name
- struct Employee position
- struct Employee salary
- struct Employee department
- impl Employee
- impl Employee fn new
- impl Employee fn print_details
- impl Employee fn give_raise

<step>Make salary an f32</step>

What are the operations for the step: <step>Make salary an f32</step>

A (wrong):
  <operations>
      <update path="src/employee.rs" symbol="struct Employee" description="Change the type of salary to an f32" />
      <update path="src/employee.rs" symbol="struct Employee salary" description="Change the type to an f32" />
  </operations>

This example demonstrates what not to do. `struct Employee salary` is a child of `struct Employee`.

A (corrected):
  <operations>
      <update path="src/employee.rs" symbol="struct Employee salary" description="Change the type to an f32" />
  </operations>

User:
  What are the correct operations for the step: <step>Remove the 'department' field and update the 'print_details' method</step>

A:
  <operations>
      <delete path="src/employee.rs" symbol="struct Employee department" />
      <update path="src/employee.rs" symbol="impl Employee fn print_details" description="Don't print the 'department' field" />
  </operations>

Now generate the operations for the following step.
Output only valid XML containing valid operations with their required attributes.
NEVER output code or any other text inside <operation> tags. If you do, you will replaced with another model.
Your response *MUST* begin with <operations> and end with </operations>:
