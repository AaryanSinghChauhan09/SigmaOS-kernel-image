// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-kernel-image: kernel image component model.
//!
//! Real implementation: images, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
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
pub enum KernelImageError {
    EmptyImageName,
    DuplicateImage(String),
    EmptyEventSubject,
}

impl fmt::Display for KernelImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyImageName => write!(f, "image name must not be empty"),
            Self::DuplicateImage(id) => write!(f, "duplicate image {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KernelImageError {}

pub struct KernelImage {
    pub name: String,
    images: BTreeMap<String, Image>,
    events: BTreeMap<String, Event>,
}

impl KernelImage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            images: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_image(&mut self, item: Image) -> Result<(), KernelImageError> {
        if item.name.trim().is_empty() {
            return Err(KernelImageError::EmptyImageName);
        }
        let id = item.id.clone();
        if self.images.contains_key(&id) {
            return Err(KernelImageError::DuplicateImage(id.clone()));
        }
        self.images.insert(id, item);
        Ok(())
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }

    pub fn get_image(&self, id: &str) -> Option<&Image> {
        self.images.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KernelImageError> {
        if event.subject.trim().is_empty() {
            return Err(KernelImageError::EmptyEventSubject);
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

    fn sample_image(id: &str, name: &str) -> Image {
        Image {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = KernelImage::new("KernelImage");
        assert_eq!(m.name, "KernelImage");
        assert_eq!(m.image_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_image() {
        let mut m = KernelImage::new("KernelImage");
        m.add_image(sample_image("a1", "Alpha")).unwrap();
        assert_eq!(m.image_count(), 1);
        assert!(m.get_image("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = KernelImage::new("KernelImage");
        assert!(m.add_image(sample_image("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = KernelImage::new("KernelImage");
        m.add_image(sample_image("x", "One")).unwrap();
        assert!(m.add_image(sample_image("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = KernelImage::new("KernelImage");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
