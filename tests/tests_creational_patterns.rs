// tests/tests_creational_patterns.rs

#[cfg(test)]
mod tests {
    use crate::builder_pattern::*;
    use crate::factory_method::*;
    use crate::simple_factory::*;
    use crate::abstract_factory::*;
    use crate::prototype_pattern::*;
    use crate::singleton_pattern::*;

    #[test]
    fn test_simple_factory_sensor() {
        let sensor = SensorFactory::create_sensor(SensorType::Temperature);
        assert_eq!(sensor.sensor_type, "Temperature");
    }

    #[test]
    fn test_factory_method_temperature() {
        let generator = create_report_generator("temperature");
        assert_eq!(generator.generate(), "Temperature Report: 22°C");
    }

    #[test]
    fn test_abstract_factory_mobile_ui() {
        let mobile_factory = MobileUIFactory;
        assert_eq!(mobile_factory.create_button(), "Mobile Button");
        assert_eq!(mobile_factory.create_display_panel(), "Mobile Display Panel");
    }

    #[test]
    fn test_builder_pattern_report() {
        let report = WeatherReportBuilder::new()
            .temperature(28.0)
            .humidity(70.0)
            .wind_speed(12.0)
            .build();

        assert_eq!(report.temperature, 28.0);
        assert_eq!(report.humidity, 70.0);
        assert_eq!(report.wind_speed, 12.0);
    }

    #[test]
    fn test_prototype_pattern_clone() {
        let original = ReportTemplate {
            title: "Daily Summary".into(),
            content: "Data collected from sensors.".into(),
        };
        let cache = ReportTemplateCache::new(original.clone());
        let clone = cache.clone_template();
        assert_eq!(original, clone);
    }

    #[test]
    fn test_singleton_shared_instance() {
        let conn1 = get_db_instance();
        {
            let mut db = conn1.lock().unwrap();
            db.url = "cloudhost:1234".into();
        }
        let conn2 = get_db_instance();
        let db = conn2.lock().unwrap();
        assert_eq!(db.url, "cloudhost:1234");
    }
}
