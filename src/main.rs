mod ludo;
use ludo::*;
mod rc_string;


fn main() {
    let mut ludo = Ludo::default();
    ludo.run();
}