use crate::RayLibData;
use crate::Coords;
use crate::MyColor;
use crate::MySize;
use crate::AppWindow;
use crate::Area;

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
use std::boxed::Box;

pub struct Renderer {
    pub entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    globals: Rc<RefCell<Globals>>,
    ray_lib_data: Rc<RefCell<RayLibData>>,

    draw_cmds: Box<dyn Fn(&mut RaylibDrawHandle)>,
}


impl Renderer {
    pub fn new(
        globals: Rc<RefCell<Globals>>,
        ray_lib_data: Rc<RefCell<RayLibData>>) -> Renderer {
        ray_lib_data.borrow().rl.borrow_mut().set_target_fps(20);

        let renderer = Renderer {
            entities: HashSet::new(),
            component_types: HashSet::new(),

            globals: globals.clone(),
            ray_lib_data,

            draw_cmds: Renderer::empty_cmds(), // TODO: to remove
        };

        renderer
    }

    pub fn draw_buffered_cmds(&mut self, d: &mut RaylibDrawHandle) {
        let cmds = &mut self.draw_cmds;
        cmds(d);
        self.draw_cmds = Renderer::empty_cmds();
    }

    // TODO: either empty_cmds, or empty_cmds2 should be removed
    fn empty_cmds() -> Box<dyn Fn(&mut RaylibDrawHandle)> {
        Box::new(|_| {})
    }

    fn empty_cmds2(g: Rc<RefCell<Globals>>) -> Box<dyn Fn(Rc<RefCell<Globals>>, &mut RaylibDrawHandle, i32, i32)> {
        Box::new(|g: Rc<RefCell<Globals>>, d: &mut RaylibDrawHandle, _gui_x: i32, _gui_y: i32| {})
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
