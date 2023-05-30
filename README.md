# Game-Boy-Emulator
My attempt to create a Gameboy Emulator with Rust

- [Playlist](https://www.youtube.com/playlist?list=PLVxiWMqQvhg_yk4qy2cSC3457wZJga_e5)
- [Reference Repo](https://github.com/rockytriton/LLD_gbemu)
- [Pan Docs](https://gbdev.io/pandocs/About.html)

## Prerequisite

- [SDL2](https://github.com/libsdl-org/SDL)
- [SDL_ttf](https://github.com/libsdl-org/SDL_ttf)

### How to install SDL2 and SDL2_ttf for MACOS

- [Video Tutorial](https://www.youtube.com/watch?v=Li5Xzk0lBgU)

1. Download the dmg files for both the links above
2. Once you mount the dmg, extract the folder with the suffix of `.framework`
3. Move the those folders into `/Library/Frameworks`
4. In you're rust project with in the `Cargo.toml` add:
   - `build = "build.rs"` under `[package]`
   - [dependencies.sdl2] and under that add `version = "*"` and below that add `features = ["use_mac_framework", "ttf"]`
5. Create a new file in the root of the project with add the contents below:
```
fn main() {
    println!("cargo:rustc-link-search=framework=/Library/Frameworks");
}
```