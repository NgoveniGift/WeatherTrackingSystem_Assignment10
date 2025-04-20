#  CHANGELOG.md – Assignment 10

All commits and pattern additions for Assignment 10: *From Class Diagrams to Code*.

---

##  Implemented Core Classes (Class Diagram)
- **Added** `UserAccount`, `Location`, `Sensor`, `WeatherStation`, `WeatherReport`, `ForecastRequest`, and `Alert` structs.
- **Methods Implemented:** registration, login, report generation, request handling, and more.

 Related: #1, #2, #3

---

##  Creational Design Patterns

### 1.  Simple Factory
- **Created** `SensorFactory` to instantiate `Sensor` types.
- **Tested** temperature and humidity sensor creation.

 Related: #4

### 2.  Factory Method
- **Implemented** `ReportGenerator` trait with `TemperatureReport` and `WindSpeedReport`.
- **Abstracted** generator logic by report type.

 Related: #5

### 3.  Abstract Factory
- **Built** `UIComponentFactory` with `MobileUIFactory` and `WebUIFactory`.
- **Components:** buttons and display panels per platform.

 Related: #6

### 4.  Builder
- **Constructed** `WeatherReportBuilder` for fluent weather report creation.
- **Fields:** temperature, humidity, wind speed.

 Related: #7

### 5.  Prototype
- **Cloned** `ReportTemplate` using Rust’s `Clone` trait.
- **Managed** report template instances with `ReportCache`.

 Related: #8

### 6.  Singleton
- **Singleton config** using `once_cell::sync::Lazy`.
- **Ensures** one global instance of `Config` across system.

 Related: #9

---

##  Unit Testing
- **Added** unit tests for all 6 patterns in `tests_creational_patterns.rs`
- **Coverage:** Constructor behavior, method logic, singleton consistency

 Related: #10, #11

---

 **Last Updated:** Assignment 10 submission – April 2025

