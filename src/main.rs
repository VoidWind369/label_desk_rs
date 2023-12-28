#![windows_subsystem = "windows"]

use slint::LogicalPosition;

use label::LabelPath;

mod label;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    let ui_handle = ui.as_weak();
    ui.on_close_window(move || {
        let ui = ui_handle.unwrap();
        ui.hide().unwrap();
    });

    let ui_handle = ui.as_weak();
    ui.on_move_window(move |offset_x, offset_y| {
        let ui = ui_handle.unwrap();
        //获取窗口的物理坐标
        let logical_pos = ui.window().position().to_logical(ui.window().scale_factor());
        //窗口坐标添加上偏移量，设置为新的位置
        ui.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });

    let ui_handle = ui.as_weak();
    ui.on_label_res(move || {
        let ui = ui_handle.unwrap();
        let label_value = label::new(LabelPath {
            our: ui.get_label_check_our().to_string(),
            you: ui.get_label_check_you().to_string(),
            fight: ui.get_label_check_bool(),
        });
        ui.set_label_check_value(label_value.0.into());
        ui.set_label_check_color(label_value.1.into());
        ui.set_window_hight(255.0)
    });

    ui.run()
}
