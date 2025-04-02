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
use raylib::prelude::*;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MouseInput {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    ray_lib_data: Rc<RefCell<RayLibData>>,
    app_win: Rc<RefCell<AppWindow>>,
}

impl MouseInput {
    pub fn new(
        ray_lib_data: Rc<RefCell<RayLibData>>,
        app_win: Rc<RefCell<AppWindow>>) -> MouseInput {
        MouseInput {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
                ComponentType::of::<MouseControlled>(),
            ]),

            ray_lib_data: ray_lib_data, // TODO: rename rl to ray_lib_data or sth
            app_win,
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
        let rl = &self.ray_lib_data.borrow().rl;
        let view_w = self.app_win.borrow().view_area.w;
        let mouse_pos = rl.borrow().get_mouse_position().clone();

        for e in self.entities.iter() {
            cm.add(*e, Coords {
                x: mouse_pos.x.round() as i32,
                y: mouse_pos.y.round() as i32 });
        }

        if (mouse_pos.x as i32) < view_w {
            if rl.borrow().is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let (x, y) = (mouse_pos.x.round() as i32, mouse_pos.y.round() as i32 );
                return Box::new(move | c| {
                    let e = c.entity_take();
                    let coords = Coords { x, y };
                    c.add_component(e, coords);
                    c.add_component(e, MySize { s: 3f32 });
                    c.add_component(e, TTL { ttl: 40 });
                    c.add_component(e, MyColor { c: Color::INDIANRED });
                })
            }
        }

        Box::new(| _ | {})
    }
}
