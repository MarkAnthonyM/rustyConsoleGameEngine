# rustyConsoleGameEngine

A Rust implementation of the [olcConsoleGameEngine](https://github.com/OneLoneCoder/videos/blob/master/olcConsoleGameEngine.h)

![](https://media.giphy.com/media/XB4MM7zrTuGfURTjlc/200w.webp)
![](https://media.giphy.com/media/TjMcssqMuzcz2rUmnN/giphy.gif)

## Status

rustyConsoleGameEngine is very much a work-in-progress. It most likely will change as I grow as a rust developer.

## Usage

Add to Project Cargo.toml file:
```
[dependencies]
rustyConsoleGameEngine = "0.1"
```

## Example

main.rs:
```Rust
use rustyConsoleGameEngine::olcConsoleGameEngine;

fn main() {
  let closure: Box<dyn FnMut(&mut olcConsoleGameEngine)> = Box::new(move |data| {
    // Implement game logic
  });

  let demo = olcConsoleGameEngine::new(closure);

  demo.consturct_console(100, 50, 6, 12);

  demo.start();
}
```

## Documentation

Coming Soon

## Acknowledgments

* [Javidx9](https://github.com/OneLoneCoder) - The original writer of the olcConsoleGameEngine

* [Peter Atashian](https://github.com/retep998) - Owner/Maintainer of Winapi

* [Rust Lang Forum](https://users.rust-lang.org/)

## License

**[GNU General Public License v3.0](https://github.com/MarkAnthonyM/rustyConsoleGameEngine/blob/master/LICENSE.md)**