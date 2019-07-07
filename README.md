# About

A small command line app that will download parkrun results to a csv file.

## Usage

```console
USAGE:
    proff --url <parkrun url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --url <parkrun url>    The URL containing the park run results.
```

## Example

```console
$ # macOS
$ ./proff -u https://www.parkrun.org.uk/aylesbury/results/latestresults/ > out.csv
```

```console
$ # windows
$ .\proff.exe -u https://www.parkrun.org.uk/aylesbury/results/latestresults/ > out.csv
```