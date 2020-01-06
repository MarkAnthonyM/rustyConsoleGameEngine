extern crate rustyConsoleGameEngine;

use rand::prelude::*;
use rustyConsoleGameEngine::{ Color, OlcConsoleGameEngine };
use std::thread;

struct PolePole {
    best_laptime: f64,
    car_distance: f64,
    car_position: f64,
    car_speed: f64,
    current_laptime: f64,
    curvature: f64,
    stars: Vec<Star>,
    track: Vec<(f64, f64)>,
    track_curvature: f64,
    track_distance: f64,
    player_curvature: f64,
}

struct Star {
    column: usize,
    row: f64,
    speed: f64,
    star: String,
}

impl Star {
    fn new() -> Self {
        Self {
            column: 0,
            row: 0.0,
            speed: 0.0,
            star: ".".to_string(),
        }
    }
}

fn prepare_star(star: &mut Star) {
    let mut rng = rand::thread_rng();
    let ran_column = rng.gen_range(0, 150);
    let ran_speed = rng.gen_range(5, 40);

    star.column = ran_column as usize;
    star.row = 0.0;
    star.speed = ran_speed as f64;
}

fn main() {
    let mut init = true;

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

    let mut game_data = PolePole {
        best_laptime: 0.0,
        car_distance: 0.0,
        car_position: 0.0,
        car_speed: 0.0,
        current_laptime: 0.0,
        curvature: 0.0,
        stars: Vec::new(),
        track: Vec::new(),
        track_curvature: 0.0,
        track_distance: 0.0,
        player_curvature: 0.0,
    };

    let max_stars = 100;

    for n in 0..max_stars {
        let mut star = Star::new();

        prepare_star(&mut star);

        game_data.stars.push(star);
    }

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

    for track in game_data.track.iter() {
        game_data.track_distance += track.1;
    }

    let closure: Box<dyn FnMut(&mut OlcConsoleGameEngine<PolePole>)> = Box::new(move |data| {
        if data.keys[0x26].held {
            data.game_struct.car_speed += 2.0 * data.time_delta;
        } else {
            data.game_struct.car_speed -= 1.0 * data.time_delta;
        }

        if data.keys[0x25].held {
            data.game_struct.player_curvature -= 0.7 * data.time_delta;
        }

        if data.keys[0x27].held {
            data.game_struct.player_curvature += 0.7 * data.time_delta;
        }

        if data.game_struct.player_curvature - data.game_struct.track_curvature >= 0.8 {
            data.game_struct.car_speed -= 5.0 * data.time_delta;
        }

        // Clamp speed
        if data.game_struct.car_speed < 0.0 { data.game_struct.car_speed = 0.0 };
        if data.game_struct.car_speed > 1.0 { data.game_struct.car_speed = 1.0 };

        // Move car along track according to car speed
        data.game_struct.car_distance += (70.0 * data.game_struct.car_speed) * data.time_delta;

        // Get point on track
        let mut off_set = 0.0;
        let mut track_section = 0;

        data.game_struct.current_laptime += data.time_delta;

        if data.game_struct.car_distance >= data.game_struct.track_distance {
            data.game_struct.car_distance -= data.game_struct.track_distance;
            if data.game_struct.best_laptime < data.game_struct.current_laptime {
                data.game_struct.best_laptime = data.game_struct.current_laptime;
            }
            data.game_struct.current_laptime = 0.0;
        }

        // Find position on track (could optimise)
        while track_section < data.game_struct.track.len() && off_set <= data.game_struct.car_distance {
            off_set += data.game_struct.track[track_section].1;
            track_section += 1;
        }

        let target_curvature = data.game_struct.track[track_section - 1].0;

        let track_curvature_diff = (target_curvature - data.game_struct.curvature) * data.time_delta * data.game_struct.car_speed;

        data.game_struct.curvature += track_curvature_diff;

        data.game_struct.track_curvature += data.game_struct.curvature * data.time_delta * data.game_struct.car_speed;

        if init {
            data.fill(0, 0, data.screen_width as usize, data.screen_height as usize, ' ' as i16, 0);
            for x in 0..data.screen_width as usize {
                data.draw(x, ((data.screen_height / 2) - 1) as usize, ' ' as i16, 0x00F0);
            }
            data.load_custom_palette();
            init = false;
        } else {
            data.fill(0, 0, data.screen_width as usize, 10, ' ' as i16, 0);

            for i in 0..data.game_struct.stars.len() {
                let formatted_string = format!("{}", data.game_struct.stars[i].star);
                let string_array = formatted_string.as_bytes();

                data.game_struct.stars[i].row += (data.game_struct.stars[i].speed * data.game_struct.car_speed) * data.time_delta;

                for s in 0..string_array.len() {
                    let row = data.game_struct.stars[i].row - s as f64;
                    let column = data.game_struct.stars[i].column;

                    let color;

                    if data.game_struct.stars[i].speed < 8.0 {
                        color = Color::fg_dark_grey;
                    } else {
                        color = Color::fg_white;
                    }

                    data.draw(column, row as usize, string_array[s] as i16, color as i16);
                }

                if data.game_struct.stars[i].row > 9.0 {
                    prepare_star(&mut data.game_struct.stars[i]);
                }
            }

            for y in 0..data.screen_height / 2 {
                for x in 0..data.screen_width {

                    // Draw sunset
                    if y >= 1 && y <= 15 {
                        let row = (data.screen_height / 2) as usize - y as usize;

                        let pixel = data.text_buffer[row * data.screen_width as usize + x as usize].Attributes;

                        let element = row * data.screen_width as usize + x as usize;

                        if pixel == 0 {
                            data.text_buffer[element - data.screen_width as usize].Attributes = 0;
                        } else {
                            data.text_buffer[element - data.screen_width as usize].Attributes = pixel - 16;
                        }
                    }

                    let perspective = y as f64 / (data.screen_height as f64 / 2.0);

                    let middle_point = 0.5 + data.game_struct.curvature * f64::powf(1.0 - perspective, 3.0);
                    let mut road_width = 0.1 + perspective * 0.8;
                    let clip_width = road_width * 0.15;

                    road_width *= 0.5;

                    let left_grass = (middle_point - road_width - clip_width) * data.screen_width as f64;
                    let left_clip = (middle_point - road_width) * data.screen_width as f64;

                    let right_clip = (middle_point + road_width) * data.screen_width as f64;
                    let right_grass = (middle_point + road_width + clip_width) * data.screen_width as f64;

                    let row = data.screen_height / 2 + y;

                    let grass_calc = 20.0 * f64::powf(1.0 - perspective, 3.0) + data.game_struct.car_distance * 0.1;

                    let clip_calc = 40.0 * f64::powf(1.0 - perspective, 2.0) + data.game_struct.car_distance;

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

                    let road_color;

                    if track_section - 1 == 0 {
                        road_color = 0x000F;
                    } else {
                        road_color = 0x0007;
                    }

                    if x < left_grass as i16 {
                        data.draw(x as usize, row as usize, 0x2588, grass_color);
                    }

                    if x >= left_grass as i16 && x < left_clip as i16 {
                        data.draw(x as usize, row as usize, 0x2588, clip_color);
                    }

                    if x >= left_clip as i16 && x < right_clip as i16 {
                        data.draw(x as usize, row as usize, 0x2588, road_color);
                    }

                    if x >= right_clip as i16 && x < right_grass as i16 {
                        data.draw(x as usize, row as usize, 0x2588, clip_color);
                    }

                    if x >= right_grass as i16 && x < data.screen_width {
                        data.draw(x as usize, row as usize, 0x2588, grass_color)
                    }
                }
            }

            // Draw car
            data.game_struct.car_position = data.game_struct.player_curvature - data.game_struct.track_curvature;
            let mut car_position = data.game_struct.car_position;

            car_position = data.screen_width as f64 / 2.0 + ((data.screen_width as f64 * car_position) / 2.0) - 7.0;

            data.draw_string_alpha(car_position as usize, 40, "   ||####||   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 41, "      ##   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 42, "     ####   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 43, "     ####   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 44, "|||  ####  |||", 0x000F);
            data.draw_string_alpha(car_position as usize, 45, "|||########|||   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 46, "|||  ####  |||   ", 0x000F);

            //Debug information
            data.draw_string(0, 0, format!("Distance: {}", data.game_struct.car_distance), 0x000F);
            data.draw_string(0, 1, format!("Target Curve: {}", data.game_struct.curvature), 0x000F);
            data.draw_string(0, 2, format!("Player Curve: {}", data.game_struct.player_curvature), 0x000F);
            data.draw_string(0, 3, format!("Player Speed: {}", data.game_struct.car_speed), 0x000F);
            data.draw_string(0, 4, format!("Track Curve: {}", data.game_struct.track_curvature), 0x000F);

            data.draw_string(0, 5, format!("Current LapTime: {:.2}", data.game_struct.current_laptime), 0x000F);

            data.draw_string(0, 6, format!("Best LapTime: {:.2}", data.game_struct.current_laptime), 0x000F);
        }
    });

    let mut demo = OlcConsoleGameEngine::new(closure, game_data);

    demo.consturct_console(150, 50, 6, 12, rgb_palette);

    demo.start();
}
