pub struct Car {
    pub brand: String,
    pub model: String,
    pub color: String,
    pub doors: u8,
}

pub struct CarBuilder {
    brand: String,
    model: String,
    color: String,
    doors: u8,
}

impl CarBuilder {
    pub fn new() -> Self {
        Self {
            brand: String::default(),
            model: String::default(),
            color: String::default(),
            doors: 0,
        }
    }

    pub fn brand(mut self, value: &str) -> Self {
        self.brand = value.to_owned();
        self
    }

    pub fn model(mut self, value: &str) -> Self {
        self.model = value.to_owned();
        self
    }

    pub fn color(mut self, value: &str) -> Self {
        self.color = value.to_owned();
        self
    }

    pub fn doors(mut self, value: u8) -> Self {
        self.doors = value;
        self
    }

    pub fn build(self) -> Car {
        Car {
            brand: self.brand,
            model: self.model,
            color: self.color,
            doors: self.doors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_specification() {
        let car = CarBuilder::new()
            .brand("Toyota")
            .model("Camry")
            .color("Red")
            .doors(4)
            .build();

        assert_eq!(car.brand, "Toyota");
        assert_eq!(car.model, "Camry");
        assert_eq!(car.color, "Red");
        assert_eq!(car.doors, 4);
    }

    #[test]
    fn test_partial_specification() {
        let car = CarBuilder::new().brand("Ford").build();

        assert_eq!(car.brand, "Ford");
        assert_eq!(car.model, "");
        assert_eq!(car.color, "");
        assert_eq!(car.doors, 0);
    }

    #[test]
    fn test_default_specification() {
        let car = CarBuilder::new().build();

        assert_eq!(car.brand, "");
        assert_eq!(car.model, "");
        assert_eq!(car.color, "");
        assert_eq!(car.doors, 0);
    }
}
