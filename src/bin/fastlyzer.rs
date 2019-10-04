#![deny(rust_2018_idioms)]
use fastlyzer::{reader, run, FastResult};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fastlyzer")]
struct Opt {
    /// Max number of entries to display
    #[structopt(short = "m", long, default_value = "10")]
    max: usize,
    /// The field in the logs which you want to count
    #[structopt(short = "s", long, default_value = "client_ip")]
    selector: String,
    /// One or more key=value pairs to AND by when filtering the results
    #[structopt(short = "f", long)]
    filter: Option<Vec<String>>,
    /// The log file or directory to be analyzed
    #[structopt(short = "i", long)]
    input: String,
}

fn main() -> FastResult<()> {
    let opt = Opt::from_args();
    let file = reader(&opt.input)?;
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
    run(file, opt.max, &opt.selector, filters);
    Ok(())
}
