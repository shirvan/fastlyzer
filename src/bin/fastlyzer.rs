#![deny(rust_2018_idioms)]
use fastlyzer::{get_domains, read, FastResult};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fastlyzer")]
struct Opt {
    #[structopt(short = "m", long, default_value = "10")]
    max: usize,
    #[structopt(short = "s", long, default_value = "client_ip")]
    selector: String,
    #[structopt(short = "f", long)]
    filter: Option<Vec<String>>,
    #[structopt(short = "i", long)]
    input: String,
}

fn main() -> FastResult<()> {
    let opt = Opt::from_args();
    let file = read(&opt.input)?;
    let filters = opt
        .filter
        .as_ref()
        .map(|filter| {
            filter
                .iter()
                .map(|filter| {
                    let split = filter.splitn(2, '=').take(2).collect::<Vec<&str>>();
                    (split[0], split[1])
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);
    get_domains(file, opt.max, &opt.selector, filters);
    Ok(())
}
