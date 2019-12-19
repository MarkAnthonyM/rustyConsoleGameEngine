extern crate rustyConsoleGameEngine;

use rustyConsoleGameEngine::{ OlcConsoleGameEngine, Color };
use rand::prelude::*;
use std::thread;

struct StringStreamer {
    column: usize,
    row: f64,
    speed: f64,
    text: String,
}

impl StringStreamer {
    fn new() -> Self {
        StringStreamer {
            column: 0,
            row: 0.0,
            speed: 0.0,
            text: String::new(),
        }
    }
}

struct Matrix {
    streamer: Vec<StringStreamer>,
}

fn prepare_streamer(streamer: &mut StringStreamer) {
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0, 128);

    // let ran_str: String = (0..10).map(|_| rand::random::<u8>() as char).collect();
    // let char_bytes = ran_char.bytes();

    streamer.column = rng.sample(distr);
    streamer.row = 0.0;
    streamer.speed = (rng.sample(distr) % 40 + 5) as f64;
    streamer.text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
}

fn main() {
    let rgb_palette = vec![
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
    ];

    let mut matrix = Matrix {
        streamer: Vec::new(),
    };

    let max_streamers = 200;

    for n in 0..max_streamers {
        let mut s = StringStreamer::new();
        prepare_streamer (&mut s);

        matrix.streamer.push(s);
    }

    let closure: Box<dyn FnMut(&mut OlcConsoleGameEngine<Matrix>)> = Box::new(move |data| {
        data.fill(0, 0, data.screen_width as usize, data.screen_height as usize, ' ' as i16, 0);

        for i in 0..data.game_struct[0].streamer.len() {
            let formatted_string = format!("{}", data.game_struct[0].streamer[i].text);
            let string_arr = formatted_string.as_bytes();

            data.game_struct[0].streamer[i].row += data.game_struct[0].streamer[i].speed * data.time_delta;

            for z in 0..formatted_string.len() {
                let row = data.game_struct[0].streamer[i].row;
                let off_set_index = ((z as f64 - row) % formatted_string.len() as f64).abs();
                let mut color;

                if data.game_struct[0].streamer[i].speed < 15.0 {
                    color = Color::fg_dark_green;
                } else {
                    color = Color::fg_green;
                }

                if z == 0 {
                    color = Color::fg_white;
                } else if z <= 3 {
                    color = Color::fg_dark_grey;
                }

                data.draw(data.game_struct[0].streamer[i].column, (row - z as f64) as usize, string_arr[off_set_index as usize] as i16, color as i16);
            }

            if data.game_struct[0].streamer[i].row - formatted_string.len() as f64 >= data.screen_height as f64 {
                prepare_streamer(&mut data.game_struct[0].streamer[i]);
            }
        }
    });

    let mut demo = OlcConsoleGameEngine::new(closure, matrix);

    demo.consturct_console(128, 50, 12, 12);

    demo.start();
}
