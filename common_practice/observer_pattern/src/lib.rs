use std::cell::RefCell;
use std::rc::Rc;

pub trait Observer {
    fn update(&mut self, temperature: f32, humidity: f32);
}

pub struct WeatherStation {
    temperature: f32,
    humidity: f32,
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl WeatherStation {
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, observer: usize) {
        if observer >= self.observers.len() {
            panic!("Wrong observer id");
        }

        self.observers.remove(observer);
    }

    pub fn set_measurements(&mut self, temperature: f32, humidity: f32) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.notify_observers();
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
        self.notify_observers();
    }

    pub fn set_humidity(&mut self, humidity: f32) {
        self.humidity = humidity;
        self.notify_observers();
    }

    fn notify_observers(&mut self) {
        self.observers
            .iter_mut()
            .for_each(|obs| obs.borrow_mut().update(self.temperature, self.humidity))
    }
}

pub struct TemperatureDisplay {
    temperature: f32,
}

impl TemperatureDisplay {
    pub fn display(&self) {
        println!("Temperature is: {:?}", self.temperature);
    }
}

impl Observer for TemperatureDisplay {
    fn update(&mut self, temperature: f32, _humidity: f32) {
        self.temperature = temperature;
    }
}

pub struct HumidityDisplay {
    humidity: f32,
}

impl Observer for HumidityDisplay {
    fn update(&mut self, _temperature: f32, humidity: f32) {
        self.humidity = humidity;
    }
}

impl HumidityDisplay {
    pub fn display(&self) {
        println!("Humidity is: {:?}", self.humidity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_display_update() {
        let mut weather_station = WeatherStation::new();
        let temp_display = Rc::new(RefCell::new(TemperatureDisplay { temperature: 0.0 }));
        let temp_display_ref = Rc::clone(&temp_display);

        weather_station.add_observer(temp_display_ref);
        weather_station.set_temperature(22.5);
        weather_station.notify_observers();

        assert_eq!(temp_display.borrow().temperature, 22.5);
    }

    #[test]
    fn test_add_and_remove_observer() {
        let mut weather_station = WeatherStation::new();
        let temp_display = Rc::new(RefCell::new(TemperatureDisplay { temperature: 0.0 }));

        weather_station.add_observer(temp_display);
        assert_eq!(weather_station.observers.len(), 1);

        weather_station.remove_observer(0);
        assert_eq!(weather_station.observers.len(), 0);
    }

    #[test]
    fn test_notify_observers() {
        let mut weather_station = WeatherStation::new();
        let temp_display = Rc::new(RefCell::new(TemperatureDisplay { temperature: 0.0 }));
        let humidity_display = Rc::new(RefCell::new(HumidityDisplay { humidity: 0.0 }));

        let temp_display_ref = Rc::clone(&temp_display);
        let humidity_display_ref = Rc::clone(&humidity_display);
        weather_station.add_observer(temp_display_ref);
        weather_station.add_observer(humidity_display_ref);
        weather_station.set_measurements(18.3, 55.0);

        assert_eq!(temp_display.borrow().temperature, 18.3);
        assert_eq!(humidity_display.borrow().humidity, 55.0);
    }
}
