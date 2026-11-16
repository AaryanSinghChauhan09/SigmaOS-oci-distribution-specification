// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-oci-distribution-specification: oci distribution specification component model.
//!
//! Real implementation: specifications, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specification {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OciDistributionSpecificationError {
    EmptySpecificationName,
    DuplicateSpecification(String),
    EmptyEventSubject,
}

impl fmt::Display for OciDistributionSpecificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySpecificationName => write!(f, "specification name must not be empty"),
            Self::DuplicateSpecification(id) => write!(f, "duplicate specification {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for OciDistributionSpecificationError {}

pub struct OciDistributionSpecification {
    pub name: String,
    specifications: BTreeMap<String, Specification>,
    events: BTreeMap<String, Event>,
}

impl OciDistributionSpecification {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            specifications: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_specification(&mut self, item: Specification) -> Result<(), OciDistributionSpecificationError> {
        if item.name.trim().is_empty() {
            return Err(OciDistributionSpecificationError::EmptySpecificationName);
        }
        let id = item.id.clone();
        if self.specifications.contains_key(&id) {
            return Err(OciDistributionSpecificationError::DuplicateSpecification(id.clone()));
        }
        self.specifications.insert(id, item);
        Ok(())
    }

    pub fn specification_count(&self) -> usize {
        self.specifications.len()
    }

    pub fn get_specification(&self, id: &str) -> Option<&Specification> {
        self.specifications.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), OciDistributionSpecificationError> {
        if event.subject.trim().is_empty() {
            return Err(OciDistributionSpecificationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_specification(id: &str, name: &str) -> Specification {
        Specification {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = OciDistributionSpecification::new("OciDistributionSpecification");
        assert_eq!(m.name, "OciDistributionSpecification");
        assert_eq!(m.specification_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_specification() {
        let mut m = OciDistributionSpecification::new("OciDistributionSpecification");
        m.add_specification(sample_specification("a1", "Alpha")).unwrap();
        assert_eq!(m.specification_count(), 1);
        assert!(m.get_specification("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = OciDistributionSpecification::new("OciDistributionSpecification");
        assert!(m.add_specification(sample_specification("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = OciDistributionSpecification::new("OciDistributionSpecification");
        m.add_specification(sample_specification("x", "One")).unwrap();
        assert!(m.add_specification(sample_specification("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = OciDistributionSpecification::new("OciDistributionSpecification");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
