// Assignment 10: Class Implementation Based on UML Diagram

// UserAccount Entity
pub struct UserAccount {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl UserAccount {
    pub fn register(&self) {
        println!("Registering user: {}", self.name);
    }

    pub fn login(&self) {
        println!("{} logged in", self.email);
    }

    pub fn update_profile(&mut self, name: String) {
        self.name = name;
    }

    pub fn delete_account(self) {
        println!("Account {} deleted", self.user_id);
    }
}

// Location Entity
pub struct Location {
    pub location_id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: String,
}

impl Location {
    pub fn save(&self) {
        println!("Location {} saved.", self.name);
    }

    pub fn update(&mut self, name: String) {
        self.name = name;
    }

    pub fn delete(self) {
        println!("Location {} deleted.", self.name);
    }
}

// Sensor Entity
pub struct Sensor {
    pub sensor_id: String,
    pub sensor_type: String,
    pub status: String,
    pub last_reading: f64,
    pub station_id: String,
}

impl Sensor {
    pub fn read_data(&self) -> f64 {
        println!("Reading from sensor {}: {}", self.sensor_id, self.last_reading);
        self.last_reading
    }

    pub fn calibrate(&mut self) {
        println!("Calibrating sensor {}", self.sensor_id);
    }

    pub fn report_error(&self) {
        println!("Sensor {} error reported.", self.sensor_id);
    }
}

// WeatherStation Entity
pub struct WeatherStation {
    pub station_id: String,
    pub location: String,
    pub status: String,
}

impl WeatherStation {
    pub fn initialize(&self) {
        println!("Weather station {} initialized.", self.station_id);
    }

    pub fn send_data(&self) {
        println!("Station {} sending data...", self.station_id);
    }

    pub fn perform_check(&self) {
        println!("Station {} performing health check...", self.station_id);
    }
}

// WeatherReport Entity
pub struct WeatherReport {
    pub report_id: String,
    pub timestamp: String,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub location_id: String,
}

impl WeatherReport {
    pub fn generate(&self) {
        println!("Weather report {} generated.", self.report_id);
    }

    pub fn validate(&self) {
        println!("Validating report {}", self.report_id);
    }

    pub fn archive(&self) {
        println!("Archiving report {}", self.report_id);
    }
}

// ForecastRequest Entity
pub struct ForecastRequest {
    pub request_id: String,
    pub user_id: String,
    pub location_id: String,
    pub request_time: String,
    pub forecast_type: String,
}

impl ForecastRequest {
    pub fn submit(&self) {
        println!("Submitting forecast request {}", self.request_id);
    }

    pub fn process(&self) {
        println!("Processing forecast request {}", self.request_id);
    }

    pub fn return_forecast(&self) {
        println!("Returning forecast for request {}", self.request_id);
    }
}

// Alert Entity
pub struct Alert {
    pub alert_id: String,
    pub alert_type: String,
    pub severity: u8,
    pub issued_time: String,
    pub expiry_time: String,
    pub report_id: String,
}

impl Alert {
    pub fn trigger(&self) {
        println!("Triggering alert {}", self.alert_id);
    }

    pub fn cancel(&self) {
        println!("Canceling alert {}", self.alert_id);
    }

    pub fn notify_users(&self) {
        println!("Notifying users about alert {}", self.alert_id);
    }
}

