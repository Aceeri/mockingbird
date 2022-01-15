

/// A single instance of the game state of the world at one time.
pub struct Snapshot {
    data: Vec<i32>,
}

/// A ring buffer of snapshots to determine state.
pub struct History {
    snapshots: VecDeque<Snapshot>,
}

pub struct Network<T> {

}