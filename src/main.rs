mod deploy;
mod manifest;
mod manifest_toml;

use crate::deploy::deploy;
use crate::manifest_toml::parse_manifest_file;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use failure::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let matches = App::new("Clicker: File and directory templater")
        .version("0.0.1")
        .arg(Arg::with_name("manifest").short("m").long("manifest"))
        .arg(Arg::with_name("template").required(true).index(1))
        .get_matches();

    let manifest_path = Path::new(matches.value_of("manifest").unwrap_or("."));
    let template_name = get_template_name(&matches);

    println!("{:?}", manifest_path);
    println!("{}", template_name);

    let manifest = parse_manifest_file(&manifest_path)?;
    let template = manifest.template_by_name(template_name.as_str()).unwrap();

    deploy(template)?;

    Ok(())
}

fn get_template_name(matches: &ArgMatches) -> String {
    matches.value_of("template").unwrap().to_string()
}
