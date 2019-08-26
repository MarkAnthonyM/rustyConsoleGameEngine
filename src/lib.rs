//need to test this struct. may have to use winapi version
struct SmallRect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

struct Coord {
    screen_width: u32,
    screen_height: u32,
}

struct OlcConsoleGameEngine {
    screen_width: u32,
    screen_height: u32,

    // Test holder types for now. Adjust with correct types
    console_handle_out: String,
    console_handle_in: String,

    mouse_pos_x: u32,
    mouse_pos_y: u32,

    enable_sound: bool,

    app_name: String,
}

impl OlcConsoleGameEngine {
    fn new() -> OlcConsoleGameEngine {
        let output_handle = "this is a test";
        let input_handle = "this is a test";
        let mouse_x = 0;
        let mouse_y = 0;
        let application_name = "default";

        OlcConsoleGameEngine {
            screen_width: 80,
            screen_height: 80,
            console_handle_out: output_handle.to_string(),
            console_handle_in: input_handle.to_string(),
            mouse_pos_x: mouse_x,
            mouse_pos_y: mouse_y,
            enable_sound: true,
            app_name: application_name.to_string(),
        }
    }

    fn _consturct_console(&mut self, width: u32, height: u32, _font_w: u32, _font_h: u32) {
        self.screen_width = width;
        self.screen_height = height;

        //implement smallRect struct
        let rect_window = SmallRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        // Set window info using winapi. Will be a Result type.
        self.set_console_window_info().unwrap();

        let coord = Coord {
            screen_width: self.screen_width,
            screen_height: self.screen_height,
        };

        // Set the size of screen buffer. Will be a result type.
        self.set_console_screen_buffer_size().unwrap();

        // Assign screen buffer to console. Will be a result type.
        self.set_console_active_screen_buffer().unwrap();

        // Todo: Implement font struct
        // let some_font_data = SomeFontStruct {
        //     // Font struct logic here
        // };

        // Todo: Implement font console setting function. Make result type.
        // self.set_current_console_font_ex().unwrap();

        // Todo: Implement console screen buffer struct
        // let some_screen_buffer_info = SomeBufferInfoStruct {
        //     // screen buffer info here
        // };

        // Todo: Implement error checking logic for allowed window sizes.
        // self.get_console_screen_buffer_info().unwrap();
        // self.varify_allowed_window_size().unwrap();

        // Todo: Implement logic to set physical window size
        let rect_window = SmallRect {
            left: 0,
            top: 0,
            right: self.screen_width - 1,
            bottom: self.screen_height - 1,
        };

        self.set_console_window_info().unwrap();

        // Todo: Implement flag logic for mouse imput
        // self.set_console_mode().unwrap();

        // Todo: Implement screen buffer logic
        // let mut window_buffer: Vec<ctypes::wchar_t> = vec!['*' as u16; buff_width * buff_height];
        // let buffer_ptr = window_buffer.as_ptr();
        // let buff_sec_ptr = window_buffer.as_mut_ptr();

        // Todo: Implement logic to handle Ctrl+C functionality
        // self.set_console_ctrl_handler(handler_routine, bool);
    }

    fn _enable_sound() {

    }

    fn set_console_active_screen_buffer(&self) -> Result<String, String> {
        unimplemented!()
    }

    fn set_console_screen_buffer_size(&self) -> Result<String, String> {
        unimplemented!()
    }

    fn set_console_window_info(&self) -> Result<String, String> {
        unimplemented!()
    }
}

fn main() {
    let new_console = OlcConsoleGameEngine::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
