// creational_patterns/builder_pattern.rs

#[derive(Debug, PartialEq)]
pub struct WeatherReport {
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
}

pub struct WeatherReportBuilder {
    temperature: f64,
    humidity: f64,
    wind_speed: f64,
}

impl WeatherReportBuilder {
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
            wind_speed: 0.0,
        }
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = value;
        self
    }

    pub fn humidity(mut self, value: f64) -> Self {
        self.humidity = value;
        self
    }

    pub fn wind_speed(mut self, value: f64) -> Self {
        self.wind_speed = value;
        self
    }

    pub fn build(self) -> WeatherReport {
        WeatherReport {
            temperature: self.temperature,
            humidity: self.humidity,
            wind_speed: self.wind_speed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_report_builder() {
        let report = WeatherReportBuilder::new()
            .temperature(25.0)
            .humidity(50.0)
            .wind_speed(10.0)
            .build();

        assert_eq!(report.temperature, 25.0);
        assert_eq!(report.humidity, 50.0);
        assert_eq!(report.wind_speed, 10.0);
    }
}

