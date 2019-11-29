extern crate olc_console_engine_oop;

use olc_console_engine_oop::OlcConsoleGameEngine;
use std::convert::TryInto;
use winapi::um::winnt::SHORT;

use rand::random;

fn main() {
    let test_clousre: Box<dyn FnMut(&mut OlcConsoleGameEngine)> = Box::new(|data| {
        for x in 0..data.screen_width as usize {
            for y in 0..data.screen_height as usize {
                let ran_num = random::<u16>();
                let conv = ran_num % 16;
                data.draw(x, y, '@' as SHORT, conv.try_into().unwrap());
            }
        }
    });

    let mut demo = OlcConsoleGameEngine::new(test_clousre);

    demo.consturct_console(130, 60, 8, 16);

    demo.start();
}
