use crate::manifest::Template;
use crate::manifest::{Manifest, TemplateValue};
use failure::Error;
use serde_derive::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Deserialize)]
struct ManifestToml {
    #[serde(skip)]
    base_path: PathBuf,

    #[serde(default)]
    values: BTreeMap<String, TemplateValueToml>,

    #[serde(default)]
    templates: BTreeMap<String, TemplateToml>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TemplateValueToml {
    Direct(String),
    User { prompt: String },
}

#[derive(Debug, PartialEq, Deserialize)]
struct TemplateToml {
    source: Option<String>,
    destination: Option<String>,
}

impl Into<Manifest> for ManifestToml {
    fn into(self) -> Manifest {
        Manifest {
            base_path: self.base_path.clone(),
            values: self
                .values
                .iter()
                .map(|(key, value)| match value {
                    TemplateValueToml::Direct(val) => {
                        (key.to_string(), TemplateValue::Direct(val.to_owned()))
                    }
                    TemplateValueToml::User { prompt } => (
                        key.to_string(),
                        TemplateValue::User {
                            prompt: prompt.to_string(),
                        },
                    ),
                })
                .collect(),
            templates: self
                .templates
                .iter()
                .map(|(key, value)| {
                    (
                        key.to_string(),
                        Template {
                            base_path: self.base_path.clone(),
                            name: key.to_string(),
                            source: value.source.to_owned(),
                            destination: value.destination.to_owned(),
                        },
                    )
                })
                .collect(),
        }
    }
}

pub fn parse_manifest_file(path: &Path) -> Result<Manifest, Error> {
    let path = path.canonicalize()?;

    let (base_path, manifest_path) = if path.is_dir() {
        (path.clone(), path.to_owned().join(".clicker"))
    } else {
        (path.parent().unwrap().to_owned(), path.clone())
    };

    println!("{:?} {:?}", base_path, manifest_path);

    let mut content = String::new();
    File::open(&manifest_path)?.read_to_string(&mut content)?;

    Ok(parse_manifest_toml(&content, path)?)
}

pub fn parse_manifest_toml(value: &str, base_path: PathBuf) -> Result<Manifest, Error> {
    let mut parsed: ManifestToml = toml::from_str(value)?;
    parsed.base_path = base_path;

    Ok(parsed.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::TemplateValue;

    #[test]
    fn should_parse_empty_manifest() {
        assert_eq!(
            parse_manifest_toml("", PathBuf::new()).unwrap(),
            Manifest {
                base_path: PathBuf::new(),
                values: BTreeMap::new(),
                templates: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn should_parse_string_values() {
        let manifest = parse_manifest_toml(
            r#"
            [values]
            my_value = "stuff"
            my_other_value = "other stuff"
        "#,
            PathBuf::new(),
        )
        .unwrap();

        assert_eq!(
            manifest.values["my_value"],
            TemplateValue::Direct("stuff".to_string())
        );
        assert_eq!(
            manifest.values["my_other_value"],
            TemplateValue::Direct("other stuff".to_string())
        );
    }

    #[test]
    fn should_parse_user_values() {
        assert_eq!(
            parse_manifest_toml(
                r#"
                [values.my_value]
                prompt = "Please enter a value"
            "#,
                PathBuf::new()
            )
            .unwrap()
            .values["my_value"],
            TemplateValue::User {
                prompt: "Please enter a value".to_string()
            }
        )
    }

    #[test]
    fn should_fail_parsing_invalid_value_list() {
        assert!(parse_manifest_toml(
            r#"
                values = "these_are_not_values"
            "#,
            PathBuf::new()
        )
        .is_err());
    }

    #[test]
    fn should_fail_parsing_invalid_value() {
        assert!(parse_manifest_toml(
            r#"
                [values]
                my_value = 4
            "#,
            PathBuf::new()
        )
        .is_err(),);
    }

    #[test]
    fn should_parse_templates() {
        let man = parse_manifest_toml(
            r#"
            [templates.some_name]
            source = "source"
        "#,
            PathBuf::new(),
        )
        .unwrap();

        assert_eq!(
            man.templates["some_name"],
            Template {
                name: "some_name".to_string(),
                base_path: PathBuf::new(),
                source: Some("source".to_string()),
                destination: None,
            }
        )
    }
}
