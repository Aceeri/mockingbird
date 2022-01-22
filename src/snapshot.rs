use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};

pub struct Snapshot {
    components: HashMap<TypeId, ComponentSnapshot>,
}

pub struct ComponentSnapshot<C: Component + Replicate> {
    entities: Vec<NetworkId>, // entity at each index
    data: Vec<Vec<u8>>,
}

impl<C> ComponentSnapshot<C>
where
    C: Component + Replicate,
{
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, entity: Entity, c: C) {
        bincode::encode
    }
}

pub struct PeerDelta {}

/// A ring buffer of snapshots to determine state.
pub struct SnapshotHistory {
    snapshots: VecDeque<Snapshot>,
}

impl SnapshotHistory {
    pub fn new() -> Self {
        Self {
            snapshots: VecDeq::default(),
        }
    }

    pub fn previous(&self) -> &Snapshot {}
}
