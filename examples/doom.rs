extern crate rustyConsoleGameEngine;

use rustyConsoleGameEngine::OlcConsoleGameEngine;
use std::convert::TryInto;
use winapi::um::winnt::SHORT;

use rand::random;

fn spread_fire(data: &mut OlcConsoleGameEngine, src: usize) {
    let pixel = data.text_buffer[src].Attributes;

    let ran_num = rand::random::<u16>();

    if pixel == 0 {
        data.text_buffer[src - data.screen_width as usize].Attributes = 0;
    } else {
        data.text_buffer[src - data.screen_width as usize].Attributes = pixel - (ran_num & 16);
    }
}

fn main() {
    let mut init = true;

    let test_clousre: Box<dyn FnMut(&mut OlcConsoleGameEngine)> = Box::new(move |data| {
        if init {
            for x in 0..data.screen_width as usize {
                for y in 0..data.screen_height as usize {
                    if y == (data.screen_height - 1) as usize {
                        data.draw(x, y, ' ' as SHORT, 0x00F0);
                        init = false;
                    } else {
                        data.draw(x, y, ' ' as SHORT, 0x0000);
                        init = false;
                    }
                }
            }
        } else {
            for x in 0..data.screen_width as usize {
                for y in 1..data.screen_height as usize {
                    let src = y * data.screen_width as usize + x;
                    spread_fire(data, src);
                }
            }
        }
    });

    let mut demo = OlcConsoleGameEngine::new(test_clousre);

    demo.consturct_console(100, 50, 6, 12);

    demo.start();
}
