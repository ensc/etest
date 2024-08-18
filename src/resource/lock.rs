use std::sync::Arc;

use crate::Location;
use crate::trace_resources;

use super::{ ResourceEntry, ResourceManagerNotify };

pub struct ResourceLockGuard {
    pub(super) managed:	Vec<ResourceEntry>,
    pub(super) owner:	Location,
    pub(super) notify:	Arc<ResourceManagerNotify>,
}

impl std::ops::Drop for ResourceLockGuard {
    fn drop(&mut self) {
        trace_resources!("dropping {:?}", self.owner);
        self.release();
    }
}

impl ResourceLockGuard {
    fn release(&mut self) {
        let mut changed = false;

        for m in &self.managed {
            let mut entry = m.write().unwrap();

            if Some(&self.owner) == entry.owner.as_ref() {
                trace_resources!("  releasing owned {:?}", entry.id);
                entry.owner = None;
                changed = true;
            }

            changed |= entry.users.remove(&self.owner);

            trace_resources!("  entry {:?} used by {:?}", entry.id, entry.users);
        }

        self.managed.clear();

        if changed {
            self.notify.notify();
        }
    }
}
