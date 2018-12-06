use self::controllers::NodejsController;
use exception::{handle_exception, Exception};


pub fn start_server() -> bool {
    let controller = get_controller();
    
    if !controller.is_nodejs_install() {
        let install_error = controller.install_nodejs();
        if !install_error.is_none() {
            handle_exception(install_error);
        } else {
            controller.start_nodejs();
        }
    } else {
        controller.start_nodejs();
    }
    
}

fn get_controller() -> NodejsController {
    if cfg!(unix) {
        controllers::get_unix_controller()    
    } else if cfg!(windows) {
        controllers::get_windows_controller()
    } else {
        handle_exception(Exception::Fatal("Unsupported platform"));
    }
}

mod controllers;
