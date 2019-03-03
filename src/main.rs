use crate::manifest_toml::parse_manifest_file;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use failure::Error;
use std::path::{Path, PathBuf};

mod manifest;
mod manifest_toml;

fn main() -> Result<(), Error> {
    let matches = App::new("Clicker file and directory templater")
        .version("0.0.1")
        .arg(Arg::with_name("manifest").short("m").long("manifest"))
        .arg(Arg::with_name("template").required(true).index(1))
        .get_matches();

    let manifest_path = get_manifest_path(&matches)?;
    let template_name = get_template_name(&matches);

    println!("{:?}", manifest_path);
    println!("{}", template_name);

    let manifest = parse_manifest_file(&manifest_path)?;
    let _template = manifest.template_by_name(template_name.as_str());

    Ok(())
}

fn get_manifest_path(matches: &ArgMatches) -> Result<PathBuf, Error> {
    let manifest_path =
        Path::new(matches.value_of("manifest").unwrap_or(".clicker")).canonicalize()?;

    if manifest_path.is_dir() {
        Ok(manifest_path.join(".clicker"))
    } else {
        Ok(manifest_path.to_path_buf())
    }
}

fn get_template_name(matches: &ArgMatches) -> String {
    matches.value_of("template").unwrap().to_string()
}
