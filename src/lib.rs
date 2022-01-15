

/// A single instance of the game state of the world at one time.
pub struct Snapshot { }

/// A ring buffer of snapshots to determine state.
pub struct History {
    snapshots: VecDeque<Snapshot>,
}

pub trait Replicate {
    fn quantize<W: Writer>(&self, writer: W);
    fn decode(&mut self, buf: &[u8]);
}

pub trait Network<C: Component> {
    fn replicate(
        mut commands: Commands,
    ) {

    }
}

// Just an index for the client/server.
pub struct ClientId(u32);

// Network ID is just a remote entity id/generation.
pub struct NetworkId {
    from: ClientId,
    entity: Entity,
}

pub struct ClientOwnership {
    own: Vec<Entity>,
    defer: Vec<Entity>,
}

// We own this entity, do not defer to anything sending updates for it.
#[derive(Component, Debug, Clone)]
pub struct Own;

// We are allowed to predict this entity moving/etc. but defer to any updates.
#[derive(Component, Debug, Clone)]
pub struct Predict;

// Don't predict this at all, just defer to server.
#[derive(Component, Debug, Clone)]
pub struct Defer;