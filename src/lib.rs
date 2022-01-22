#![feature(cstring_from_vec_with_nul)]

use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};
use std::collections::VecDeque;
use std::io::Write;

pub mod network_id;
pub mod replicate;
pub mod snapshot;

pub mod prelude {
    pub use super::*;
    pub use network_id::*;
    pub use replicate::*;
    pub use snapshot::*;
}

// Just an index for the client/server.
#[derive(Debug, Clone)]
pub struct PeerId(u8);

// Who owns something.
#[derive(Debug, Clone)]
pub enum PeerOwnership {
    Any,
    Specific(PeerId),
}
// We own this entity, do not defer to anything sending updates for it.
//
// This should only be on entities in a server in a client <-> server model.
#[derive(Component, Debug, Clone)]
pub struct Own;

// We are allowed to predict this entity moving/etc. but defer to any updates.
#[derive(Component, Debug, Clone)]
pub struct Predict(PeerOwnership);

// Don't predict this at all, just defer to server.
#[derive(Component, Debug, Clone)]
pub struct Defer(PeerOwnership);

fn record<C: Component + Replicate>(
    mut history: ResMut<SnapshotHistory>,
    components: Query<(Entity, &NetworkId, &C), Changed<C>>,
) {
    for (network_id, component) in components.iter() {}
}

#[cfg(test)]
mod test {
    use super::*;
    use bitpacking::{BitPacker, BitPacker1x};
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use rand::RngCore;
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

    #[test]
    fn encode_test() {
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
            if index % 10 == 0 {
                test.0 += (rand::thread_rng().next_u32() / 5);
            }
        }
        let encoded3: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();

        //println!("e1: {:?}", encoded);
        //println!("e2: {:?}", encoded2);
        //println!("e3: {:?}", encoded3);

        let delta = xor(encoded.iter(), encoded2.iter());
        //println!("delta: {:?}", delta);
        let newest = xor(encoded2.iter(), encoded3.iter());
        //println!("newest: {:?}", newest);
        let delta_updated = xor(delta.iter(), newest.iter());
        //println!("delta_updated: {:?}", delta_updated);
        // apply delta

        let applied = xor(encoded.iter(), delta_updated.iter());
        //println!("applied: {:?}", applied);

        let encode = delta_updated;
        let buf = Vec::new();
        let mut encoder = ZlibEncoder::new(buf, Compression::fast());
        encoder.write_all(&encode.as_slice()).unwrap();
        //encoder.read_to_end(delta_updated.as_slice()).unwrap();
        encoder.flush().unwrap();
        let buffer = encoder.finish().unwrap();

        let buffer = zstd::block::compress(&encode.as_slice(), 0).unwrap();
        let bytes = zstd::block::decompress(&buffer.as_slice(), size).unwrap();

        //println!("{:?}", &bytes);
        assert_eq!(encode, bytes);

        //println!("{:?}", &buffer);
        println!("compressed len: {:?}", buffer.len());
        println!("uncompressed len: {:?}", encode.len());
        println!(
            "compression ratio: {:?}",
            buffer.len() as f64 / encode.len() as f64
        );
    }
}
