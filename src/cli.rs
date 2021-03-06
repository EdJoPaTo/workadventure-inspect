use clap::{App, AppSettings, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("WorkAdventure Inspect")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("jitsi-host")
                .short("j")
                .long("jitsi-host")
                .value_name("URL")
                .takes_value(true)
                .help("The host being used by the Workadventure instance"),
        )
        .arg(
            Arg::with_name("workadventure-link")
                .value_name("URL")
                .takes_value(true)
                .help("URL to the work adventure including the map url"),
        )
}
