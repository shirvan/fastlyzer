use std::{
    collections::{BTreeMap, HashMap},
    io::{self, BufRead, Write},
};

use rayon::prelude::*;
use tabwriter::TabWriter;

pub fn run(
    reader: Box<dyn BufRead>,
    max: usize,
    selector: &str,
    filters: Vec<(&str, &str)>,
) {
    let mut tw = TabWriter::new(io::stdout());

    // (:?"cache_status":)\s?\"?(.+?)\"?\s?[,}\s]
    let selector_matcher = {
        let regex = format!("(:?\"{}\":)\\s?\"?(.+?)\"?\\s?[,}}\\s]", selector);
        regex::Regex::new(&regex).unwrap()
    };

    // (:?"cache_status":)\s?"?(:?MISS-CLUSTER)"?\s?[,}\s]
    let filter_matcher = {
        let filters: Vec<String> = filters
            .iter()
            .map(|(key, value)| format!("(:?\"{}\":)\\s?\"?(:?{})\"?\\s?[,}}\\s]", key, value))
            .collect();

        if !filters.is_empty() {
            Some(regex::RegexSet::new(&filters).unwrap())
        } else {
            None
        }
    };

    let lines: Vec<_> = reader.lines().map(std::result::Result::unwrap).collect();

    let total_logs = lines.len();
    let mut hits: HashMap<&str, usize> = HashMap::with_capacity(total_logs);
    lines
        .par_iter()
        .filter(|line| {
            filter_matcher
                .as_ref()
                .map(|matcher| {
                    let size = matcher.len();
                    let number_of_matches = matcher.matches(&line).into_iter().count();

                    number_of_matches == size
                })
                .unwrap_or(true)
        })
        .map(|line| {
            selector_matcher
                .captures(line)
                .and_then(|capture| capture.get(2))
                .map(|capture| capture.as_str())
                .unwrap_or("<Missing Field>")
        })
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|field| {
            hits.entry(field)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    writeln!(&mut tw, "{}\tHits\tHit %", selector).unwrap();
    let hits: BTreeMap<usize, &str> = hits.into_par_iter().map(|(k, v)| (v, k)).collect();
    for (count, field) in hits.iter().rev().take(max) {
        let percentage = (*count as f32 / total_logs as f32) * 100.0;
        writeln!(&mut tw, "{}\t{}\t{:.3}%", field, count, percentage).unwrap();
    }

    tw.flush().unwrap();
}
