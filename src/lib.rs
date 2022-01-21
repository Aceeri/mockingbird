#![feature(cstring_from_vec_with_nul)]

use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};
use std::collections::VecDeque;
use std::io::Write;

/// A single instance of the changed game state of the world at one time.
/// 
/// When the round starts this will be somewhat large, but recurring
/// snapshots will be a delta based on this.
/// 
/// Driven by the `Changed` filter in the ECS.
pub struct Snapshot(Vec<u8>);

pub struct PeerDelta {

}

/// A ring buffer of snapshots to determine state.
pub struct History {
    snapshots: VecDeque<Snapshot>,
}

// Anything that is going to be packed and pushed over the network.
pub trait Replicate: Encode + Decode { }

pub enum EncodeError {
    Bincode(bincode::error::EncodeError),
    Custom(String),
}

pub enum DecodeError {
    Bincode(bincode::error::DecodeError),
    Custom(String),
}

// Just an index for the client/server.
#[derive(Debug, Clone)]
pub struct PeerId(u32);

// Who owns something.
#[derive(Debug, Clone)]
pub enum PeerOwnership {
    Any,
    Specific(PeerId),
}

// Network ID is just a remote entity id/generation.
//
// We probably need some sort of consensus stuff here if
// we are doing peer to peer, but for now I'll just
// assume we are a client <-> server model.
#[derive(Debug, Clone)]
pub struct NetworkId {
    from: PeerId,
    entity: Entity,
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

#[cfg(test)]
mod test {
    use super::*;
    use bitpacking::{BitPacker, BitPacker1x};
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io;
    use std::io::BufReader;
    use std::io::prelude::*;

    #[derive(Encode, Decode, Component, Debug, Clone)]
    pub struct Test(u32);

    fn xor<'a, 'b>(first: impl Iterator<Item = &'a u8>, second: impl Iterator<Item = &'b u8>) -> Vec<u8> {
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
        //let mut tests = vec![Test(5), Test(180), Test(3812)];
        let mut tests = Vec::new();
        for i in 0..(32 * 90) {
            tests.push(Test(i));
        }

        let config = Configuration::standard();

        let encoded: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();
        tests[0].0 += 6;
        let encoded2: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();
        for (index, mut test) in tests.iter_mut().enumerate() {
            test.0 += index as u32;
        }
        let encoded3: Vec<u8> = bincode::encode_to_vec(&tests, config).unwrap();

        println!("e1: {:?}", encoded);
        println!("e2: {:?}", encoded2);
        println!("e3: {:?}", encoded3);

        let delta = xor(encoded.iter(), encoded2.iter());
        println!("delta: {:?}", delta);
        let newest = xor(encoded2.iter(), encoded3.iter());
        println!("newest: {:?}", newest);
        let delta_updated = xor(delta.iter(), newest.iter());
        println!("delta_updated: {:?}", delta_updated);
        // apply delta

        let applied = xor(encoded.iter(), delta_updated.iter());
        println!("applied: {:?}", applied);

        let encode = delta;
        let buf = Vec::new();
        let mut encoder = ZlibEncoder::new(buf, Compression::fast());
        encoder.write_all(&encode.as_slice()).unwrap();
        //encoder.read_to_end(delta_updated.as_slice()).unwrap();
        encoder.flush().unwrap();
        let buffer = encoder.finish().unwrap();

        println!("{:?}", &buffer);
        println!("compressed len: {:?}", buffer.len());
        println!("uncompressed len: {:?}", encode.len());
        println!("compression ratio: {:?}", buffer.len() as f64 / encode.len() as f64);
    }
}
