struct Car {
    brand: String,
    model: String,
    color: String,
    doors: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let car = CarBuilder::new()
            .brand("Toyota")
            .model("Camry")
            .color("Red")
            .doors(4)
            .build();

        let car = CarBuilder::new()
            .brand("Ford")
            .build();
    }
}
