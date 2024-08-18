use std::collections::HashSet;

use super::ResourceId;

pub struct ResourceSet {
    pub(super) uses:		HashSet<ResourceId>,
    pub(super) consumes:	HashSet<ResourceId>,
}
