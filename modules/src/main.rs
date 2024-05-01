mod controllers;

use controllers::user_controller;

fn main() {
    user_controller::index();
    user_controller::find();
    user_controller::store();
    user_controller::delete();
    user_controller::update();

    println!("Hello, world!");
}
