extern crate rustyConsoleGameEngine;

use rustyConsoleGameEngine::OlcConsoleGameEngine;

use std::thread;

struct PolePole {
    best_laptime: f64,
    car_distance: f64,
    car_position: f64,
    car_speed: f64,
    current_laptime: f64,
    curvature: f64,
    track: Vec<(f64, f64)>,
    track_curvature: f64,
    track_distance: f64,
    player_curvature: f64,
}

fn main() {
    let mut init = true;


    let mut game_data = PolePole {
        best_laptime: 0.0,
        car_distance: 0.0,
        car_position: 0.0,
        car_speed: 0.0,
        current_laptime: 0.0,
        curvature: 0.0,
        track: Vec::new(),
        track_curvature: 0.0,
        track_distance: 0.0,
        player_curvature: 0.0,
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

    for track in game_data.track.iter() {
        game_data.track_distance += track.1;
    }

    let closure: Box<dyn FnMut(&mut OlcConsoleGameEngine<PolePole>)> = Box::new(move |data| {
        if data.keys[0x26].held {
            data.game_struct[0].car_speed += 2.0 * data.time_delta;
        } else {
            data.game_struct[0].car_speed -= 1.0 * data.time_delta;
        }

        if data.keys[0x25].held {
            data.game_struct[0].player_curvature -= 0.7 * data.time_delta;
        }

        if data.keys[0x27].held {
            data.game_struct[0].player_curvature += 0.7 * data.time_delta;
        }

        if data.game_struct[0].player_curvature - data.game_struct[0].track_curvature >= 0.8 {
            data.game_struct[0].car_speed -= 5.0 * data.time_delta;
        }

        // Clamp speed
        if data.game_struct[0].car_speed < 0.0 { data.game_struct[0].car_speed = 0.0 };
        if data.game_struct[0].car_speed > 1.0 { data.game_struct[0].car_speed = 1.0 };

        // Move car along track according to car speed
        data.game_struct[0].car_distance += (70.0 * data.game_struct[0].car_speed) * data.time_delta;

        // Get point on track
        let mut off_set = 0.0;
        let mut track_section = 0;

        data.game_struct[0].current_laptime += data.time_delta;

        if data.game_struct[0].car_distance >= data.game_struct[0].track_distance {
            data.game_struct[0].car_distance -= data.game_struct[0].track_distance;
            if data.game_struct[0].best_laptime < data.game_struct[0].current_laptime {
                data.game_struct[0].best_laptime = data.game_struct[0].current_laptime;
            }
            data.game_struct[0].current_laptime = 0.0;
        }

        // Find position on track (could optimise)
        while track_section < data.game_struct[0].track.len() && off_set <= data.game_struct[0].car_distance {
            off_set += data.game_struct[0].track[track_section].1;
            track_section += 1;
        }

        let target_curvature = data.game_struct[0].track[track_section - 1].0;

        let track_curvature_diff = (target_curvature - data.game_struct[0].curvature) * data.time_delta * data.game_struct[0].car_speed;

        data.game_struct[0].curvature += track_curvature_diff;

        data.game_struct[0].track_curvature += data.game_struct[0].curvature * data.time_delta * data.game_struct[0].car_speed;

        if init {
            data.fill(0, 0, data.screen_width as usize, data.screen_height as usize, ' ' as i16, 0);
            for x in 0..data.screen_width as usize {
                data.draw(x, ((data.screen_height / 2) - 1) as usize, ' ' as i16, 0x00F0);
            }
            init = false;
        } else {
            // for y in 1..(data.screen_height / 2) as usize {
            //     for x in 0..data.screen_width as usize {
            //
            //         let pixel = data.text_buffer[y * data.screen_width as usize + x].Attributes;
            //
            //         let element = y * data.screen_width as usize + x;
            //         println!("{}", pixel);
            //         thread::sleep_ms(10);
            //
            //         if pixel == 0 {
            //             data.text_buffer[element - data.screen_width as usize].Attributes = 0;
            //         } else {
            //             // println!("{}", pixel);
            //             // thread::sleep_ms(1);
            //             data.text_buffer[element - data.screen_width as usize].Attributes = pixel - 16;
            //         }
            //     }
            // }

            for y in 0..data.screen_height / 2 {
                for x in 0..data.screen_width {
                    if y >= 1 {
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

                    let middle_point = 0.5 + data.game_struct[0].curvature * f64::powf(1.0 - perspective, 3.0);
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
            data.game_struct[0].car_position = data.game_struct[0].player_curvature - data.game_struct[0].track_curvature;
            let mut car_position = data.game_struct[0].car_position;

            car_position = data.screen_width as f64 / 2.0 + ((data.screen_width as f64 * car_position) / 2.0) - 7.0;

            data.draw_string_alpha(car_position as usize, 40, "   ||####||   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 41, "      ##   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 42, "     ####   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 43, "     ####   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 44, "|||  ####  |||", 0x000F);
            data.draw_string_alpha(car_position as usize, 45, "|||########|||   ", 0x000F);
            data.draw_string_alpha(car_position as usize, 46, "|||  ####  |||   ", 0x000F);

            //Debug information
            data.draw_string(0, 0, format!("Distance: {}", data.game_struct[0].car_distance), 0x000F);
            data.draw_string(0, 1, format!("Target Curve: {}", data.game_struct[0].curvature), 0x000F);
            data.draw_string(0, 2, format!("Player Curve: {}", data.game_struct[0].player_curvature), 0x000F);
            data.draw_string(0, 3, format!("Player Speed: {}", data.game_struct[0].car_speed), 0x000F);
            data.draw_string(0, 4, format!("Track Curve: {}", data.game_struct[0].track_curvature), 0x000F);

            data.draw_string(0, 5, format!("Current LapTime: {:.2}", data.game_struct[0].current_laptime), 0x000F);

            data.draw_string(0, 6, format!("Best LapTime: {:.2}", data.game_struct[0].current_laptime), 0x000F);
        }
    });

    let mut demo = OlcConsoleGameEngine::new(closure, game_data);

    demo.consturct_console(150, 50, 6, 12);

    demo.start();
}
