mod base;
mod id;
mod set;
mod builder;
mod manager;
mod notify;
mod lock;

pub use builder::ResourceBuilder;
pub use id::ResourceId;

use base::Resource;
use set::ResourceSet;
use manager::ResourceManager;
use manager::ResourceEntry;
use notify::ResourceManagerNotify;
use lock::ResourceLockGuard;

lazy_static::lazy_static!(
    /// Internal global object which manages the resouces.
    pub static ref RESOURCES: std::sync::RwLock<ResourceManager> = Default::default();
);
