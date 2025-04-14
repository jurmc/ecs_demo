use crate::AppWindow;
use crate::Coords;
use crate::Velocity;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Coordinator;
use ecs::Entity;
use ecs::System;
use ecs::Globals;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Borders {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    globals: Rc<RefCell<Globals>>,
}

impl Borders {
    pub fn new(g: Rc<RefCell<Globals>>) -> Borders {
        Borders {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
            ]),

            globals: g,
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
//        for e in self.entities.iter() {
//            if let Some(v) = cm.get_mut::<Velocity>(e) {
//                let mut v = Velocity {vx: v.vx, vy: v.vy};
//                if let Some(coords) = cm.get::<Coords>(e) {
//
//                    let globals = self.globals.borrow();
//                    let w = globals.get::<AppWindow>("app_window").unwrap().view_area.w;
//                    let h = globals.get::<AppWindow>("app_window").unwrap().view_area.h;
//
//                    if coords.x < 0 || coords.x > w {
//                        v.vx = -v.vx;
//                    }
//                    if coords.y < 0 || coords.y > h {
//                        v.vy = -v.vy;
//                    }
//                    cm.add(*e, v);
//                }
//            }
//        }

        Box::new(| _coordinator | {})
    }
}
