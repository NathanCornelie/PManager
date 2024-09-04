

pub use slint::*;
use pmanager::ui;
slint::include_modules!();

fn main()  {
    
   
    
    let app = ui::AppWindow::new().unwrap();
    
    ui::main::init_applications(&app);
    

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });
    

}
