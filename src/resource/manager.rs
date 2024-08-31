use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{trace_resources, Location};

use super::{ Resource, ResourceId, ResourceSet, ResourceLockGuard, ResourceManagerNotify };

pub type ResourceEntry = Arc<RwLock<Resource>>;

#[derive(Default)]
pub struct ResourceManager {
    resources:		HashMap<ResourceId, ResourceEntry>,
    notify:		Arc<ResourceManagerNotify>,
}

impl ResourceManager {
    fn find_or_insert_resource(&mut self, id: &ResourceId) -> ResourceEntry {
        use std::collections::hash_map::Entry as E;

        match self.resources.entry(id.clone()) {
            E::Occupied(e)	=> e.get().clone(),
            E::Vacant(v)	=> v.insert(Resource::new(id).into()).clone(),
        }
    }

    fn try_reserve(&mut self, request: &ResourceSet, owner: &Location) -> Option<ResourceLockGuard> {
        let mut managed = Vec::new();

        trace_resources!("trying to acquire resources for {}", owner);

        // first step: check whether requested resources are available.
        //
        // because they can be reserved only by going through the ResourceManager,
        // they are available when reserving them later
        for req in &request.consumes {
            let entry = self.find_or_insert_resource(req);
            let entry = entry.read().unwrap();

            if entry.owner.is_some() || !entry.users.is_empty() {
                trace_resources!("  entry {:?} already owned by {:?} or used by {:?}",
                                 entry.id, entry.owner, entry.users);
                return None;
            }
        }

        for req in &request.uses {
            let entry = self.find_or_insert_resource(req);
            let entry = entry.read().unwrap();

            if entry.owner.is_some() {
                trace_resources!("  entry {:?} already owned by {:?}", entry.id, entry.owner);
                return None;
            }
        }

        // second step: acquire the resources
        for req in &request.consumes {
            let entry = self.resources.get(req).unwrap();
            managed.push(entry.clone());

            let mut entry = entry.write().unwrap();

            assert!(entry.owner.is_none());

            trace_resources!("  acquired {:?} for ownership", entry.id);
            entry.owner = Some(owner.clone());
        }

        for req in &request.uses {
            let entry = self.resources.get(req).unwrap();
            managed.push(entry.clone());

            let mut entry = entry.write().unwrap();

            assert!(entry.owner.is_none());

            trace_resources!("  acquired {:?}", entry.id);
            entry.users.insert(owner.clone());
        }

        Some(ResourceLockGuard {
            managed:	managed,
            owner:	owner.clone(),
            notify:	self.notify.clone(),
        })
    }

    pub fn reserve(this: &RwLock<Self>, request: ResourceSet, owner: &Location) -> ResourceLockGuard {
        loop {
            // NOTE: do not write this as the match scrutinee; it will hold
            // the lock during wait() else
            let mut mgr = this.write().unwrap();
            let token = mgr.notify.token();
            let resource = mgr.try_reserve(&request, owner);

            drop(mgr);

            match resource {
                Some(g)		=> {
                    trace_resources!("resources aquired for {owner}");
                    break g;
                }
                None		=> {
                    trace_resources!("resource not available yet for {owner}; waiting...");
                    let notify = this.read().unwrap().notify.clone();

                    // do not combine this with above; it will hold the lock
                    // on 'this' else which might block at the beginning of
                    // another loop
                    notify.wait(token);
                }
            }
        }
    }
}
