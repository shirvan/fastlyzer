use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, BufRead, Write};
use tabwriter::TabWriter;

use entry::Entry;
pub use errors::Result;
pub use rw::read;

pub mod entry;
pub mod errors;
pub mod rw;

pub fn get_domains(reader: Box<dyn BufRead>, max: usize) {
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let logs: Vec<Entry> = lines
        .iter()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    domain_req(&logs, max)
}

fn domain_req(logs: &[Entry], max: usize) {
    let mut tw = TabWriter::new(io::stdout());
    let total_logs = logs.len();
    // Speed(Matt): We want to be able to sort by size as we insert
    let mut hits: HashMap<Cow<str>, usize> = HashMap::new();
    for log in logs.iter() {
        hits.entry(log.origin_host.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let hits: BTreeMap<usize, Cow<str>> = hits.into_iter().map(|(k, v)| (v, k)).collect();
    writeln!(&mut tw, "Domain\tHits\tHit Percentage").unwrap();
    for (count, domain) in hits.iter().rev().take(max) {
        let percentage = (*count as f32 / total_logs as f32) * 100.0;
        writeln!(&mut tw, "{}\t{}\t{:.3}%", domain, count, percentage).unwrap();
    }

    tw.flush().unwrap();
}
