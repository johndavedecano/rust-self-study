#[allow(unused_variables, unused_assignments, unused_mut)]

fn main() {
    clean::perform_clean();
}

mod clean {
    pub fn perform_clean() {
        println!("clearning");
    }
}
