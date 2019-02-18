use failure::{Error, Fail};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use toml::Value;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Manifest {
    values: HashMap<String, TemplateValue>,
    templates: HashMap<String, Template>,
    default_template: Option<Template>,
}


#[derive(Debug, Fail)]
enum ManifestError {
    #[fail(display = "error parsing manifest")]
    ParseError {},

}


impl Manifest {
    fn parse_toml(manifest_text: &str) -> Result<Self, ManifestError> {
        if let Ok(Value::Table(manifest_value)) = manifest_text.parse() {
            Ok(Manifest {
                values: TemplateValue::parse_values(&manifest_value),
                templates: Template::parse_templates(&manifest_value),
                default_template: Template::parse_default(&manifest_value),
            })
        } else {
            Err(ManifestError::ParseError {})
        }
    }

//    pub fn parse_file(path: &Path) -> Result<Self, Error> {
//        let mut content = String::new();
//        File::open(path)?.read_to_string(&mut content)?;
//
//        Ok(Self::parse_string(&content))
//    }
}

#[derive(Debug, PartialEq)]
pub enum TemplateValue {
    Direct(String),
    User(UserValueValue),
}

impl TemplateValue {
    fn parse_values(val: &BTreeMap<String, Value>) -> HashMap<String, TemplateValue> {
        HashMap::new()
    }
}


#[derive(Debug, PartialEq)]
pub struct UserValueValue {
    user_prompt: String,
}


#[derive(Debug, PartialEq)]
pub struct Template {
    base_path: PathBuf,
    name: String,

    source: Option<String>,
    destination: Option<String>,
}


impl Template {
    fn parse_templates(val: &BTreeMap<String, Value>) -> HashMap<String, Template> {
        HashMap::new()
    }

    fn parse_default(val: &BTreeMap<String, Value>) -> Option<Template> {
        None
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_empty_manifest() {
        let man = Manifest::parse_toml("").unwrap();

        assert_eq!(
            man,
            Manifest {
                values: HashMap::new(),
                templates: HashMap::new(),
                default_template: None
            }
        );
    }

//    #[test]
//    fn should_parse_default_template() {
//        let man = Manifest::from_string(
//            r#"
//            source = "source"
//            destination = "destination"
//        "#,
//        );
//
//        assert_eq!(
//            man.default_template,
//            Template {
//                source: Some("source".to_string()),
//                destination: Some("destination".to_string())
//            }
//        );
//    }
//
//    #[test]
//    fn should_parse_templates() {
//        let man = Manifest::from_string(
//            r#"
//            [templates.my_template]
//            source = "source"
//        "#,
//        );
//
//        assert_eq!(
//            man.templates["my_template"],
//            Template {
//                source: Some("source".to_string()),
//                destination: None
//            }
//        )
//    }
//
//    #[test]
//    fn should_parse_string_values() {
//        let man = Manifest::from_string(
//            r#"
//            [values]
//            my_value = "stuff"
//            my_other_value = "other stuff"
//        "#,
//        );
//
//        assert_eq!(
//            man.values["my_value"],
//            Value::DirectValue("stuff".to_string())
//        );
//        assert_eq!(
//            man.values["my_other_value"],
//            Value::DirectValue("other stuff".to_string())
//        );
//    }
//
//    #[test]
//    fn should_parse_user_values() {
//        let man = Manifest::from_string(
//            r#"
//            [values.my_value]
//            user_prompt = "Please enter a value"
//        "#,
//        );
//
//        assert_eq!(
//            man.values["my_value"],
//            Value::UserValue(UserValueValue {
//                user_prompt: "Please enter a value".to_string()
//            })
//        )
//    }
}
