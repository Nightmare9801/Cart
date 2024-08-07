pub mod hasher;
pub mod case;
pub mod cart;
pub mod tui;

/// The main function in Rust calls the run_tui function from the tui module.
fn main() {
    tui::run_tui()
}