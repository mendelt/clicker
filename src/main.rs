use clap::App;
use clap::Arg;
use crate::manifest::Manifest;

mod manifest;

fn main() {

    // let app = Application::from_options();


    println!("Hello click")
}


struct Application {
    manifest: Manifest,
    template: String
}


impl Application {
    fn from_options() -> () {
        let matches = App::new("Clicker file and directory templater")
            .version("0.0.1")
            .arg(Arg::with_name("manifest")
                .short("m")
                .long("config"))
            .arg(Arg::with_name("Template")
                .required(true)
                .index(1)).get_matches();

    }
}
