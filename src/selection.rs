//! Selection management for Dioxus Three

use crate::input::EntityId;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Single,
    Multiple,
    Toggle,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Single
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionStyle {
    pub outline: bool,
    pub outline_color: String,
    pub outline_width: f32,
    pub highlight: bool,
    pub highlight_color: String,
    pub highlight_opacity: f32,
    pub show_gizmo: bool,
}

impl Default for SelectionStyle {
    fn default() -> Self {
        Self {
            outline: true,
            outline_color: "#DEC647".to_string(),
            outline_width: 2.0,
            highlight: true,
            highlight_color: "#DEC647".to_string(),
            highlight_opacity: 0.3,
            show_gizmo: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    selected: HashSet<EntityId>,
    mode: SelectionMode,
    last_selected: Option<EntityId>,
}

impl Selection {
    pub fn new() -> Self {
        Self::empty()
    }
    
    pub fn empty() -> Self {
        Self {
            selected: HashSet::new(),
            mode: SelectionMode::Single,
            last_selected: None,
        }
    }
    
    pub fn with_mode(mode: SelectionMode) -> Self {
        Self {
            selected: HashSet::new(),
            mode,
            last_selected: None,
        }
    }
    
    pub fn is_selected(&self, entity: EntityId) -> bool {
        self.selected.contains(&entity)
    }
    
    pub fn select(&mut self, entity: EntityId) {
        match self.mode {
            SelectionMode::Single => {
                self.selected.clear();
                self.selected.insert(entity);
            }
            _ => {
                self.selected.insert(entity);
            }
        }
        self.last_selected = Some(entity);
    }
    
    pub fn toggle(&mut self, entity: EntityId) {
        if self.selected.contains(&entity) {
            self.selected.remove(&entity);
        } else {
            if self.mode == SelectionMode::Single {
                self.selected.clear();
            }
            self.selected.insert(entity);
        }
        self.last_selected = Some(entity);
    }
    
    pub fn deselect(&mut self, entity: EntityId) {
        self.selected.remove(&entity);
        if self.last_selected == Some(entity) {
            self.last_selected = self.selected.iter().copied().last();
        }
    }
    
    pub fn clear(&mut self) {
        self.selected.clear();
        self.last_selected = None;
    }
    
    pub fn count(&self) -> usize {
        self.selected.len()
    }
    
    pub fn has_selection(&self) -> bool {
        !self.selected.is_empty()
    }
    
    pub fn primary(&self) -> Option<EntityId> {
        self.last_selected
    }
    
    pub fn iter(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.selected.iter().copied()
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}
