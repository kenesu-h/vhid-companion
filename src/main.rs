use vhid_companion::ApplicationController;

fn main() {
    let controller: ApplicationController = ApplicationController::new();
    controller.join();
}
