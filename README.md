# chip-egg
CHIP-8 Interpreter in Rust

# Usage
1. Run chip-egg.exe followed by your .ch8 ROM. Example:
```
$ ./chip-egg.exe rom.ch8
```

# Input
CHIP-8 Keypad  QWERTY Keyboard
(COSMAC VIP)
1 2 3 C        1 2 3 4
4 5 6 D        Q W E R
7 8 9 E        A S D F
A 0 B F        Z X C V

# Builidng from source
1. Git clone this repo:
```
$ git clone https://github.com/Noobeggs/chip-egg
```
2. Run cargo build in release mode (or debug mode if you want to):
```
$ cargo build --release
```

# TODO (means I prolly won't do these anytime soon lul):
- [ ] Audio
- [ ] GUI/TUI
- [ ] Allow Remapping of Keybindings
- [ ] Add SUPER-CHIP Support (Prolly won't do)
- [ ] Add XO-CHIP Support (Prolly won't do)
- [ ] Debugger
- [ ] More performance optimizations (JIT/Cached Interpreter) just to see how much fps I can hit lol
