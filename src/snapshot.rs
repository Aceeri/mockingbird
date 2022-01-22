use bevy::prelude::*;
use bincode::{config::Configuration, Decode, Encode};
use crate::prelude::*;
use fxhash::FxHashMap;
use std::marker::PhantomData;
use std::any::TypeId;

/// A ring buffer of snapshots to determine state.
pub struct SnapshotHistory {
    snapshots: VecDeque<Snapshot>,
}

impl SnapshotHistory {
    pub fn new() -> Self {
        Self {
            snapshots: VecDeque::default(),
        }
    }

}

pub struct Snapshot {
    components: FxHashMap<TypeId, RawComponentSnapshot>,
}

pub struct RawComponentSnapshot {
    ids: Vec<NetworkId>, // associated network id at each index
    data: Vec<Vec<u8>>,
}

impl RawComponentSnapshot {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, id: NetworkId, data: Vec<u8>) {
        self.ids.push(id);
        self.data.push(data);
    }
}

pub struct ComponentSnapshot<C: Component + Replicate> {
    raw: RawComponentSnapshot,
    phantom: PhantomData<C>,
}

impl<C> ComponentSnapshot<C>
where
    C: Component + Replicate,
{
    pub fn new() -> Self {
        Self {
            raw: RawComponentSnapshot::new(),
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, id: NetworkId, component: &C) {
        let data = bincode::encode_to_vec(component, Configuration::standard()).unwrap();
        self.raw.push(id, data);
    }

    pub fn raw(self) -> (TypeId, RawComponentSnapshot) {
        (TypeId::of::<C>(), self.raw)
    }
}
