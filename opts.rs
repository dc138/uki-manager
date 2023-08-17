use std::env;

pub struct Opts {
    bool build_all;
}

pub fn parse_opts() -> Result<Opts, getopts::Fail> {
    let args = env::args().collect();
}
