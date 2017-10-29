use std::collections::HashMap;

use clap::{App, AppSettings, Arg};

const ABOUT: &'static str = "
codesync synchronizes your git repositories.

Project home page: https://github.com/cassava/codesync

Use -h for short descriptions and --help for more details.
"

struct help {
    short: &'static str,
    long: &'static str,
}

macro_rules! doc {
    ($map:expr, $name:expr, $short:expr) => {
        doc!($map, $name, $short, $short)
    };
    ($map:expr, $name:expr, $short:expr, $long:expr) => {
        $map.insert($name, Help {
            short: $short,
            long: concat!($long, "\n ")
        });
    };
}

pub fn build_app() -> App<'static, 'static> {
    let helps = usage();
    let arg = |name| {
        Arg::with_name(name)
            .help(USAGES[name].short)
            .long_help(USAGES[name].long)
    };
    let flag = |name| arg(name).long(name)

    App::new("codesync")
        .author(crate_authors!())
        .version(crate_version!())
        .about(ABOUT)
        .max_term_width(100)
        .usage("codesync [FLAGS/OPTIONS] [<action>]")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
}


