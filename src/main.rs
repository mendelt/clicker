use clap::App;
use clap::Arg;
use failure::Error;
use std::collections::HashMap;
use crate::manifest::Manifest;

mod manifest;

fn main() {
    let app = Application::from_options();

    println!("Hello click")
}


struct Application {
    /// The loaded manifest
    manifest: Manifest,

    /// User options from command line
    commandline_options: HashMap<String, String>,

    /// Template specified on command line
    template: Option<String>
}


impl Application {
    fn from_options() -> Result<Self, Error> {
        let matches = App::new("Clicker file and directory templater")
            .version("0.0.1")
            .arg(Arg::with_name("manifest")
                .short("m")
                .long("manifest"))
            .arg(Arg::with_name("template")
                .required(true)
                .index(1)).get_matches();

        let manifest = Manifest::parse_file(matches.value_of("manifest").unwrap_or(".clicker"))?;

        let template =
            if let Some(template_ref) = matches.value_of("template") {
                Some(template_ref.to_string())
            } else {
                None
            };

        Ok(Application {
            manifest,
            commandline_options: HashMap::new(),
            template,
        })
    }
}
