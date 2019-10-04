use std::{
    collections::{BTreeMap, HashMap},
    io::{self, BufRead, Write},
};

use rayon::prelude::*;
use tabwriter::TabWriter;

/// Main run function.
///
/// Right now all of Fastlyzer's functionality is contained in this function.
pub fn run(reader: Box<dyn BufRead>, max: usize, selector: &str, filters: Vec<(&str, &str)>) {
    // We're going to use TabWriter to get a nicely spaced columnar output that matches screen/terminal size.
    let mut tw = TabWriter::new(io::stdout());

    // Constructs the Selector regex if one is provided, defaults to `client_ip`.
    // (:?"cache_status":)\s?\"?(.+?)\"?\s?[,}\s]
    let selector_matcher = {
        let regex = format!("(:?\"{}\":)\\s?\"?(.+?)\"?\\s?[,}}\\s]", selector);
        regex::Regex::new(&regex).unwrap()
    };

    // Constructs the Filter regex if one is provided, if not, returns an empty one.
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

    // Collects each line from the buffer into a vec.
    let lines: Vec<_> = reader.lines().map(std::result::Result::unwrap).collect();

    // We want to know the total number of logs to pre-allocate a HashMap.
    // Adding stuff to a pre-sized HashMap is faster since it does not have to re-allocate more
    // memory when we bump up against it's default size.
    let total_logs = lines.len();
    let mut hits: HashMap<&str, usize> = HashMap::with_capacity(total_logs);

    // We use Rayon to iterate over the lines in parallel. We don't care about the order of the
    // process here since we are doing our analysis on the entire data set and returning a sum.
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

    // Format the title of our output.
    writeln!(&mut tw, "{}\tHits\tHit %", selector).unwrap();
    // Turn our HashMap into a BTreeMap.
    // Using a BTreeMap to sort our keys by size.
    let hits: BTreeMap<usize, &str> = hits.into_par_iter().map(|(k, v)| (v, k)).collect();
    // Reverse the BTreeMap and take the `max` number of results. Defaults to 10.
    // Calculate the results as percentages and write it to the TabWriter.
    for (count, field) in hits.iter().rev().take(max) {
        let percentage = (*count as f32 / total_logs as f32) * 100.0;
        writeln!(&mut tw, "{}\t{}\t{:.3}%", field, count, percentage).unwrap();
    }

    // Print the TabWriter our to stdout.
    tw.flush().unwrap();
}
