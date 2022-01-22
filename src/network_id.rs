use bevy::prelude::*;

// We probably need some sort of consensus stuff here if
// we are doing peer to peer, but for now I'll just
// assume we are a client <-> server model.
#[derive(Component, Encode, Decode, Debug, Clone)]
pub struct NetworkId(u32);

#[derive(Debug, Clone)]
pub struct NetworkIds(u32);

impl NetworkIds {
    pub fn new() -> Self {
        Self(1) // just skipping 1 for the sake of sanity checking
    }

    pub fn create(&self) -> NetworkId {
        let id = NetworkId(self.0);
        self.0 = self
            .0
            .checked_add(1)
            .expect("NetworkId has overflowed u32::MAX.");
        id
    }
}

impl Default for NetworkIds {
    fn default() -> Self {
        Self::new()
    }
}

// Local mapping for clients.
#[derive(Default, Debug, Clone)]
pub struct NetworkMapping {
    entities: HashMap<NetworkId, Entity>,
}

impl NetworkMapping {
    fn new() -> Self {
        Self::default()
    }
}
