extern crate rustyConsoleGameEngine;

use rustyConsoleGameEngine::OlcConsoleGameEngine;
use std::convert::TryInto;
use winapi::um::winnt::SHORT;

use rand::random;

struct Doom;

fn spread_fire(data: &mut OlcConsoleGameEngine<Doom>, src: usize) {
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

    let game_data = Doom {};

    let rgb_palette = Some(vec![
        (7,7,7),
        (71,15,7),
        (103,31,7),
        (143,39,7),
        (175,63,7),
        (199,71,7),
        (233,87,7),
        (215,95,7),
        (207,111,15),
        (207,127,15),
        (199,135,23),
        (199,151,31),
        (191,159,31),
        (191,175,47),
        (183,183,47),
        (207,207,111),
    ]);

    let test_clousre: Box<dyn FnMut(&mut OlcConsoleGameEngine<Doom>)> = Box::new(move |data| {
        if init {
            data.load_custom_palette();

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

    let mut demo = OlcConsoleGameEngine::new(test_clousre, game_data);

    demo.consturct_console(100, 50, 6, 12, rgb_palette);

    demo.start();
}
