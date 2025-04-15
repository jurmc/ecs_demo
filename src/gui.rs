use crate::AppWindow;
use crate::Area;

use raylib::prelude::*;

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
}

impl GuiState {
    pub fn new() -> GuiState {
        GuiState {
            sim_mode: SimMode::Stopped,
            quit: false,

            entity_scroll_idx: 0,
            entity_selected_idx: 0,
        }
    }
}

pub fn draw_gui(d: &mut RaylibDrawHandle, gui_state: &mut GuiState, entities_list: &String, app_window: &AppWindow) {
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
    if d.gui_button( rrect(gui_x + 120, gui_y + level_y, 100, 30), "Remove") {
        println!("Remove button pressed");
        println!("gonna to remove entity: {}", gui_state.entity_selected_idx);
    }
    level_y += 70;

    d.gui_list_view(
        rrect(gui_x +5, level_y, 100, 200),
        &entities_list,
        &mut gui_state.entity_scroll_idx,
        &mut gui_state.entity_selected_idx);

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
