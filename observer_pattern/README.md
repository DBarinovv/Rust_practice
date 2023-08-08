**OBSERVER PATTERN**

- **Subject**: Maintains a list of observers and notifies them of state changes.
- **Observer**: Defines an updating interface for objects that need to be notified of changes in the subject.
- **ConcreteObserver**: Implements the observer updating interface to keep its state consistent with the subject's.
- **ConcreteSubject**: Stores the state of interest to ConcreteObserver objects and sends notifications to them when its state changes.

**Task**: Implement the Observer pattern in Rust. Create a `WeatherStation` that tracks temperature and humidity. It will be your ConcreteSubject. Then, create two ConcreteObservers: `TemperatureDisplay` and `HumidityDisplay`. Both should implement an Observer trait and update their display whenever the weather station changes its measurements.

Here's the skeleton to get you started:

```rust
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
        // Implementation here
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        // Implementation here
    }

    pub fn remove_observer(&mut self, observer: usize) {
        // Implementation here
    }

    pub fn set_measurements(&mut self, temperature: f32, humidity: f32) {
        // Implementation here
    }

    fn notify_observers(&self) {
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

fn main() {
    let mut weather_station = WeatherStation::new(25.0, 60.0);
    let temp_display = Box::new(TemperatureDisplay);
    let humidity_display = Box::new(HumidityDisplay);

    weather_station.add_observer(temp_display);
    weather_station.add_observer(humidity_display);

    weather_station.set_measurements(30.0, 55.0);
    // Both TemperatureDisplay and HumidityDisplay should be updated
}
```
