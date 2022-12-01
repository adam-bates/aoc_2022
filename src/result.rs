pub use anyhow::Context;

pub type Result<T = (), E = anyhow::Error> = anyhow::Result<T, E>;

