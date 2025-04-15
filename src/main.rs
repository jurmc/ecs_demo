pub mod systems;
pub mod gui;


use gui::SimMode;
use systems::CursorInput;
use systems::Reaper;
use systems::IntegrateVelocity;
use systems::Gravity;
use systems::Renderer;
use systems::MouseInput;
use systems::Borders;

use ecs::Entity;
use ecs::Coordinator;
use ecs::ComponentType;
use raylib::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Write;

pub struct Area {
    w: i32,
    h: i32
}

pub struct AppWindow {
    view_area: Area,
    gui_area: Area,
}

pub struct RayLibData {
    rl: Rc<RefCell<RaylibHandle>>, // TODO: if RayLibData is already wrapped in Rc<RefCell> do we
                                   // need rl andraylib_thread to be wrapped?
    raylib_thread: Rc<RefCell<RaylibThread>>,
}

impl RayLibData {
    pub fn new(app_window: &AppWindow) -> RayLibData {
        let window_width = app_window.view_area.w + app_window.gui_area.w;
        let window_height = i32::max(app_window.view_area.h, app_window.gui_area.h);

        let (rl, raylib_thread) = raylib::init()
            .size(window_width, window_height)
            .title("ECS demo")
            .build();

        RayLibData {
            rl: Rc::new(RefCell::new(rl)).clone(),
            raylib_thread: Rc::new(RefCell::new(raylib_thread)).clone(),
        }
    }
}

#[derive(Debug)]
pub struct Coords {
    x: i32,
    y: i32,
}

pub struct MySize {
    s: f32,
}

pub struct MyColor {
    c: Color,
}

struct Velocity {
    vx: f64,
    vy: f64,
}

struct TTL {
    ttl: i32,
}

struct Weight {
    w: i32,
}

struct MouseControlled {}
struct CursorControlled {}

fn main() {
    let (width, height) = (640, 480);
    let app_window = AppWindow {
        view_area: Area {w: width, h: height},
        gui_area: Area {w: width, h: height},
    };
    let rl_data = RayLibData::new(&app_window);

    let rl_data = Rc::new(RefCell::new(rl_data));

    let renderer_sys = Renderer::new(rl_data.clone());
    let renderer_sys = Rc::new(RefCell::new(renderer_sys));
    let mouse_input_sys = MouseInput::new(rl_data.clone());
    let mouse_input_sys = Rc::new(RefCell::new(mouse_input_sys));
    let cursor_input_sys = CursorInput::new();
    let cursor_input_sys = Rc::new(RefCell::new(cursor_input_sys));

    let borders = Borders::new(width, height);

    let mut c = Coordinator::new();

    let mouse = c.entity_take();
    let cursor = c.entity_take();
    let e0 = c.entity_take();
    let e1 = c.entity_take();
    let e2 = c.entity_take();
    let e3 = c.entity_take();

    c.register_system(renderer_sys.clone()); // TODO: this block of registered systems should
                                      // also work if move after block of registered component
                                      // types, and adding components to coordinato
    c.register_system(mouse_input_sys.clone());
    c.register_system(cursor_input_sys.clone());
    c.register_system(Rc::new(RefCell::new(IntegrateVelocity::new())));
    c.register_system(Rc::new(RefCell::new(Gravity::new())));
    c.register_system(Rc::new(RefCell::new(Reaper::new())));
    c.register_system(Rc::new(RefCell::new(borders)));

    c.register_component::<Coords>();
    c.register_component::<MyColor>();
    c.register_component::<Velocity>();
    c.register_component::<MouseControlled>();
    c.register_component::<CursorControlled>();
    c.register_component::<Weight>();
    c.register_component::<MySize>();
    c.register_component::<TTL>();

    c.add_component(mouse, Coords {x: 70, y: 90});
    c.add_component(mouse, MyColor {c: Color::ORANGE});
    c.add_component(mouse, MouseControlled{});
    c.add_component(mouse, MySize { s: 0f32 });

    c.add_component(cursor, Coords { x: 30, y: 130 });
    c.add_component(cursor, MyColor {c: Color::INDIGO});
    c.add_component(cursor, CursorControlled{});

    c.add_component(e0, Coords { x: 230, y: 130 });
    c.add_component(e0, MyColor {c: Color::INDIGO});
    c.add_component(e0, TTL { ttl: 40 });

    c.add_component(e1, Coords{x: 20, y: 30});
    c.add_component(e1, MyColor {c: Color::GRAY});
    c.add_component(e1, Velocity{vx: 5f64, vy: 0f64});
    c.add_component(e1, Weight { w: 1 });

    c.add_component(e2, Coords{x: 20, y: 60});
    c.add_component(e2, Velocity{vx: 1f64, vy: 2f64});
    c.add_component(e2, Weight { w: 1 });

    c.add_component(e3, Coords{x: 500, y: 400});
    c.add_component(e3, Velocity{vx: -2f64, vy: 0f64});
    c.add_component(e3, Weight { w: -1 });

    let mut gui_state = gui::GuiState::new();

    loop {
        let mut entities_list = String::new();
        for e in c.entities_iter() {
            write!(entities_list, "{}\n", e).unwrap();
        }

        let ray_lib_data = rl_data.borrow_mut();
        let mut rl= ray_lib_data.rl.borrow_mut();
        let raylib_thread = ray_lib_data.raylib_thread.borrow();
        {

            if rl.window_should_close() || gui_state.quit {
                break;
            }

            let mut d = rl.begin_drawing(&raylib_thread);
            d.clear_background(Color::DARKGRAY);

            let view_area = &app_window.view_area;
            let gui_area = &app_window.gui_area;

            let (gui_x, gui_y) = (view_area.w, 0);
            gui::draw_gui(&mut d, &mut gui_state, &entities_list, &app_window);

            for e in renderer_sys.borrow().entities.iter() {
                let coords = c.get::<Coords>(&e);
                if let Some(coords) = coords {
                    let coords = Coords {x: coords.x, y: coords.y};
                    let size = match c.get::<MySize>(&e) {
                        Some(size) => size.s,
                        None => 10f32,
                    };
                    let color = match c.get::<MyColor>(&e) {
                        Some(color) => color,
                        None => &mut MyColor { c: Color::CYAN},
                    };
                    d.draw_circle(coords.x, coords.y, size, color.c);
                }
            }
        }

        // CursorInput
        {
            let (mut inc_x, mut inc_y) = (0, 0);

            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                inc_x += 1;
            }
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                inc_x -= 1;
            }
            if rl.is_key_down(KeyboardKey::KEY_UP) {
                inc_y -= 1;
            }
            if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                inc_y +=1;
            }

            for e in cursor_input_sys.borrow().entities.iter() {
                let coords = c.get_mut::<Coords>(e).unwrap();
                coords.x = coords.x+ inc_x;
                coords.y = coords.y+ inc_y;
            }
        }

        match gui_state.sim_mode {
            SimMode::Started => c.apply_all(),
            SimMode::OneStep => {
                c.apply_all();
                gui_state.sim_mode = SimMode::Stopped;
            },
            _ => (),
        }
    }
}

