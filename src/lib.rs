//! SigmaOS-archlinux-norton: Norton security config model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for Norton security config.
pub struct NortonScan {
    pub id: String,
    pub name: String,
    pub scan_type: String,
    pub enabled: bool,
}

/// Secondary domain entity for Norton security config.
pub struct NortonThreat {
    pub id: String,
    pub threat_name: String,
    pub description: String,
}

/// Errors for the Norton security config domain.
#[derive(Debug, PartialEq)]
pub enum NortonError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for NortonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for NortonError {}

/// Registry and manager for Norton security config.
pub struct NortonManager {
    pub name: String,
    items: BTreeMap<String, NortonScan>,
    configs: BTreeMap<String, NortonThreat>,
}

impl NortonManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: NortonScan) -> Result<(), NortonError> {
        if item.name.is_empty() {
            return Err(NortonError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(NortonError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&NortonScan> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: NortonThreat) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), NortonError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| NortonError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_NortonScan(id: &str, name: &str) -> NortonScan {
        NortonScan {
            id: id.to_string(),
            name: name.to_string(),
            scan_type: "archlinux-norton-default".to_string(),
            enabled: true,
        }
    }

    fn make_NortonThreat(id: &str, val: &str) -> NortonThreat {
        NortonThreat {
            id: id.to_string(),
            threat_name: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = NortonManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = NortonManager::new("test");
        let item = make_NortonScan("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = NortonManager::new("test");
        let item = make_NortonScan("item1", "");
        assert_eq!(mgr.add_item(item), Err(NortonError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = NortonManager::new("test");
        let item1 = make_NortonScan("item1", "First");
        let item2 = make_NortonScan("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(NortonError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = NortonManager::new("test");
        let cfg = make_NortonThreat("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = NortonManager::new("test");
        let item = make_NortonScan("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = NortonManager::new("test");
        let item = make_NortonScan("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(NortonError::NotFound("none".to_string())));
    }
}
