use std::path::Path;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct ServerCli {
    #[structopt(short, long, default_value = "8000")]
    pub port: u32,
    #[structopt(long, short)]
    pub root: String,

    #[structopt(long, short, default_value = "0.0.0.0")]
    pub bind: String,
}
