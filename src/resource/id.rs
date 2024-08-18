#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ResourceId {
    id:		String,
}

impl ResourceId {
    pub fn new(id: &str) -> Self {
        Self {
            id:	id.to_string()
        }
    }
}

impl From<&str> for ResourceId {
    fn from(val: &str) -> Self {
        ResourceId {
            id:	val.to_string()
        }
    }
}
