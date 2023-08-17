use std::env;

pub struct Opts {
    pub help: bool,
    pub version: bool,
    pub genall: bool,
    pub usage: String,
}

pub fn parse_opts() -> Result<Opts, getopts::Fail> {
    let args: Vec<String> = env::args().collect();

    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "show copyright and version information");

    opts.optflag(
        "G",
        "genall",
        "try to generate a uki for all installed kernels",
    );

    let matches = opts.parse(&args[1..])?;

    Ok(Opts {
        help: matches.opt_present("h"),
        version: matches.opt_present("v"),
        genall: matches.opt_present("G"),
        usage: opts.usage("Usage: uki-manager [options]"),
    })
}
