use crate::AppWindow;
use crate::Coords;
use crate::Velocity;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Coordinator;
use ecs::Entity;
use ecs::System;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Borders {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    w: i32,
    h: i32,
}

impl Borders {
    pub fn new(w: i32, h: i32) -> Borders {
        Borders {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
            ]),

            w,
            h,
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
                    if coords.x < 0 || coords.x > self.w {
                        v.vx = -v.vx;
                    }
                    if coords.y < 0 || coords.y > self.h {
                        v.vy = -v.vy;
                    }
                    cm.add(*e, v);
                }
            }
        }

        Box::new(| _coordinator | {})
    }
}
