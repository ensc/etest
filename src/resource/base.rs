use std::collections::HashSet;
use std::sync::{ Arc, RwLock };

use crate::Location;

use super::{ ResourceId, ResourceEntry };

#[derive(Debug, Eq, PartialEq)]
pub struct Resource {
    pub id:		ResourceId,
    pub(super) owner:	Option<Location>,
    pub(super) users:	HashSet<Location>,
}

impl Resource {
    pub fn new(id: &ResourceId) -> Self {
        Self {
            id:		id.clone(),
            owner:	None,
            users:	HashSet::new(),
        }
    }
}

impl From<Resource> for ResourceEntry {
    fn from(val: Resource) -> Self {
        Arc::new(RwLock::new(val))
    }
}
