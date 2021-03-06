use criterion::{criterion_group, criterion_main, Criterion};

use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};
use bitpacking::{BitPacker, BitPacker1x};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Encode, Decode, Component, Debug, Clone)]
pub struct Test(u32);

fn xor<'a, 'b>(
    first: impl Iterator<Item = &'a u8>,
    second: impl Iterator<Item = &'b u8>,
) -> Vec<u8> {
    first.zip(second).map(|(f, s)| f ^ s).collect::<Vec<u8>>()
}

fn as_u32<'a>(list: &'a Vec<u8>) -> &'a [u32] {
    unsafe {
        let (prefix, u32s, suffix) = list.as_slice().align_to::<u32>();
        u32s
    }
}

criterion_group!(benches, zlib, zstd);
criterion_main!(benches);

fn zlib(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("zlib");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    let mut tests = Vec::new();
    for i in 0..1_000 {
        tests.push(Test(i));
    }

    let config = Configuration::standard();

    let encoded: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();
    tests[0].0 += 6;
    let encoded2: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();
    for (index, mut test) in tests.iter_mut().enumerate() {
        test.0 += index as u32;
        //test.0 += 6;
    }
    let encoded3: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();

    let delta = xor(encoded.iter(), encoded2.iter());
    let newest = xor(encoded2.iter(), encoded3.iter());
    let delta_updated = xor(delta.iter(), newest.iter());
    // apply delta
    let applied = xor(encoded.iter(), delta_updated.iter());
    let encode = delta_updated;
    //println!("{:?}", &encode);

    let buf = Vec::new();
    let mut encoder = ZlibEncoder::new(buf, Compression::fast());
    encoder.write_all(&encode.as_slice()).unwrap();
    encoder.flush().unwrap();
    let buffer = encoder.finish().unwrap();

    let mut decoder = ZlibDecoder::new(&buffer[..]);
    let mut bytes = Vec::new();
    decoder.read_to_end(&mut bytes).unwrap();

    //println!("{:?}", &bytes);
    assert_eq!(bytes, encode);

    group.bench_function(format!("encode"), |bencher| {
        bencher.iter(|| {
            let buf = Vec::new();
            let mut encoder = ZlibEncoder::new(buf, Compression::fast());
            encoder.write_all(&encode.as_slice()).unwrap();
            encoder.flush().unwrap();
            let buffer = encoder.finish().unwrap();
        });
    });

    group.bench_function(format!("decode"), |bencher| {
        bencher.iter(|| {
            let mut decoder = ZlibDecoder::new(&buffer[..]);
            let mut bytes = Vec::new();
            decoder.read_to_end(&mut bytes).unwrap();
        });
    });

    group.finish();
}

fn zstd(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("zstd");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));

    let mut tests = Vec::new();
    let amount: u32 = 1_000_000;
    let size = std::mem::size_of::<Test>() * (amount as usize * 2);
    for i in 0..amount {
        tests.push(Test(i));
    }

    let config = Configuration::standard();

    let encoded: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();
    tests[0].0 += 6;
    let encoded2: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();
    for (index, mut test) in tests.iter_mut().enumerate() {
        test.0 += index as u32;
        //test.0 += 6;
    }
    let encoded3: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();

    let delta = xor(encoded.iter(), encoded2.iter());
    let newest = xor(encoded2.iter(), encoded3.iter());
    let delta_updated = xor(delta.iter(), newest.iter());
    // apply delta
    let applied = xor(encoded.iter(), delta_updated.iter());
    let encode = delta_updated;
    //println!("{:?}", &encode);

    let buffer = zstd::block::compress(&encode.as_slice(), 0).unwrap();
    let bytes = zstd::block::decompress(&buffer.as_slice(), size).unwrap();

    //println!("{:?}", &bytes);
    assert_eq!(encode, encode);

    group.bench_function(format!("encode"), |bencher| {
        bencher.iter(|| {
            let buffer = zstd::block::compress(&encode.as_slice(), 0).unwrap();
        });
    });

    group.bench_function(format!("decode"), |bencher| {
        bencher.iter(|| {
            let bytes = zstd::block::decompress(&buffer.as_slice(), size).unwrap();
        });
    });

    group.finish();
}
