use std::env;

pub struct Opts {
    pub help: bool,
    pub version: bool,
    pub gen_all: bool,
    pub global_config: String,
    pub usage: String,
}

pub fn parse_opts() -> Result<Opts, getopts::Fail> {
    let args: Vec<String> = env::args().collect();

    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "show copyright and version information");

    opts.optflag(
        "G",
        "gen-all",
        "try to generate a uki for all installed kernels",
    );

    opts.optopt("c", "config", "path to the global config file", "FILE");

    let matches = opts.parse(&args[1..])?;

    Ok(Opts {
        help: matches.opt_present("h"),
        version: matches.opt_present("v"),
        gen_all: matches.opt_present("G"),
        global_config: matches
            .opt_str("c")
            .unwrap_or("/etc/uki-manager/config.conf".to_owned()),
        usage: opts.usage("Usage: uki-manager [options]"),
    })
}
