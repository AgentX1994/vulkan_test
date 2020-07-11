use std::error;

use vulkan_test::controller::Controller;

fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
    Controller::start()
}
