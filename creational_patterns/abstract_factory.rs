// creational_patterns/abstract_factory.rs

pub trait UIComponentFactory {
    fn create_button(&self) -> String;
    fn create_display_panel(&self) -> String;
}

pub struct MobileUIFactory;
impl UIComponentFactory for MobileUIFactory {
    fn create_button(&self) -> String {
        "Mobile Button".to_string()
    }

    fn create_display_panel(&self) -> String {
        "Mobile Display Panel".to_string()
    }
}

pub struct WebUIFactory;
impl UIComponentFactory for WebUIFactory {
    fn create_button(&self) -> String {
        "Web Button".to_string()
    }

    fn create_display_panel(&self) -> String {
        "Web Display Panel".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_ui_factory() {
        let factory = MobileUIFactory;
        assert_eq!(factory.create_button(), "Mobile Button");
        assert_eq!(factory.create_display_panel(), "Mobile Display Panel");
    }

    #[test]
    fn test_web_ui_factory() {
        let factory = WebUIFactory;
        assert_eq!(factory.create_button(), "Web Button");
        assert_eq!(factory.create_display_panel(), "Web Display Panel");
    }
}

