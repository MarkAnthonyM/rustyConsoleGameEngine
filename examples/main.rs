extern crate olc_console_engine_oop;

use olc_console_engine_oop::OlcConsoleGameEngine;

fn main() {
    let mut demo = OlcConsoleGameEngine::new();

    demo.consturct_console(130, 60, 8, 16);

    demo.start();
}
