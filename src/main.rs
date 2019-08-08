use clap::{App, Arg};
use librarian::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    env_logger::init();
    let app = App::new("Derive file structure from tags")
        .version("0.1")
        .author("Christian Dietz")
        .about("Derive file structure from tags")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("Source Directory")
                .help("Source Directory")
                .required(true),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("Target Directory")
                .default_value(".")
                .help("Entry point for derived file structure")
                .takes_value(true),
        )
        .get_matches();

    let source = PathBuf::from(app.value_of("source").expect("Source"));
    let target = PathBuf::from(app.value_of("target").expect("Target"));

    println!(
        "\nSource: \"{}\" -> Target: \"{}\"",
        source.display(),
        target.display()
    );

    librarian::tags_to_file_system(source, target)
}
