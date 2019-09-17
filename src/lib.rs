use std::convert::TryInto;
use std::mem::{ size_of, zeroed };
use std::time:: { Duration, Instant };

use rand::prelude::*;

use winapi::ctypes:: { wchar_t };
use winapi::shared::minwindef::{ BOOL, TRUE, FALSE };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{ lstrcpyW, STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::wincon::{ GetConsoleScreenBufferInfo, SetCurrentConsoleFontEx, SetConsoleWindowInfo, SetConsoleScreenBufferSize, SetConsoleActiveScreenBuffer, SetConsoleTitleW, WriteConsoleOutputW, CONSOLE_FONT_INFOEX, CONSOLE_SCREEN_BUFFER_INFO, PCONSOLE_FONT_INFOEX, PCONSOLE_SCREEN_BUFFER_INFO };
use winapi::um::wincontypes::{ CHAR_INFO, CHAR_INFO_Char, COORD, PSMALL_RECT, SMALL_RECT };
use winapi::um::wingdi::{ FF_DONTCARE, FW_NORMAL };
use winapi::um::winnt::{ HANDLE, WCHAR, SHORT, LPCWSTR, LPWSTR };
use winapi::um::winuser::wsprintfW;

use widestring::U16CString;

//Initialize empty struct
trait Empty {
    fn empty() -> Self;
}

impl Empty for CHAR_INFO {
    fn empty() -> CHAR_INFO {
        CHAR_INFO {
            Char: CHAR_INFO_Char::empty(),
            Attributes: 0,
        }
    }
}

impl Empty for CHAR_INFO_Char {
    fn empty() -> CHAR_INFO_Char {
        let mut char_struct_char: CHAR_INFO_Char = unsafe { zeroed() };

        char_struct_char
    }
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

pub struct OlcConsoleGameEngine {
    app_name: String,

    console_handle: HANDLE,
    console_handle_in: HANDLE,

    enable_sound: bool,

    loop_state: bool,

    mouse_pos_x: u32,
    mouse_pos_y: u32,

    rect_window: SMALL_RECT,

    screen_width: i16,
    screen_height: i16,

    text_buffer: Vec<CHAR_INFO>,
}

impl OlcConsoleGameEngine {
    pub fn new() -> OlcConsoleGameEngine {
        let application_name = "default";
        let loop_state = true;
        let mouse_x = 0;
        let mouse_y = 0;
        let output_handle = unsafe{ GetStdHandle(STD_OUTPUT_HANDLE) };
        let input_handle = unsafe{ GetStdHandle(STD_INPUT_HANDLE) };
        let rect_window = SMALL_RECT::empty();
        let window_buffer: Vec<CHAR_INFO> = Vec::new();

        OlcConsoleGameEngine {
            app_name: application_name.to_string(),
            console_handle: output_handle,
            console_handle_in: input_handle,
            enable_sound: true,
            loop_state: loop_state,
            mouse_pos_x: mouse_x,
            mouse_pos_y: mouse_y,
            rect_window: rect_window,
            screen_width: 80,
            screen_height: 80,
            text_buffer: window_buffer,
        }
    }

    pub fn consturct_console(&mut self, width: i16, height: i16, font_w: i16, font_h: i16) {
        // Check for valid handle
        if self.console_handle == INVALID_HANDLE_VALUE {
            println!("failed to get valid console handle");
            return
        }

        self.screen_width = width;
        self.screen_height = height;

        //Set initial rect_window field
        self.rect_window = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 1,
            Bottom: 1,
        };

        // Set window info using winapi. Will be a Result type.
        self.set_console_window_info(self.console_handle, TRUE, &self.rect_window).unwrap();

        let coord = COORD {
            X: self.screen_width,
            Y: self.screen_height,
        };

        // Set the size of screen buffer
        self.set_console_screen_buffer_size(self.console_handle, coord).unwrap();

        // Assign screen buffer to console
        self.set_console_active_screen_buffer(self.console_handle).unwrap();

        // set font size and settings
        let mut font_cfi = CONSOLE_FONT_INFOEX::empty();
        font_cfi.cbSize = size_of::<CONSOLE_FONT_INFOEX>().try_into().unwrap();
        font_cfi.nFont = 0;
        font_cfi.dwFontSize.X = font_w;
        font_cfi.dwFontSize.Y = font_h;
        font_cfi.FontFamily = FF_DONTCARE;
        font_cfi.FontWeight = FW_NORMAL.try_into().unwrap();

        // Set FaceName field for CONSOLE_FONT_INFOEX struct
        let face_name = format!("Consolas");
        let face_str = U16CString::from_str(face_name).unwrap();
        let face_ptr = face_str.as_ptr();
        let face_field_ptr = font_cfi.FaceName.as_mut_ptr();

        self.set_face_name(face_field_ptr, face_ptr);

        // Set extended information about current console font
        self.set_current_console_font_ex(self.console_handle, FALSE, &mut font_cfi).unwrap();

        // Todo: Implement console screen buffer struct
        let mut screen_buffer_csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();

        // Retrive information about supplied console handle
        self.get_console_screen_buffer_info(self.console_handle, &mut screen_buffer_csbi).unwrap();

        // Check for valid window size
        self.validate_window_size(&screen_buffer_csbi).unwrap();

        // Set physical console window size
        self.rect_window = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: self.screen_width - 1,
            Bottom: self.screen_height - 1,
        };

        self.set_console_window_info(self.console_handle, TRUE, &self.rect_window).unwrap();

        // Todo: Implement flag logic for mouse imput
        // self.set_console_mode().unwrap();

        // Todo: Implement screen buffer logic
        // let mut window_buffer: Vec<wchar_t> = vec!['*' as u16; (self.screen_width * self.screen_height).try_into().unwrap()];
        self.text_buffer = vec![CHAR_INFO::empty(); (self.screen_width * self.screen_height).try_into().unwrap()];
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

    fn get_console_screen_buffer_info(&self, console_handle: HANDLE, buffer_struct: PCONSOLE_SCREEN_BUFFER_INFO) -> Result<i32, &'static str> {
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

    fn set_console_title(&self, console_title: LPCWSTR) -> Result<i32, &'static str> {
        let title_string = unsafe { SetConsoleTitleW(console_title) };

        if title_string != 0 {
            return Ok(title_string)
        } else {
            return Err("Set console title function failed")
        }
    }

    fn set_console_window_info(&self, console_handle: HANDLE, absolute: BOOL, rect_struct: *const SMALL_RECT) -> Result<i32, &'static str> {
        let window_info = unsafe { SetConsoleWindowInfo(console_handle, absolute, rect_struct) };

        if window_info != 0 {
            return Ok(window_info)
        } else {
            return Err("Set console window info function failed")
        }
    }

    fn set_current_console_font_ex(&self, console_handle: HANDLE, max_window: BOOL, font_struct: PCONSOLE_FONT_INFOEX) -> Result<i32, &'static str> {
        let set_font = unsafe { SetCurrentConsoleFontEx(console_handle, max_window, font_struct) };

        if set_font != 0 {
            return Ok(set_font)
        } else {
            return Err("Set current console font function failed")
        }
    }

    //not sure this is right
    fn set_face_name(&self, string_1: LPWSTR, string_2: LPCWSTR) {
        unsafe { lstrcpyW(string_1, string_2) };
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

    fn write_console_output(&self, console_handle: HANDLE, buffer: *const CHAR_INFO, buffer_size: COORD, buffer_coord: COORD, write_region: PSMALL_RECT) -> Result<i32, &'static str> {
        let buffer_output = unsafe { WriteConsoleOutputW(console_handle, buffer, buffer_size, buffer_coord, write_region) };

        if buffer_output != 0 {
            return Ok(buffer_output)
        } else {
            return Err("Write console output function failed")
        }
    }

    fn draw(&mut self, x: i16, y: i16, c: SHORT, col: SHORT) {
        if x >= 0 && x < self.screen_width && y >= 0 && y < self.screen_height {
            unsafe {
                let mut chr: CHAR_INFO_Char = CHAR_INFO_Char::empty();
                *chr.UnicodeChar_mut() = c.try_into().unwrap();

                self.text_buffer[y as usize * self.screen_width as usize + x as usize].Char = chr;
                self.text_buffer[y as usize * self.screen_width as usize + x as usize].Attributes = col.try_into().unwrap();
            }
        }
    }

    pub fn game_thread(&mut self) {
        // Validate successful on_user_create function call
        self.on_user_create();

        // Todo: Implement sound system enable check

        // Window Title buffer
        let mut s: [wchar_t; 256] = [0; 256];
        let s_ptr = s.as_mut_ptr();

        // Console title information
        let w_char = format!("OneLoneCoder.com - Console Game Engine - {}", self.app_name);
        let w_string = U16CString::from_str(w_char).unwrap();
        let w_ptr = w_string.as_ptr();

        // Time deltas for smooth fps
        let mut tp_1 = Instant::now();
        let mut tp_2 = Instant::now();

        // Main game loop
        while self.loop_state {
            while self.loop_state {
                // Time delta calulations for smooth frame speed
                tp_2 = Instant::now();
                let mut elapsed_time = tp_2.duration_since(tp_1);
                let in_nano = elapsed_time.as_micros() as f64 / 100_000.0;
                tp_1 = tp_2;

                // Todo: Implement input handle logic

                // Todo: Implement user update function
                self.on_user_update(elapsed_time);

                // Update title and push frame to buffer
                unsafe {
                    let mut rect = self.rect_window;
                    let rect_ptr = &mut rect;
                    wsprintfW(s_ptr, w_ptr);

                    self.set_console_title(s.as_ptr()).unwrap();

                    self.write_console_output(self.console_handle, self.text_buffer.as_ptr(), COORD {X: self.screen_width, Y: self.screen_height}, COORD { X:0, Y:0 }, rect_ptr).unwrap();
                }
            }
        }

        // Todo: Implement free resources functions
    }

    fn on_user_create(&self) -> bool {
        true
    }

    fn on_user_update(&mut self, time_delta: Duration) -> bool {
        // Possibly more efficent rand use
        // let mut v = vec![1, 2, 3];
        //
        // for x in v.iter_mut() {
        //     *x = rand::random();
        // }

        for x in 0..self.screen_width {
            for y in 0..self.screen_height {
                let ran_num = rand::random::<u16>();
                let conv = ran_num % 16;
                self.draw(x, y, '#' as SHORT, conv.try_into().unwrap());
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
