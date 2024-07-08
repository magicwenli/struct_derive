# StructUpdate Derive Macro

`StructUpdate` is a derive macro in Rust that automatically generates an `update_struct` method for your struct. This method transforms all fields of type `String` in your struct to SCREAMING_SNAKE_CASE (all uppercase with underscores between words).

## Usage

1. First, you need to add the `struct_update` dependency in your `Cargo.toml` file.

```toml
[dependencies]
struct_update = "0.1.0"
```

2. Then, you can use the `#[derive(StructUpdate)]` annotation on your struct. Also, you need to use the `#[update_struct(with(ty = String, func = "to_screaming_snake_case"))]` annotation to specify which type of fields need to be updated and what function to use to update these fields.

```rust
use struct_update::StructUpdate;

fn to_screaming_snake_case(input: String) -> String {
    input.to_uppercase().replace(" ", "_")
}

#[derive(StructUpdate, Debug, Clone)]
#[update_struct(with(ty = String, func = "to_screaming_snake_case"))]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}
```

3. Now, your struct has an `update_struct` method. You can call this method to update the fields in the struct.

```rust
fn main() {
    let mut user = User {
        username: "johndoe".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    println!("{:#?}", user);

    user.update_struct();

    println!("{:#?}", user);
}
```

In the example above, the `update_struct` method transforms the `username`, `first_name`, and `last_name` fields to SCREAMING_SNAKE_CASE, but does not change the `age` field, as the `age` field is not of type `String`.