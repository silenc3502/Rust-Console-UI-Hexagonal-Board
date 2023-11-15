use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoardId {
    id: Uuid,
}

impl BoardId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn with_id(id: &str) -> Self {
        Self {
            id: Uuid::parse_str(id).expect("Invalid UUID format"),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn nil() -> Self {
        Self { id: Uuid::nil() }
    }
}

impl fmt::Display for BoardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id_generation() {
        let id = BoardId::new();
        assert_ne!(id.id, Uuid::nil());
    }

    #[test]
    fn test_id_with_value() {
        let id = BoardId::with_id("550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(id.id, Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    }

    #[test]
    fn test_id_equality() {
        let id1 = BoardId::new();
        let id2 = BoardId::new();
        assert_ne!(id1.id, id2.id);
    }

    #[test]
    fn test_dynamic_uuid_creation() {
        let dynamic_id = BoardId::new();
        let static_id = BoardId::with_id(dynamic_id.get_id().to_string().as_str());
        assert_eq!(dynamic_id.id, static_id.id);
    }
}
