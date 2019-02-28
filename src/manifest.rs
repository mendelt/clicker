use failure::Fail;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub base_path: PathBuf,

    pub values: BTreeMap<String, TemplateValue>,
    pub templates: BTreeMap<String, Template>,
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
mod tests {}
