extern crate clap;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use dice_roller::runner::Runner;

fn main() {
    let config = App::new("roller")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .takes_value(false)
                .help("Display additional information"),
        )
        .arg(
            Arg::with_name("DEFINITION")
                .help("Define dice to roll in dice notation (e.x., 2d10, d6+2, 10d8-2)")
                .default_value("1d6")
                .index(1),
        )
        .get_matches();

    let runner = Runner::new(
        config.value_of("DEFINITION").unwrap().to_string(),
        config.is_present("verbose"),
    );
    runner.run()
}
