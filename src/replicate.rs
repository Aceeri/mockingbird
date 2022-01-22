use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};
use crate::prelude::*;

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

fn record_snapshot<C: Component + Replicate>(
    mut history: ResMut<SnapshotHistory>,
    components: Query<(&NetworkId, &C), Changed<C>>,
) {
    let mut snapshot = ComponentSnapshot::<C>::new();
    for (network_id, component) in components.iter() {
        snapshot.push(network_id.clone(), component);
    }
}