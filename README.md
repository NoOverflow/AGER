
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
|CPU MISC 08 (Blargg's)  | ❌ Fail: missing instructions |
|CPU OP R,R 09 (Blargg's)  | ❌ Fail: missing instructions |
|CPU BIT OPS 10 (Blargg's)  | ❌ Fail: missing instructions |
|CPU OP A,(HL) 11 (Blargg's)  | ❌ Fail: missing instructions |
## Sources used:
- http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
- https://gbdev.io/pandocs/
