use clap::{
    crate_authors, /* , Arg */
    crate_description, crate_name, crate_version, App, AppSettings,
};

pub fn new() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
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
