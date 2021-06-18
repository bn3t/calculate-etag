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

## Integration tests

Upload files specifying chunk size of 5MB using s3cmd:

    s3cmd put --multipart-chunk-size-mb=5 *.dat s3://xxx/test/

List files using -l to see etag information:

    > s3cmd ls -l s3://xxx/test/
    2021-06-18 16:11         1000  5f9cab54dc1ebba728c0a1f3eb21ddb8     STANDARD     s3://test.smartobjects.be/test/test_1_chunk.dat
    2021-06-18 16:11      5243880  57f07dca152a449cdf231fa452729912-2   STANDARD     s3://test.smartobjects.be/test/test_2_chunk.dat
    2021-06-18 16:11     10486760  ef5a94b16ea22b69dccf1ca25aafeea4-3   STANDARD     s3://test.smartobjects.be/test/test_3_chunk.dat
    2021-06-18 16:11      5242880  4db6a21cb2d743a218d11db53c1c40b0     STANDARD     s3://test.smartobjects.be/test/test_exact_1_chunk.dat
    2021-06-18 16:11     10485760  4e02084b12268966f48d6ad889e6b862-2   STANDARD     s3://test.smartobjects.be/test/test_exact_2_chunk.dat
    2021-06-18 16:11     15728640  e0c05a33e6b27852e7514b2d80ad1aa4-3   STANDARD     s3://test.smartobjects.be/test/test_exact_3_chunk.dat

Execute calculate_etag locally to verify:

    ❯ cargo run --  --size 5242880 *.dat
    Compiling calculate_etag v0.1.0 (/Users/bernard/projects/rust/calculate-etag)
        Finished dev [unoptimized + debuginfo] target(s) in 1.52s
        Running `target/debug/calculate_etag --size 5242880 test_1_chunk.dat test_2_chunk.dat test_3_chunk.dat test_exact_1_chunk.dat test_exact_2_chunk.dat test_exact_3_chunk.dat`
    5f9cab54dc1ebba728c0a1f3eb21ddb8 test_1_chunk.dat
    57f07dca152a449cdf231fa452729912-2 test_2_chunk.dat
    ef5a94b16ea22b69dccf1ca25aafeea4-3 test_3_chunk.dat
    4db6a21cb2d743a218d11db53c1c40b0 test_exact_1_chunk.dat
    4e02084b12268966f48d6ad889e6b862-2 test_exact_2_chunk.dat
    e0c05a33e6b27852e7514b2d80ad1aa4-3 test_exact_3_chunk.dat

Clean files on s3:

    aws s3 rm s3://xxx/test/ --exclude "*" --include "*.dat" --recursive
