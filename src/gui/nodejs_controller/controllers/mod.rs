use self::unix_controller::UnixNodejsController;
use self::windows_controller::WindowsNodejsController;
use exception::Exception;


pub trait NodejsController {
    fn is_nodejs_install() -> bool;
    fn install_nodejs() -> Option<Exception>;
    fn start_nodejs() -> bool;
}

pub fn get_unix_controller() {
    UnixNodejsController::new()
}

pub fn get_windows_controller() {
    WindowsNodejsController::new()
}

mod unix_controller;
mod windows_controller;