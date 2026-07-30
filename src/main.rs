use std::error::Error;

use slint::ComponentHandle;

mod ui {
    slint::include_modules!();
}


fn main() -> Result<(), Box<dyn Error>> {
    let window = ui::AppWindow::new()?;

    window.run()?;

    Ok(())
}
