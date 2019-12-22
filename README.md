# Calculate S3 etag

# Usage

A cross-platform utility to calculte the etag of a file the way S3 would calculate it.

## Features

- Print the calculated etag for a list of provided files on the command line
- Configure the chunk size used when uploading the file to S3 (default to 8MB)

## Example Usage

### Calculate the etags for all the jpg files in the current directory

    calculate-etag *.jpg

## Command Line Usage

### General Usage

```
$ calculate-etag --help
calculate-etag 0.1.0
Bernard Niset


USAGE:
    calculate-etag [OPTIONS] <files>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --size <chunk-size>     [default: 8388608]

ARGS:
    <files>...    Files to process
```

# Development Guidelines

## Unit tests

Install cargo watch

    cargo install cargo-watch

Run unit tests. Unit tests need to run single threaded.

    cargo watch -w src -x "test"
