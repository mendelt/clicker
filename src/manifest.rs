use failure::Fail;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub base_path: PathBuf,

    pub values: BTreeMap<String, TemplateValue>,
    pub templates: BTreeMap<String, Template>,
}

impl Manifest {
    pub fn template_by_name(&self, name: &str) -> Option<&Template> {
        self.templates.get(name)
    }
}

#[derive(Debug, PartialEq, Fail)]
enum ManifestError {
    #[fail(display = "error parsing manifest")]
    ParseError,
}

#[derive(Debug, PartialEq)]
pub enum TemplateValue {
    /// Template value is given in the manifest
    Direct(String),

    /// Template value is given by the user either on the command line or interactively
    User { prompt: String },
}

#[derive(Debug, PartialEq)]
pub struct Template {
    pub base_path: PathBuf,
    pub name: String,

    pub source: Option<String>,
    pub destination: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    /// Construct a dummy manifest with some values for testing
    fn test_manifest() -> Manifest {
        let mut templates: BTreeMap<String, Template> = BTreeMap::new();

        templates.insert(
            "first_template".to_string(),
            Template {
                name: "first_template".to_string(),
                base_path: PathBuf::new(),
                source: Some("source".to_string()),
                destination: Some("destination".to_string()),
            },
        );

        templates.insert(
            "second_template".to_string(),
            Template {
                name: "second_template".to_string(),
                base_path: PathBuf::new(),
                source: Some("source".to_string()),
                destination: Some("destination".to_string()),
            },
        );

        Manifest {
            base_path: PathBuf::new(),
            values: BTreeMap::new(),
            templates: templates,
        }
    }

    #[test]
    fn should_get_template_by_name() {
        let manifest = test_manifest();
        let template = manifest.template_by_name("second_template").unwrap();

        assert_eq!(template.name, "second_template");
    }

    #[test]
    fn should_return_none_for_none_existing_template() {
        let manifest = test_manifest();
        let template = manifest.template_by_name("other_name");

        assert_eq!(template, None);
    }
}
