use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(default)]
    name: String,

    #[serde(default)]
    values: HashMap<String, String>,
    templates: Vec<Template>,
}

impl Manifest {
    fn from_string(manifest_text: &str) -> Self {
        println!("{}", manifest_text);
        toml::from_str(manifest_text).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Template {
    name: String,

    source: Option<String>,
    destination: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_manifest_name() {

        let man = Manifest::from_string(r#"
name= "name"
[templates]
        "#);

        assert_eq!(man.name, "name".to_string());
    }

    #[test]
    fn should_parse_empty_name() {
        let man = Manifest::from_string( r#"[templates]"#);

        assert_eq!(man.name, String::default())
    }

    #[test]
    fn should_parse_values() {
        let man = Manifest::from_string(r#"
name = "name"

[templates]
        "#);

        assert_eq!(man.values["value1"], "value one".to_string());
        assert_eq!(man.values["value2"], "value two".to_string());
    }

    #[test]
    fn should_parse_templates() {
        let man = Manifest::from_string(r#"
name = "name"

[values]
[templates]
        "#);

    }
}
