use crate::RayLibData;
use crate::Coords;
use crate::MyColor;
use crate::MySize;
use crate::AppWindow;

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
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    globals: Rc<RefCell<Globals>>,
    ray_lib_data: Rc<RefCell<RayLibData>>,

    pub draw_gui_cmds: Box<dyn Fn(Rc<RefCell<Globals>>, &mut RaylibDrawHandle, i32, i32)>,
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

            draw_gui_cmds: Renderer::empty_cmds2(globals.clone()),
            draw_cmds: Renderer::empty_cmds(), // TODO: to remove
        };

        renderer
    }

    pub fn draw_buffered_cmds(&mut self, d: &mut RaylibDrawHandle) {
        let cmds = &mut self.draw_cmds;
        cmds(d);
        self.draw_cmds = Renderer::empty_cmds();
    }

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
        let ray_lib_data = self.ray_lib_data.borrow_mut();

        let mut rl= ray_lib_data.rl.borrow_mut();
        let raylib_thread = ray_lib_data.raylib_thread.borrow();

        if rl.window_should_close() {
            panic!("Exitted..."); // TODO: this condition should rather be somehow signalled to the
                                  // outside world...
        }



        let mut d = rl.begin_drawing(&raylib_thread);
        let g = self.globals.borrow();
        let app_window = g.get::<AppWindow>("app_window").unwrap();
        let view_area = &app_window.view_area;

        d.clear_background(Color::DARKGRAY);

        let (gui_x, gui_y) = (view_area.w, 0);
        self.draw_gui_cmds.as_ref()(self.globals.clone(), &mut d, gui_x, gui_y);

        for e in self.entities.iter() {
            let c = cm.get::<Coords>(&e);
            if let Some(c) = c {
                let c = Coords {x: c.x, y: c.y};
                let size = match cm.get::<MySize>(&e) {
                    Some(size) => size.s,
                    None => 10f32,
                };
                let color = match cm.get::<MyColor>(&e) {
                    Some(color) => color,
                    None => &mut MyColor { c: Color::CYAN},
                };
                d.draw_circle(c.x, c.y, size, color.c);
            }
        }

        self.draw_cmds = Box::new(|h| {
            h.draw_circle(50, 150, 30f32, Color::DEEPPINK);
        });

        Box::new(| _coordinator | {})
    }
}
