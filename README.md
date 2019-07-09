[![Build Status][azure-badge]][azure-url]
[![MIT licensed][license-badge]][license-url]
[![dependency status][dependency-badge]][dependency-url]

# About

A small command line app that will download parkrun results to a csv file.

## Latest Release

[Latest Release Page][latest-release]

### Downloads

* [x86_64-apple-darwin.zip][mac-release]
* [x86_64-pc-windows-msvc.zip][windows-release]
* [x86_64-unknown-linux-musl.zip][linux-release]

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

[azure-badge]: https://dev.azure.com/morpork73/parkrun-offline/_apis/build/status/ian-hamlin.parkrun-offline?branchName=master
[azure-url]: https://dev.azure.com/morpork73/parkrun-offline/_build/latest?definitionId=8&branchName=master
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: LICENSE
[dependency-badge]: https://deps.rs/repo/github/ian-hamlin/parkrun-offline/status.svg
[dependency-url]: https://deps.rs/repo/github/ian-hamlin/parkrun-offline
[latest-release]: https://github.com/ian-hamlin/parkrun-offline/releases/latest
[mac-release]: https://github.com/ian-hamlin/parkrun-offline/releases/latest/download/x86_64-apple-darwin.zip
[windows-release]: https://github.com/ian-hamlin/parkrun-offline/releases/latest/download/x86_64-pc-windows-msvc.zip
[linux-release]: https://github.com/ian-hamlin/parkrun-offline/releases/latest/download/x86_64-unknown-linux-musl.zip