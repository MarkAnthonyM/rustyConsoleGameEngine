use winapi::shared::minwindef::TRUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{ STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::wincon::{ SetConsoleWindowInfo, SetConsoleScreenBufferSize, SetConsoleActiveScreenBuffer };
use winapi::um::wincontypes::{ SMALL_RECT, COORD };
use winapi::um::winnt::HANDLE;

//This struct is most likely unneeded
struct SmallRect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

//This struct is most likely unneeded
struct Coord {
    x: i16,
    y: i16,
}

struct OlcConsoleGameEngine {
    screen_width: i16,
    screen_height: i16,

    // Test holder types for now. Adjust with correct types
    console_handle: HANDLE,
    console_handle_in: HANDLE,

    mouse_pos_x: u32,
    mouse_pos_y: u32,

    enable_sound: bool,

    app_name: String,
}

impl OlcConsoleGameEngine {
    fn new() -> OlcConsoleGameEngine {
        let output_handle = unsafe{ GetStdHandle(STD_OUTPUT_HANDLE) };
        let input_handle = unsafe{ GetStdHandle(STD_INPUT_HANDLE) };
        let mouse_x = 0;
        let mouse_y = 0;
        let application_name = "default";

        OlcConsoleGameEngine {
            screen_width: 80,
            screen_height: 80,
            console_handle: output_handle,
            console_handle_in: input_handle,
            mouse_pos_x: mouse_x,
            mouse_pos_y: mouse_y,
            enable_sound: true,
            app_name: application_name.to_string(),
        }
    }

    fn _consturct_console(&mut self, width: i16, height: i16, _font_w: u32, _font_h: u32) {
        self.screen_width = width;
        self.screen_height = height;

        //implement smallRect struct
        let rect_window = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 1,
            Bottom: 1,
        };

        // Set window info using winapi. Will be a Result type.
        self.set_console_window_info(rect_window).unwrap();

        let coord = COORD {
            X: self.screen_width,
            Y: self.screen_height,
        };

        // Set the size of screen buffer. Will be a result type.
        self.set_console_screen_buffer_size(self.console_handle, coord).unwrap();

        // Assign screen buffer to console. Will be a result type.
        self.set_console_active_screen_buffer(self.console_handle).unwrap();

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
        let rect_window = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: self.screen_width - 1,
            Bottom: self.screen_height - 1,
        };

        self.set_console_window_info(rect_window).unwrap();

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

    fn set_console_active_screen_buffer(&self, console_handle: HANDLE) -> Result<i32, &'static str> {
        let active_buffer = unsafe { SetConsoleActiveScreenBuffer(console_handle) };

        if active_buffer != 0 {
            return Ok(active_buffer)
        } else {
            return Err("Set console active screen buffer failed")
        }
    }

    fn set_console_screen_buffer_size(&self, console_handle: HANDLE, size: COORD) -> Result<i32, &'static str> {
        let set_size = unsafe { SetConsoleScreenBufferSize(console_handle, size) };

        if set_size != 0 {
            return Ok(set_size)
        } else {
            return Err("Set console screen buffer size function failed")
        }
    }

    fn set_console_window_info(&self, rect_struct: SMALL_RECT) -> Result<i32, &'static str> {
        let window_info = unsafe { SetConsoleWindowInfo(self.console_handle, TRUE, &rect_struct) };

        if window_info != 0 {
            return Ok(window_info)
        } else {
            return Err("Set console window info function failed")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
