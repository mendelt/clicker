use crate::manifest::Manifest;
use crate::manifest::TemplateValue;
use failure::Error;
use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Deserialize)]
struct ManifestToml {
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

pub fn parse_manifest_toml(toml: &str) -> Result<Manifest, Error> {
    let manifest_toml: ManifestToml = toml::from_str(toml)?;

    Ok(Manifest {
        values: manifest_toml
            .values
            .iter()
            .map(|(key, value)| match value {
                TemplateValueToml::Direct(val) => {
                    (key.to_owned(), TemplateValue::Direct(val.to_owned()))
                }
                TemplateValueToml::User { prompt } => (
                    key.to_owned(),
                    TemplateValue::User {
                        prompt: prompt.to_string(),
                    },
                ),
            })
            .collect(),
        templates: BTreeMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::TemplateValue;

    #[test]
    fn should_parse_empty_manifest() {
        assert_eq!(
            parse_manifest_toml("").unwrap(),
            Manifest {
                values: BTreeMap::new(),
                templates: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn should_parse_string_values() {
        let man = parse_manifest_toml(
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
}
