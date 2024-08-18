use std::collections::HashSet;
use std::sync::RwLock;

use crate::Location;

use super::{ ResourceId, ResourceLockGuard, ResourceManager, ResourceSet };

pub struct ResourceBuilder {
    uses:	HashSet<ResourceId>,
    consumes:	HashSet<ResourceId>,
}

impl ResourceBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            uses:	HashSet::default(),
            consumes:	HashSet::default(),
        }
    }

    pub fn consumes<T: Into<ResourceId>>(mut self, id: T) -> Self {
        self.consumes.insert(id.into());
        self
    }

    pub fn uses<T: Into<ResourceId>>(mut self, id: T) -> Self {
        self.uses.insert(id.into());
        self
    }

    pub fn finish(mut self) -> ResourceSet {
        for r in &self.consumes {
            self.uses.remove(r);
        }

        ResourceSet {
            uses:	self.uses,
            consumes:	self.consumes,
        }
    }

    pub fn reserve(self, manager: &RwLock<ResourceManager>, owner: &Location) -> ResourceLockGuard {
        let set = self.finish();

        ResourceManager::reserve(manager, set, owner)
    }
}
