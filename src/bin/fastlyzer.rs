#![deny(rust_2018_idioms)]
use fastlyzer::{get_domains, read, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fastlyzer")]
struct Opt {
    #[structopt(short = "md", long, default_value = "10")]
    max_domains: usize,
    #[structopt(short = "i", long)]
    input: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let file = read(&opt.input)?;
    get_domains(file, opt.max_domains);
    Ok(())
}
