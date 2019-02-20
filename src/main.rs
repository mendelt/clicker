use crate::manifest::Manifest;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use failure::Error;
use std::path::{Path, PathBuf};

mod manifest;
mod manifest_toml;

fn main() -> Result<(), Error> {
    let app = Application::load();

    println!("{:?}", app.get_manifest_path()?);
    println!("{}", app.get_template());

    let manifest = Manifest::parse_file(app.get_manifest_path()?.as_ref());

    Ok(())
}

struct Application<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Application<'a> {
    fn load() -> Self {
        let matches = App::new("Clicker file and directory templater")
            .version("0.0.1")
            .arg(Arg::with_name("manifest").short("m").long("manifest"))
            .arg(Arg::with_name("template").required(true).index(1))
            .get_matches();

        Application { matches }
    }

    fn get_manifest_path(&self) -> Result<PathBuf, Error> {
        let manifest_path =
            Path::new(self.matches.value_of("manifest").unwrap_or(".clicker")).canonicalize()?;

        if manifest_path.is_dir() {
            Ok(manifest_path.join(".clicker"))
        } else {
            Ok(manifest_path.to_path_buf())
        }
    }

    fn get_template(&self) -> String {
        self.matches.value_of("template").unwrap().to_string()
    }
}
