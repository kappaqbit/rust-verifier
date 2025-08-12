#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_build;

mod thir_obtainer;

fn main() {
    thir_obtainer::obtain_thir();
}
