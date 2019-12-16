use clap::{crate_description, crate_name, crate_version, App, AppSettings/* , Arg */};

pub fn new<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColorAuto)
        // .arg(
        //     Arg::with_name("option")
        //         .short("o")
        //         .long("option")
        //         .help("option description")
        //         .takes_value(true)
        //         .value_name("opts"),
        // )
}
