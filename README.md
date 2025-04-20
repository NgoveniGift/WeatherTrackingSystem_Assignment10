# WeatherTrackingSystem_Assignment10

# 🧩 Assignment 10: From Class Diagrams to Code – Weather Tracking System

This assignment implements the class structure and six creational design patterns for the Weather Tracking System, based on the UML diagram and domain model created in Assignment 9.

---

## 📚 Contents

### ✅ Class Implementation
- Core entities like `UserAccount`, `Location`, `Sensor`, `WeatherStation`, etc.
- Placed in `src/class_implementation.rs`
- Methods follow business logic from the domain model

### 🔧 Creational Design Patterns
All six creational patterns are implemented in `/creational_patterns/`:

| Pattern           | File                      | Use Case                                                  |
|------------------|---------------------------|------------------------------------------------------------|
| Simple Factory    | `simple_factory.rs`       | `SensorFactory` for Temperature, Humidity, WindSpeed      |
| Factory Method    | `factory_method.rs`       | `ReportGenerator` trait with temperature/wind subtypes    |
| Abstract Factory  | `abstract_factory.rs`     | `WeatherUIFactory` for mobile/web UI components           |
| Builder           | `builder_pattern.rs`      | `WeatherReportBuilder` for report generation              |
| Prototype         | `prototype_pattern.rs`    | `ReportTemplateCache` for cloning reports                 |
| Singleton         | `singleton_pattern.rs`    | `Config` for a shared global DB/config instance           |

### 🧪 Unit Testing (20 Marks)
- Located in `tests/tests_creational_patterns.rs`
- Each design pattern is covered with:
  - Validation of core behaviors and expected outputs
  - Pattern-specific logic (e.g., Singleton instance sync, Builder chaining)
  - Cloning verification and report data construction
- Tests are written using Rust's `#[test]` macro

🧪 Example:
```rust
#[test]
fn test_simple_factory_sensor_creation() {
    let sensor = SensorFactory::create_sensor(SensorType::Temperature);
    assert_eq!(sensor.sensor_type, "Temperature");
}
```

### 🧾 CHANGELOG
- `CHANGELOG.md` tracks commit-style pattern additions and class development

### 📘 Pattern Summary
- See `pattern_implementation_overview.md` for full breakdown

---

## 🧠 Learning Outcomes
- Applied six design patterns in a real-world system
- Practiced clean code structure, encapsulation, and testability
- Gained experience aligning UML models with maintainable codebases

---

## 📁 File Structure
```
/assignment10/
├── src/
│   └── class_implementation.rs
├── creational_patterns/
│   ├── simple_factory.rs
│   ├── factory_method.rs
│   ├── abstract_factory.rs
│   ├── builder_pattern.rs
│   ├── prototype_pattern.rs
│   ├── singleton_pattern.rs
│   └── tests_creational_patterns.rs
├── pattern_implementation_overview.md
├── CHANGELOG.md
└── README_assignment10.md
```
