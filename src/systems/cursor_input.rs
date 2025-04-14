use crate::RayLibData;
use crate::Entity;
use crate::ComponentType;
use crate::Coords;
use crate::CursorControlled;

use ecs::ComponentManager;
use ecs::Coordinator;
use ecs::System;

use std::collections::HashSet;

pub struct CursorInput {
    pub entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,
}

impl CursorInput {
    pub fn new() -> CursorInput {
        CursorInput {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
                ComponentType::of::<CursorControlled>(),
            ]),
        }
    }
}

impl System for CursorInput {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<ComponentType> {
        &self.component_types
    }

    fn apply(&mut self, _cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        Box::new(| _ | {})
    }
}

