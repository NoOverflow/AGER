
# AGER
A Gameboy emulator coded in Rust, serves as a first project to learn Rust.

The goal is to implement a Gameboy emulator that replicates a real machine as closely as can be (especially in the typical gameboy quirks: OAM trash write, HALT skip instructions...)

## Features planned
- Debugger with full control flow capabilities (Step, Back)
- Memory editor and live viewer
- Custom shaders for rendering (OpenGL)
- Mappable controls
- Controller support
- Cheats support
- CPU Speed control
- Custom color palette

## Tests passing
|Test name|Status  |
|--|--|
|CPU Special 01 (Blargg's)  | ✅ Pass |
|CPU Interrupts 02 (Blargg's)  | ❌ Fail: EI|
|CPU OP SP, HL 03 (Blargg's)  | ✅ Pass |
|CPU OP R, IMM 04 (Blargg's)  | ✅ Pass |
|CPU OP RP 05 (Blargg's)  | ✅ Pass |
|CPU LD R,R 06 (Blargg's)  | ✅ Pass |
|CPU JR,JP,CALL,RET,RST 07 (Blargg's)  | ✅ Pass |
|CPU MISC 08 (Blargg's)  | ✅ Pass |
|CPU OP R,R 09 (Blargg's)  | ✅ Pass |
|CPU BIT OPS 10 (Blargg's)  | ✅ Pass |
|CPU OP A,(HL) 11 (Blargg's)  | ✅ Pass |
## Sources used:
- http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
- https://gbdev.io/pandocs/

pkg-config
libglib2.0-dev
libcairo2-dev
libgraphene-1.0-dev
libpango1.0-dev
libgdk-pixbuf-2.0-dev
libgtk-4-dev
libgtk-3-dev
