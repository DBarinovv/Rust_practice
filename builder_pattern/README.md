**Create a Builder for a `Car` struct.**

You have a `Car` struct that can have different properties like brand, model, color, number of doors, etc. You want to make it easy for users of your code to create a car and set only the properties they care about, leaving the others with default values.

Here's a starting point for the `Car` struct:

```rust
struct Car {
    brand: String,
    model: String,
    color: String,
    doors: u8,
}
```

Now, your task is to create a `CarBuilder` struct that allows the construction of a `Car` object step by step. The builder should allow the setting of the brand, model, color, and doors, with methods to do so, and a final method to build the car. Any properties not set should fall back to default values.

For example, you might want to be able to create a car like this:

```rust
let car = CarBuilder::new()
    .brand("Toyota")
    .model("Camry")
    .color("Red")
    .doors(4)
    .build();
```

Or just:

```rust
let car = CarBuilder::new()
    .brand("Ford")
    .build();
```
