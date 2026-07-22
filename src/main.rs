use std::error;

use clicker::app;

fn main() -> Result<(), Box<dyn error::Error>> {
    app::run()
}
