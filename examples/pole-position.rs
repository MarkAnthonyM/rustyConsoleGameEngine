extern crate rustyConsoleGameEngine;

use rustyConsoleGameEngine::OlcConsoleGameEngine;

use std::thread;

struct PolePole {
    car_position_test: f64,
    car_distance: f64,
    track: Vec<(f64, f64)>,
}

fn main() {
    let mut init = true;

    let mut game_data = PolePole {
        car_position_test: 0.0,
        car_distance: 0.0,
        track: Vec::new(),
    };

    game_data.track.push((0.0, 10.0)); // Short section for start/finish line
    game_data.track.push((0.0, 200.0));
    game_data.track.push((1.0, 200.0));
    game_data.track.push((0.0, 400.0));
    game_data.track.push((-1.0, 100.0));
    game_data.track.push((0.0, 200.0));
    game_data.track.push((-1.0, 200.0));
    game_data.track.push((1.0, 200.0));
    game_data.track.push((0.0, 200.0));
    game_data.track.push((0.2, 500.0));
    game_data.track.push((0.0, 200.0));

    let off_set = 0.0;
    let track_section = 0;

    let closure: Box<dyn FnMut(&mut OlcConsoleGameEngine<PolePole>)> = Box::new(move |data| {
        if data.keys[0x26].held {
            data.game_struct[0].car_distance += 100.0 * data.time_delta;
        }

        if init {
            data.fill(0, 0, data.screen_width as usize, data.screen_height as usize, ' ' as i16, 0);
            init = false;
        } else {
            for y in 0..data.screen_height / 2 {
                for x in 0..data.screen_width {
                    let perspective = y as f64 / (data.screen_height as f64 / 2.0);

                    let middle_point = 0.5;
                    let mut road_width = 0.1 + perspective * 0.8;
                    let clip_width = road_width * 0.15;

                    road_width *= 0.5;

                    let left_grass = (middle_point - road_width - clip_width) * data.screen_width as f64;
                    let left_clip = (middle_point - road_width) * data.screen_width as f64;

                    let right_clip = (middle_point + road_width) * data.screen_width as f64;
                    let right_grass = (middle_point + road_width + clip_width) * data.screen_width as f64;

                    let row = data.screen_height / 2 + y;

                    let grass_calc = 20.0 * f64::powf(1.0 - perspective, 3.0) + data.game_struct[0].car_distance * 0.1;

                    let clip_calc = 40.0 * f64::powf(1.0 - perspective, 2.0) + data.game_struct[0].car_distance;

                    let grass_color;

                    if grass_calc.sin() > 0.0 {
                        grass_color = 0x000A;
                    } else {
                        grass_color = 0x0002;
                    }

                    let clip_color;

                    if clip_calc.sin() > 0.0 {
                        clip_color = 0x0000;
                    } else {
                        clip_color = 0x000F;
                    }

                    if x < left_grass as i16 {
                        data.draw(x as usize, row as usize, 0x2588, grass_color);
                    }

                    if x >= left_grass as i16 && x < left_clip as i16 {
                        data.draw(x as usize, row as usize, 0x2588, clip_color);
                    }

                    if x >= left_clip as i16 && x < right_clip as i16 {
                        data.draw(x as usize, row as usize, 0x2588, 0x0007);
                    }

                    if x >= right_clip as i16 && x < right_grass as i16 {
                        data.draw(x as usize, row as usize, 0x2588, clip_color);
                    }

                    if x >= right_grass as i16 && x < data.screen_width {
                        data.draw(x as usize, row as usize, 0x2588, grass_color)
                    }
                }
            }

            let mut car_position = data.game_struct[0].car_position_test;

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

    let mut demo = OlcConsoleGameEngine::new(closure, game_data);

    demo.consturct_console(150, 50, 6, 12);

    demo.start();
}
