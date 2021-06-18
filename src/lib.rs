use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;

pub mod error;

use error::Result;

pub fn calculate_etag_from_read(f: &mut dyn Read, chunk_size: usize) -> Result<String> {
    let mut md5 = Md5::new();
    let mut concat_md5 = Md5::new();
    let mut input_buffer = vec![0u8; chunk_size];
    let mut chunk_count = 0;
    let mut current_md5: Vec<u8> = repeat(0).take((md5.output_bits() + 7) / 8).collect();

    let md5_result = loop {
        let amount_read = f.read(&mut input_buffer)?;
        if amount_read > 0 {
            md5.reset();
            md5.input(&input_buffer[0..amount_read]);
            chunk_count += 1;
            md5.result(&mut current_md5);
            concat_md5.input(&current_md5);
        } else {
            if chunk_count > 1 {
                break format!("{}-{}", concat_md5.result_str(), chunk_count);
            } else {
                break md5.result_str();
            }
        }
    };
    Ok(md5_result)
}

pub fn calculate_etag(file: &str, chunk_size: usize) -> Result<String> {
    let mut f = File::open(file)?;
    calculate_etag_from_read(&mut f, chunk_size)
}

#[cfg(test)]
mod test_calculate_etag {
    use super::*;
    use std::fs::File;
    use std::io::{self, Read};

    /// util test function to write a test file
    fn _write(chunk: usize, file_name: &str) {
        let buf: Vec<u8> = repeat(0b101).take(chunk).collect();
        let mut file = File::create(file_name).unwrap();
        let _ = file.write_all(&buf);
    }
    /// util test function to write a bunch of test files
    fn _write_test_files() {
        _write(1000, "test_1_chunk.dat");
        _write(5242880 + 1000, "test_2_chunk.dat");
        _write(5242880 * 2 + 1000, "test_3_chunk.dat");
        _write(5242880, "test_exact_1_chunk.dat");
        _write(5242880 * 2, "test_exact_2_chunk.dat");
        _write(5242880 * 3, "test_exact_3_chunk.dat");
    }

    /*
    2019-12-22T17:16:44.000Z          0 d41d8cd98f00b204e9800998ecf8427e   STANDARD     tests/test_0_byes.dat
    2019-12-22T17:06:05.000Z       1000 5f9cab54dc1ebba728c0a1f3eb21ddb8   STANDARD     tests/test_1_chunk.dat
    2019-12-22T17:06:05.000Z    5243880 57f07dca152a449cdf231fa452729912-2 STANDARD     tests/test_2_chunk.dat
    2019-12-22T17:06:06.000Z   10486760 ef5a94b16ea22b69dccf1ca25aafeea4-3 STANDARD     tests/test_3_chunk.dat
    2019-12-22T17:06:06.000Z    5242880 4db6a21cb2d743a218d11db53c1c40b0   STANDARD     tests/test_exact_1_chunk.dat
    2019-12-22T17:06:07.000Z   10485760 4e02084b12268966f48d6ad889e6b862-2 STANDARD     tests/test_exact_2_chunk.dat
    2019-12-22T17:06:05.000Z   15728640 e0c05a33e6b27852e7514b2d80ad1aa4-3 STANDARD     tests/test_exact_3_chunk.dat
    */

    #[test]
    fn test_0_byte_chunk() {
        let mut buf = io::repeat(0b101).take(0);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_1_chunk() {
        let mut buf = io::repeat(0b101).take(1000);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "5f9cab54dc1ebba728c0a1f3eb21ddb8");
    }

    #[test]
    fn test_2_chunk() {
        let mut buf = io::repeat(0b101).take(5242880 + 1000);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "57f07dca152a449cdf231fa452729912-2");
    }

    #[test]
    fn test_3_chunk() {
        let mut buf = io::repeat(0b101).take(5242880 * 2 + 1000);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "ef5a94b16ea22b69dccf1ca25aafeea4-3");
    }

    #[test]
    fn test_exact_1_chunk() {
        let mut buf = io::repeat(0b101).take(5242880);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "4db6a21cb2d743a218d11db53c1c40b0");
    }

    #[test]
    fn test_exact_2_chunk() {
        let mut buf = io::repeat(0b101).take(5242880 * 2);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "4e02084b12268966f48d6ad889e6b862-2");
    }

    #[test]
    fn test_exact_3_chunk() {
        let mut buf = io::repeat(0b101).take(5242880 * 3);

        let actual = calculate_etag_from_read(&mut buf, 5242880);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "e0c05a33e6b27852e7514b2d80ad1aa4-3");
    }
}
