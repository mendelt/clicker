use crate::manifest::Template;
use failure::Error;

pub fn deploy(template: &Template) -> Result<(), Error> {
    println!("{:?}", template.source_path());

    get_template_files(template)?;

    Ok(())
}

fn get_template_files(template: &Template) -> Result<(), Error> {
    let source_path = template.source_path();

    for file in source_path.read_dir()? {
        let file = file?;
        let path = file.path();
        println!("{:?}", path);
    }

    Ok(())
}
