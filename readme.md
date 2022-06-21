im trying to write an nes emulator in rust.
right now i just want to get the cpu core working.

im trying to pass the nestest log, and then i'll worry about ppu,apu, etc.

the only cool thing i really have in mind is using traits to represent cart mappers, so that we dont need to explicity implement a couple hundred mappers. idk ill worry about it more after the cpu works.
