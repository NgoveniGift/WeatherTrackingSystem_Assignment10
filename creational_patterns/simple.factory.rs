// creational_patterns/simple_factory.rs

pub enum SensorType {
    Temperature,
    Humidity,
    WindSpeed,
}

pub struct Sensor {
    pub sensor_id: String,
    pub sensor_type: String,
    pub status: String,
    pub last_reading: f64,
    pub station_id: String,
}

impl Sensor {
    pub fn read_data(&self) -> f64 {
        println!("[{}] Reading: {}", self.sensor_type, self.last_reading);
        self.last_reading
    }
}

pub struct SensorFactory;

impl SensorFactory {
    pub fn create_sensor(sensor_type: SensorType, station_id: &str) -> Sensor {
        let (sensor_type_str, default_reading) = match sensor_type {
            SensorType::Temperature => ("Temperature", 22.5),
            SensorType::Humidity => ("Humidity", 45.0),
            SensorType::WindSpeed => ("WindSpeed", 12.3),
        };

        Sensor {
            sensor_id: format!("{}-{}", sensor_type_str, uuid::Uuid::new_v4()),
            sensor_type: sensor_type_str.to_string(),
            status: "Active".to_string(),
            last_reading: default_reading,
            station_id: station_id.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_sensor_creation() {
        let sensor = SensorFactory::create_sensor(SensorType::Temperature, "ST001");
        assert_eq!(sensor.sensor_type, "Temperature");
        assert_eq!(sensor.status, "Active");
    }

    #[test]
    fn test_humidity_sensor_creation() {
        let sensor = SensorFactory::create_sensor(SensorType::Humidity, "ST002");
        assert_eq!(sensor.sensor_type, "Humidity");
    }
}

