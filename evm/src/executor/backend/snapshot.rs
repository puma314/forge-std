use ethers::types::{Address, H256, U256};
use hashbrown::HashMap as Map;
use revm::{AccountInfo, Env, JournaledState};
use serde::{Deserialize, Serialize};

/// A minimal abstraction of a state at a certain point in time
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub accounts: Map<Address, AccountInfo>,
    pub storage: Map<Address, Map<U256, U256>>,
    pub block_hashes: Map<U256, H256>,
}

/// Represents a snapshot taken during evm execution
#[derive(Clone, Debug)]
pub struct BackendSnapshot<T> {
    pub db: T,
    /// The journaled_state state at a specific point
    pub journaled_state: JournaledState,
    /// Contains the env at the time of the snapshot
    pub env: Env,
}

// === impl BackendSnapshot ===

impl<T> BackendSnapshot<T> {
    /// Takes a new snapshot
    pub fn new(db: T, journaled_state: JournaledState, env: Env) -> Self {
        Self { db, journaled_state, env }
    }

    /// Called when this snapshot is reverted.
    ///
    /// Since we want to keep all additional logs that were emitted since the snapshot was taken
    /// we'll merge additional logs into the snapshot's `revm::JournaledState`. Additional logs are
    /// those logs that are missing in the snapshot's journaled_state, since the current
    /// journaled_state includes the same logs, we can simply replace use that See also
    /// `DatabaseExt::revert`
    pub fn merge(&mut self, current: &JournaledState) {
        self.journaled_state.logs = current.logs.clone();
    }
}