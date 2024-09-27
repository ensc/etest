use once_cell::sync::Lazy;

mod base;
mod id;
mod set;
mod builder;
mod manager;
mod notify;
mod lock;

pub use builder::ResourceBuilder;
pub use id::ResourceId;

pub use id::ResourceIdImpl;

use base::Resource;
use set::ResourceSet;
use manager::ResourceManager;
use manager::ResourceEntry;
use notify::ResourceManagerNotify;
use lock::ResourceLockGuard;

/// Internal global object which manages the resouces.
pub static RESOURCES: Lazy<std::sync::RwLock<ResourceManager>> = Lazy::new(Default::default);
