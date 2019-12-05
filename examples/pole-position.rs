extern crate rustyConsoleGameEngine;

use rustyConsoleGameEngine::OlcConsoleGameEngine;

use std::thread;

fn main() {
    let mut init = true;

    let closure: Box<dyn FnMut(&mut OlcConsoleGameEngine)> = Box::new(move |data| {
        if init {
            data.fill(0, 0, data.screen_width as usize, data.screen_height as usize, ' ' as i16, 0);
            init = false;
        } else {
            for y in 0..data.screen_height / 2 {
                for x in 0..data.screen_width {
                    let middle_point = 0.5;
                    let mut road_width = 0.6;
                    let clip_width = road_width * 0.15;

                    road_width *= 0.5;

                    let left_grass = (middle_point - road_width - clip_width) * data.screen_width as f64;
                    let left_clip = (middle_point - road_width) * data.screen_width as f64;

                    let right_clip = (middle_point + road_width) * data.screen_width as f64;
                    let right_grass = (middle_point + road_width + clip_width) * data.screen_width as f64;

                    let row = data.screen_height / 2 + y;

                    if x < left_grass as i16 {
                        data.draw(x as usize, row as usize, 0x2588, 0x000D);
                    }

                    if x >= left_grass as i16 && x < left_clip as i16 {
                        data.draw(x as usize, row as usize, 0x2588, 0x000B);
                    }

                    if x >= left_clip as i16 && x < right_clip as i16 {
                        data.draw(x as usize, row as usize, 0x2588, 0x0007);
                    }

                    if x >= right_clip as i16 && x < right_grass as i16 {
                        data.draw(x as usize, row as usize, 0x2588, 0x000B);
                    }

                    if x >= right_grass as i16 && x < data.screen_width {
                        data.draw(x as usize, row as usize, 0x2588, 0x000D)
                    }
                }
            }

            let mut car_position = 0.0;

            car_position = data.screen_width as f64 / 2.0 + ((data.screen_width as f64 * car_position) / 2.0) - 7.0;

            data.draw_string_alpha(car_position as usize, 40, "   ||####||   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 41, "      ##   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 42, "     ####   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 43, "     ####   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 44, "|||  ####  |||", 0x000F);
            data.draw_string_alpha(car_position as usize, 45, "|||########|||   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 46, "|||  ####  |||   ", 0x000F);
        }
    });

    let mut demo = OlcConsoleGameEngine::new(closure);

    demo.consturct_console(150, 50, 6, 12);

    demo.start();
}
