use crate::RayLibData;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Entity;
use ecs::System;
use ecs::Coordinator;
use raylib::prelude::*;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;
use std::boxed::Box;

pub struct Renderer {
    pub entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    draw_cmds: Box<dyn Fn(&mut RaylibDrawHandle)>,
}


impl Renderer {
    pub fn new(
        ray_lib_data: Rc<RefCell<RayLibData>>) -> Renderer {
        ray_lib_data.borrow().rl.borrow_mut().set_target_fps(20);

        let renderer = Renderer {
            entities: HashSet::new(),
            component_types: HashSet::new(),

            draw_cmds: Renderer::empty_cmds(), // TODO: to remove
        };

        renderer
    }

    pub fn draw_buffered_cmds(&mut self, d: &mut RaylibDrawHandle) {
        let cmds = &mut self.draw_cmds;
        cmds(d);
        self.draw_cmds = Renderer::empty_cmds();
    }

    // TODO: probably to be removed
    fn empty_cmds() -> Box<dyn Fn(&mut RaylibDrawHandle)> {
        Box::new(|_| {})
    }
}

impl System for Renderer {
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
        Box::new(| _coordinator | {})
    }
}
