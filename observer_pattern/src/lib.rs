trait Observer {
    fn update(&mut self, temperature: f32, humidity: f32);
}

struct WeatherStation {
    temperature: f32,
    humidity: f32,
    observers: Vec<Box<dyn Observer>>,
}

impl WeatherStation {
    pub fn new(temperature: f32, humidity: f32) -> Self {
        unimplemented!();
        // Implementation here
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        unimplemented!();
        // Implementation here
    }

    pub fn remove_observer(&mut self, observer: usize) {
        unimplemented!();
        // Implementation here
    }

    pub fn set_measurements(&mut self, temperature: f32, humidity: f32) {
        unimplemented!();
        // Implementation here
    }

    fn notify_observers(&self) {
        unimplemented!();
        // Implementation here
    }
}

struct TemperatureDisplay;

impl Observer for TemperatureDisplay {
    fn update(&mut self, temperature: f32, humidity: f32) {
        // Implementation here
    }
}

struct HumidityDisplay;

impl Observer for HumidityDisplay {
    fn update(&mut self, temperature: f32, humidity: f32) {
        // Implementation here
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut weather_station = WeatherStation::new(25.0, 60.0);
        let temp_display = Box::new(TemperatureDisplay);
        let humidity_display = Box::new(HumidityDisplay);
    
        weather_station.add_observer(temp_display);
        weather_station.add_observer(humidity_display);
    
        weather_station.set_measurements(30.0, 55.0);
        // Both TemperatureDisplay and HumidityDisplay should be updated
    }
}
