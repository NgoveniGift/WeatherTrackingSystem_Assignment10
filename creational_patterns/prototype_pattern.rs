// creational_patterns/prototype_pattern.rs

#[derive(Clone, Debug, PartialEq)]
pub struct ReportTemplate {
    pub title: String,
    pub content: String,
}

pub struct ReportTemplateCache {
    base_template: ReportTemplate,
}

impl ReportTemplateCache {
    pub fn new(template: ReportTemplate) -> Self {
        Self { base_template: template }
    }

    pub fn clone_template(&self) -> ReportTemplate {
        self.base_template.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_template() {
        let original = ReportTemplate {
            title: "Severe Weather Alert".into(),
            content: "High winds expected in your area.".into(),
        };

        let cache = ReportTemplateCache::new(original.clone());
        let clone = cache.clone_template();

        assert_eq!(original, clone);
        assert_ne!(&original as *const _, &clone as *const _); // Ensure they are different instances
    }
}

