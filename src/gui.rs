use crate::AppWindow;
use crate::Area;
use crate::Coords;
use crate::MyColor;
use crate::Velocity;
use crate::MouseControlled;
use crate::CursorControlled;
use crate::Weight;
use crate::MySize;
use crate::TTL;

use ecs::Coordinator;
use ecs::Entity;
use raylib::prelude::*;

use std::fmt::Write;
use std::collections::BTreeMap;

pub enum SimMode {
    Stopped,
    Started,
    OneStep,
}

pub struct GuiState {
    pub sim_mode: SimMode,
    pub quit: bool,

    // Entities
    pub entity_scroll_idx: i32,
    pub entity_selected_idx: i32,

    pub entity_labels: bool,
}

impl GuiState {
    pub fn new() -> GuiState {
        GuiState {
            sim_mode: SimMode::Stopped,
            quit: false,

            entity_scroll_idx: 0,
            entity_selected_idx: 0,

            entity_labels: false,
        }
    }
}

pub fn apply(d: &mut RaylibDrawHandle, gui_state: &mut GuiState, app_window: &AppWindow, c: &mut Coordinator) {
    let view_area = &app_window.view_area;
    let gui_area = &app_window.gui_area;
    let (gui_x, gui_y) = (view_area.w, 0);

    draw_frames(d, &view_area, &gui_area);

    let mut level_y = gui_y + 5;

    if d.gui_button( rrect(gui_x + 5, gui_y + level_y, 100, 30), "Step") {
        gui_state.sim_mode = SimMode::OneStep
    }
    let play_button_text = match gui_state.sim_mode {
        SimMode::Started => "Stop",
        _ => "Play",
    };
    if d.gui_button( rrect(gui_x + 120, gui_y + level_y, 100, 30), play_button_text) {
        match gui_state.sim_mode {
            SimMode::Started => gui_state.sim_mode = SimMode::Stopped,
            _ => gui_state.sim_mode = SimMode::Started,
        }
    }
    if d.gui_button( rrect(gui_x + 235, gui_y + level_y, 100, 30), "Quit") {
        gui_state.quit = true;
    }
    level_y += 30;

    d.gui_label(
        rrect(gui_x + 5, level_y, 100, 30),
        "Entities - label"
    );
    level_y += 30;

    if d.gui_button( rrect(gui_x + 5, gui_y + level_y, 100, 30), "Add") {
        println!("Add button pressed");
    }

    // TODO: I'd like to have entites in list view presented by id order (so looking for entity is simpler)
    let mut entities_str_list = String::new();
    let mut entities_map = BTreeMap::new();
    let mut idx: usize = 0;
    for e in c.entities_iter() {
        entities_map.insert(idx, *e);
        idx += 1;
    }
    for item in entities_map.iter() {
        write!(entities_str_list, "{}\n", item.1).unwrap();
    }

    if d.gui_button( rrect(gui_x + 120, gui_y + level_y, 100, 30), "Remove") {
        let idx: usize = gui_state.entity_selected_idx as usize;
        if idx < entities_map.len() {
            c.entity_back(*entities_map.get(&(idx as usize)).unwrap());
        }
    }
    level_y += 70;

    d.gui_label(rrect(gui_x + 5, level_y - 120, 100, 200), "Entities list");

    d.gui_list_view(
        rrect(gui_x + 5, level_y, 100, 200),
        &entities_str_list,
        &mut gui_state.entity_scroll_idx,
        &mut gui_state.entity_selected_idx);

    if gui_state.entity_selected_idx != -1 && (gui_state.entity_selected_idx as usize) < entities_map.len() {
        //let e = entities_ids_vec[gui_state.entity_selected_idx as usize];
        let e = entities_map.get(&(gui_state.entity_selected_idx as usize)).unwrap(); //  [gui_state.entity_selected_idx as usize];
        d.gui_label(rrect(gui_x + 140, level_y - 120, 100, 250), &format!("Entity {} components", e));

        let components_str = build_components_str(c, &e);

        d.gui_list_view(
            rrect(gui_x + 135, level_y, 200, 200),
            &components_str,
            &mut -1,
            &mut -1);
    }

    level_y += 220;
    d.gui_check_box(rrect(gui_x + 15, level_y, 20, 20), "Entity labels", &mut gui_state.entity_labels);

}

fn build_components_str(c: &mut Coordinator, e: &Entity) -> String {
    let mut retval = String::new();

    if let Some(coords) = c.get::<Coords>(&e) {
        retval.push_str(&format!("Coords: {}, {}", coords.x, coords.y));
    }

    if let Some(col) = c.get::<MyColor>(&e) {
        retval.push_str("\n");
        retval.push_str(&format!("MyColor: {:?}", col));
    }

    if let Some(vel) = c.get::<Velocity>(&e) {
        retval.push_str("\n");
        retval.push_str(&format!("Velocity: vx: {}, vy: {}", vel.vx, vel.vy));
    }

    if let Some(_) = c.get::<MouseControlled>(&e) {
        retval.push_str("\n");
        retval.push_str("MouseControlled");
    }

    if let Some(_) = c.get::<CursorControlled>(&e) {
        retval.push_str("\n");
        retval.push_str("CursorControlled");
    }

    if let Some(w) = c.get::<Weight>(&e) {
        retval.push_str("\n");
        retval.push_str(&format!("Weight: {}", w.w));
    }

    if let Some(s) = c.get::<MySize>(&e) {
        retval.push_str("\n");
        retval.push_str(&format!("Size: {}", s.s));
    }

    if let Some(ttl) = c.get::<TTL>(&e) {
        retval.push_str("\n");
        retval.push_str(&format!("TTL: {}", ttl.ttl));
    }

    retval
}

fn draw_frames(d: &mut RaylibDrawHandle, view: &Area, gui: &Area) {
        let color = Color::DARKSLATEGRAY;
        let thickness = 5;
        d.draw_rectangle(0, 0, view.w, thickness, color);
        d.draw_rectangle(0, view.h-thickness, view.w, view.h, color);
        d.draw_rectangle(0, 0, thickness, view.h, color);
        d.draw_rectangle(view.w-(thickness/2), 0, thickness+(thickness/2), view.h, color);

        d.draw_rectangle(view.w, 0, gui.w, thickness, color);
        d.draw_rectangle(view.w, gui.h-thickness, gui.w, gui.h, color);
        d.draw_rectangle(view.w+gui.w-thickness, 0, thickness, gui.h, color);
    }
