use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};

// Anything that is going to be packed and pushed over the network.
pub trait Replicate: Encode + Decode {}

pub enum EncodeError {
    Bincode(bincode::error::EncodeError),
    Custom(String),
}

pub enum DecodeError {
    Bincode(bincode::error::DecodeError),
    Custom(String),
}
