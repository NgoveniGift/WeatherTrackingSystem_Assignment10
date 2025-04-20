// creational_patterns/singleton_pattern.rs

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub struct DatabaseConnection {
    pub url: String,
    pub pool_size: u32,
}

static INSTANCE: Lazy<Arc<Mutex<DatabaseConnection>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DatabaseConnection {
        url: "localhost:5432".to_string(),
        pool_size: 10,
    }))
});

pub fn get_db_instance() -> Arc<Mutex<DatabaseConnection>> {
    INSTANCE.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_instance_same_reference() {
        let conn1 = get_db_instance();
        let conn2 = get_db_instance();

        assert!(Arc::ptr_eq(&conn1, &conn2));
    }

    #[test]
    fn test_singleton_config_editable() {
        let conn = get_db_instance();
        {
            let mut db = conn.lock().unwrap();
            db.url = "remotehost:3306".to_string();
        }
        let db = get_db_instance();
        let db_locked = db.lock().unwrap();
        assert_eq!(db_locked.url, "remotehost:3306");
    }
}

