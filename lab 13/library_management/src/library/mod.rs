pub mod books; // Declare books submodule
pub mod media; // Declare media submodule

// Define the LibraryItem trait
pub trait LibraryItem {
    fn title(&self) -> &str;
    fn check_out(&mut self);
    fn check_in(&mut self);
    fn is_available(&self) -> bool;
}