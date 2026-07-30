use std::error::Error;

use slint::ComponentHandle;

mod ui {
    slint::include_modules!();
}


fn main() -> Result<(), Box<dyn Error>> {
    let window = ui::AppWindow::new()?;

    let window_handle = window.as_weak();
    window.on_request_increase_value(move || {
        if let Some(window_handle) = window_handle.upgrade() {
            window_handle.set_counter(window_handle.get_counter() + 1);
        }
    });

    window.run()?;

    Ok(())
}
