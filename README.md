# Fastlyzer

A log reader and analyzer. 
Originally designed to analyze Fastly logs but can be used for any log file containing one JSON entry per line.

```bash
fastlyzer 0.1.0

USAGE:
    fastlyzer [OPTIONS] --input <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <filter>...     One or more key=value pairs to AND by when filtering the results
    -i, --input <input>          The log file to be analyzed
    -m, --max <max>              Max number of entries to display [default: 10]
    -s, --selector <selector>    The field in the logs which you want to count [default: client_ip]
```