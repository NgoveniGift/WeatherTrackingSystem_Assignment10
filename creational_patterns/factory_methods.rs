// creational_patterns/factory_method.rs

pub trait ReportGenerator {
    fn generate(&self) -> String;
}

pub struct TemperatureReport;
impl ReportGenerator for TemperatureReport {
    fn generate(&self) -> String {
        "Temperature Report: 22°C".to_string()
    }
}

pub struct WindSpeedReport;
impl ReportGenerator for WindSpeedReport {
    fn generate(&self) -> String {
        "Wind Speed Report: 14 km/h".to_string()
    }
}

pub fn create_report_generator(report_type: &str) -> Box<dyn ReportGenerator> {
    match report_type.to_lowercase().as_str() {
        "temperature" => Box::new(TemperatureReport),
        "windspeed" => Box::new(WindSpeedReport),
        _ => panic!("Unsupported report type: {}", report_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_report() {
        let generator = create_report_generator("temperature");
        assert_eq!(generator.generate(), "Temperature Report: 22°C");
    }

    #[test]
    fn test_windspeed_report() {
        let generator = create_report_generator("windspeed");
        assert_eq!(generator.generate(), "Wind Speed Report: 14 km/h");
    }

    #[test]
    #[should_panic(expected = "Unsupported report type")]
    fn test_invalid_report_type() {
        create_report_generator("rainfall");
    }
}

