use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Coordinator;
use ecs::Entity;
use ecs::System;

use crate::Coords;
use crate::Velocity;

use std::collections::HashSet;

pub struct Borders {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,
}

impl Borders {
    pub fn new() -> Borders {
        Borders {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
            ]),
        }
    }
}

impl System for Borders {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<ComponentType> {
        &self.component_types
    }

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        for e in self.entities.iter() {
            if let Some(v) = cm.get_mut::<Velocity>(e) {
                let mut v = Velocity {vx: v.vx, vy: v.vy};
                if let Some(coords) = cm.get::<Coords>(e) {
                    if coords.x < 0 || coords.x > 450 { // TODO: get boundry from globals or
                                                        // somewhere else
                        v.vx = -v.vx;
                    }
                    if coords.y < 0 || coords.y > 250 { // TODO: get boundry from globals or
                                                        // somewhere else
                        v.vy = -v.vy;
                    }
                    cm.add(*e, v);
                    println!("ticcck");
                }
            }
        }

        Box::new(| _coordinator | {})
    }
}
