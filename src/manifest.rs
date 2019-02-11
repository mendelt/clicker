use serde_derive::Deserialize;
use std::collections::HashMap;
use failure::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


#[derive(Deserialize, Debug, PartialEq)]
pub struct Manifest {
    #[serde(default)]
    values: HashMap<String, Value>,

    #[serde(default)]
    templates: HashMap<String, Template>,

    #[serde(flatten)]
    default_template: Template,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Template {
    source: Option<String>,
    destination: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Value {
    DirectValue(String),
    UserValue(UserValueValue),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct UserValueValue {
    user_prompt: String
}

impl Manifest {
    fn from_string(manifest_text: &str) -> Self {
        toml::from_str(manifest_text).unwrap()
    }

    pub fn parse_file(path: &Path) -> Result<Self, Error> {
        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;

        Ok(Self::from_string(&content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_empty_manifest() {
        let man = Manifest::from_string("");

        assert_eq!(
            man,
            Manifest {
                values: HashMap::new(),
                templates: HashMap::new(),
                default_template: Template { source: None, destination: None }
            }
        );
    }

    #[test]
    fn should_parse_default_template() {
        let man = Manifest::from_string( r#"
            source = "source"
            destination = "destination"
        "#);

        assert_eq!(
            man.default_template,
            Template {
                source: Some("source".to_string()),
                destination: Some("destination".to_string())
            }
        );
    }

    #[test]
    fn should_parse_templates() {
        let man = Manifest::from_string(r#"
            [templates.my_template]
            source = "source"
        "#);

        assert_eq!(
            man.templates["my_template"],
            Template {
                source: Some("source".to_string()),
                destination: None
            }
        )
    }

    #[test]
    fn should_parse_string_values() {
        let man = Manifest::from_string(r#"
            [values]
            my_value = "stuff"
            my_other_value = "other stuff"
        "#);

        assert_eq!(man.values["my_value"], Value::DirectValue("stuff".to_string()));
        assert_eq!(man.values["my_other_value"], Value::DirectValue("other stuff".to_string()));
    }

    #[test]
    fn should_parse_user_values() {
        let man = Manifest::from_string(r#"
            [values.my_value]
            user_prompt = "Please enter a value"
        "#);

        assert_eq!(
            man.values["my_value"],
            Value::UserValue(UserValueValue {user_prompt: "Please enter a value".to_string()})
        )
    }
}
