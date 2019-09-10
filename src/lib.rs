use std::convert::TryInto;

use winapi::ctypes:: { wchar_t };
use winapi::shared::minwindef::{ BOOL, TRUE, FALSE };
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{ STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::wincon::{ GetConsoleScreenBufferInfo, SetCurrentConsoleFontEx, SetConsoleWindowInfo, SetConsoleScreenBufferSize, SetConsoleActiveScreenBuffer, CONSOLE_FONT_INFOEX, CONSOLE_SCREEN_BUFFER_INFO };
use winapi::um::wincontypes::{ SMALL_RECT, COORD };
use winapi::um::wingdi::{ FF_DONTCARE, FW_NORMAL };
use winapi::um::winnt::{ HANDLE, WCHAR };

//Initialize empty struct
trait Empty {
    fn empty() -> Self;
}

impl Empty for COORD {
    fn empty() -> COORD {
        COORD { X: 0, Y: 0 }
    }
}

impl Empty for CONSOLE_FONT_INFOEX {
    fn empty() -> CONSOLE_FONT_INFOEX {
        CONSOLE_FONT_INFOEX {
            cbSize: 0,
            nFont: 0,
            dwFontSize: COORD::empty(),
            FontFamily: 0,
            FontWeight: 0,
            FaceName: [0 as WCHAR; 32],
        }
    }
}

impl Empty for CONSOLE_SCREEN_BUFFER_INFO {
    fn empty() -> CONSOLE_SCREEN_BUFFER_INFO {
        CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD::empty(),
            dwCursorPosition: COORD::empty(),
            wAttributes: 0,
            srWindow: SMALL_RECT::empty(),
            dwMaximumWindowSize: COORD::empty(),
        }
    }
}

impl Empty for SMALL_RECT {
    fn empty() -> SMALL_RECT {
        SMALL_RECT {
            Top: 0,
            Right: 0,
            Bottom: 0,
            Left: 0,
        }
    }
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

    fn _consturct_console(&mut self, width: i16, height: i16, font_w: i16, font_h: i16) {
        // Implement console handle error check

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
        self.set_console_window_info(self.console_handle, TRUE, rect_window).unwrap();

        let coord = COORD {
            X: self.screen_width,
            Y: self.screen_height,
        };

        // Set the size of screen buffer. Will be a result type.
        self.set_console_screen_buffer_size(self.console_handle, coord).unwrap();

        // Assign screen buffer to console. Will be a result type.
        self.set_console_active_screen_buffer(self.console_handle).unwrap();

        // set font size and settings
        let mut font_cfi = CONSOLE_FONT_INFOEX::empty();
        font_cfi.nFont = 0;
        font_cfi.dwFontSize.X = font_w;
        font_cfi.dwFontSize.Y = font_h;
        font_cfi.FontFamily = FF_DONTCARE;
        font_cfi.FontWeight = FW_NORMAL.try_into().unwrap();


        // Set extended information about current console font
        self.set_current_console_font_ex(self.console_handle, FALSE, &mut font_cfi).unwrap();

        //Todo: Implement wcscpy_s() function

        // Todo: Implement console screen buffer struct
        let mut screen_buffer_csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();

        // Retrive information about supplied console handle
        self.get_console_screen_buffer_info(self.console_handle, &mut screen_buffer_csbi).unwrap();

        // Check for valid window size
        self.validate_window_size(&screen_buffer_csbi).unwrap();

        // Set physical console window size
        let rect_window = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: self.screen_width - 1,
            Bottom: self.screen_height - 1,
        };

        self.set_console_window_info(self.console_handle, TRUE, rect_window).unwrap();

        // Todo: Implement flag logic for mouse imput
        // self.set_console_mode().unwrap();

        // Todo: Implement screen buffer logic
        let mut window_buffer: Vec<wchar_t> = vec!['*' as u16; (self.screen_width * self.screen_height).try_into().unwrap()];
        // May not need these pointers to buffer
        // let buffer_ptr = window_buffer.as_ptr();
        // let buff_sec_ptr = window_buffer.as_mut_ptr();

        // Todo: Implement logic to handle Ctrl+C functionality
        // self.set_console_ctrl_handler(handler_routine, bool);
    }

    //Todo: Implement sound
    // fn _enable_sound() {
    //
    // }

    fn get_console_screen_buffer_info(&self, console_handle: HANDLE, buffer_struct: &mut CONSOLE_SCREEN_BUFFER_INFO) -> Result<i32, &'static str> {
        let screen_buffer_info = unsafe { GetConsoleScreenBufferInfo(console_handle, buffer_struct) };

        if screen_buffer_info != 0 {
            return Ok(screen_buffer_info)
        } else {
            return Err("Get console active screen buffer function failed")
        }
    }

    fn set_console_active_screen_buffer(&self, console_handle: HANDLE) -> Result<i32, &'static str> {
        let active_buffer = unsafe { SetConsoleActiveScreenBuffer(console_handle) };

        if active_buffer != 0 {
            return Ok(active_buffer)
        } else {
            return Err("Set console active screen buffer function failed")
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

    fn set_console_window_info(&self, console_handle: HANDLE, absolute: BOOL, rect_struct: SMALL_RECT) -> Result<i32, &'static str> {
        let window_info = unsafe { SetConsoleWindowInfo(console_handle, absolute, &rect_struct) };

        if window_info != 0 {
            return Ok(window_info)
        } else {
            return Err("Set console window info function failed")
        }
    }

    fn set_current_console_font_ex(&self, console_handle: HANDLE, max_window: BOOL, font_struct: &mut CONSOLE_FONT_INFOEX) -> Result<i32, &'static str> {
        let set_font = unsafe { SetCurrentConsoleFontEx(console_handle, max_window, font_struct) };

        if set_font != 0 {
            return Ok(set_font)
        } else {
            return Err("Set current console font function failed")
        }
    }

    fn validate_window_size(&self, buffer_struct: &CONSOLE_SCREEN_BUFFER_INFO) -> Result<&'static str, &'static str> {
        if self.screen_height > buffer_struct.dwMaximumWindowSize.Y {
            return Err("Screen height or Font height is too big")
        } else if self.screen_width > buffer_struct.dwMaximumWindowSize.X {
            return Err("Screen width or Font Width is too big")
        } else {
            Ok("Window size validation successful")
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
