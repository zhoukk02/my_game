use bevy::prelude::*;

use std::collections::HashMap;

use crate::app::Id;

#[derive(Resource)]
pub struct Store<T> {
    map: HashMap<Id, T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: Id, template: T) {
        self.map.insert(id, template);
    }

    pub fn get(&self, id: &Id) -> Option<&T> {
        self.map.get(id)
    }

    pub fn get_mut(&mut self, id: &Id) -> Option<&mut T> {
        self.map.get_mut(id)
    }

    pub fn contains(&self, id: &Id) -> bool {
        self.map.contains_key(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Id, &T)> {
        self.map.iter()
    }
}
