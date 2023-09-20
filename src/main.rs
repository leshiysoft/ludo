mod sdl2;
mod vulkan;
mod rc_string;
mod ludo;

use ludo::*;


fn main() {
    let mut ludo = Ludo::default();
    ludo.run();
}