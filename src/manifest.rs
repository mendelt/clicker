use failure::{Error, Fail};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug, PartialEq)]
pub struct Manifest {
    values: BTreeMap<String, TemplateValue>,
    templates: BTreeMap<String, Template>,
}

#[derive(Debug, PartialEq, Fail)]
enum ManifestError {
    #[fail(display = "error parsing manifest")]
    ParseError,
}

impl Manifest {
    fn parse_toml(manifest_text: &str) -> Result<Self, ManifestError> {
        if let Ok(Value::Table(manifest_value)) = manifest_text.parse() {
            Ok(Manifest {
                values: TemplateValue::parse_values(&manifest_value)?,
                templates: Template::parse_templates(&manifest_value),
            })
        } else {
            Err(ManifestError::ParseError)
        }
    }

    pub fn parse_file(path: &Path) -> Result<Self, Error> {
        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;

        Ok(Self::parse_toml(&content)?)
    }
}

#[derive(Debug, PartialEq)]
pub enum TemplateValue {
    /// Template value is given in the manifest
    Direct(String),

    /// Template value is given by the user either on the command line or interactively
    User(UserTemplateValue),
}

impl TemplateValue {
    fn parse_values(
        manifest: &BTreeMap<String, Value>,
    ) -> Result<BTreeMap<String, TemplateValue>, ManifestError> {
        if let Some(template_value) = manifest.get("values") {
            if let Value::Table(values) = template_value {
                Ok(BTreeMap::from_iter(values.into_iter().map(
                    |(name, value)| match value {
                        Value::String(direct_val) => (
                            name.to_owned(),
                            TemplateValue::Direct(direct_val.to_string()),
                        ),
                        _ => (
                            name.to_owned(),
                            TemplateValue::Direct("unknown".to_string()),
                        ),
                    },
                )))
            } else {
                // Values is not a map, error!
                Err(ManifestError::ParseError)
            }
        } else {
            // No values, return an empty map
            Ok(BTreeMap::new())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UserTemplateValue {
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
    fn parse_templates(val: &BTreeMap<String, Value>) -> BTreeMap<String, Template> {
        BTreeMap::new()
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
                values: BTreeMap::new(),
                templates: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn should_fail_parsing_invalid_value_map() {
        assert_eq!(
            Manifest::parse_toml(
                r#"
                values = "these_are_not_values"
            "#
            ),
            Err(ManifestError::ParseError)
        );
    }

    #[test]
    fn should_parse_string_values() {
        let man = Manifest::parse_toml(
            r#"
            [values]
            my_value = "stuff"
            my_other_value = "other stuff"
        "#,
        )
        .unwrap();

        assert_eq!(
            man.values["my_value"],
            TemplateValue::Direct("stuff".to_string())
        );
        assert_eq!(
            man.values["my_other_value"],
            TemplateValue::Direct("other stuff".to_string())
        );
    }

    #[test]
    #[ignore]
    fn should_parse_user_values() {
        let man = Manifest::parse_toml(
            r#"
            [values.my_value]
            user_prompt = "Please enter a value"
        "#,
        )
        .unwrap();

        assert_eq!(
            man.values["my_value"],
            TemplateValue::User(UserTemplateValue {
                user_prompt: "Please enter a value".to_string()
            })
        )
    }

    #[test]
    #[ignore]
    fn should_fail_parsing_invalid_value() {
        assert_eq!(
            Manifest::parse_toml(
                r#"
                [values]
                my_value = 4
            "#
            ),
            Err(ManifestError::ParseError)
        );
    }

    #[test]
    #[ignore]
    fn should_parse_templates() {
        let man = Manifest::parse_toml(
            r#"
            [templates.my_template]
            source = "source"
        "#,
        )
        .unwrap();

        assert_eq!(
            man.templates["my_template"],
            Template {
                name: "some_name".to_string(),
                base_path: PathBuf::new(),
                source: Some("source".to_string()),
                destination: None,
            }
        )
    }
}
