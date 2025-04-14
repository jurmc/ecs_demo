use crate::RayLibData;
use crate::AppWindow;
use crate::Coords;
use crate::MouseControlled;
use crate::MyColor;
use crate::MySize;
use crate::TTL;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Entity;
use ecs::System;
use ecs::Coordinator;
use ecs::Globals;
use raylib::prelude::*;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MouseInput {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    globals: Rc<RefCell<Globals>>,
    ray_lib_data: Rc<RefCell<RayLibData>>,
}

impl MouseInput {
    pub fn new(
        globals: Rc<RefCell<Globals>>,
        ray_lib_data: Rc<RefCell<RayLibData>>) -> MouseInput {
        MouseInput {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
                ComponentType::of::<MouseControlled>(),
            ]),

            globals,
            ray_lib_data,
        }
    }
}

impl System for MouseInput {
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
//        let rl = &self.ray_lib_data.borrow().rl;
//        let globals = self.globals.borrow();
//        let view_width = globals.get::<AppWindow>("app_window").unwrap().view_area.w;
//        let mouse_pos = rl.borrow().get_mouse_position().clone();
//
//        for e in self.entities.iter() {
//            cm.add(*e, Coords {
//                x: mouse_pos.x.round() as i32,
//                y: mouse_pos.y.round() as i32 });
//        }
//
//        if (mouse_pos.x as i32) < view_width {
//            if rl.borrow().is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
//                let (x, y) = (mouse_pos.x.round() as i32, mouse_pos.y.round() as i32 );
//                return Box::new(move | c| {
//                    let e = c.entity_take();
//                    let coords = Coords { x, y };
//                    c.add_component(e, coords);
//                    c.add_component(e, MySize { s: 3f32 });
//                    c.add_component(e, TTL { ttl: 40 });
//                    c.add_component(e, MyColor { c: Color::INDIANRED });
//                })
//            }
//        }

        Box::new(| _ | {})
    }
}
