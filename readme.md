im trying to write an nes emulator in rust.
right now i just want to get the cpu core working.

im trying to pass the nestest log, and then i'll worry about ppu,apu, etc.

the only cool thing i really have in mind is using traits to represent cart mappers, so that we dont need to explicity implement a couple hundred mappers. idk ill worry about it more after the cpu works.

if im still feeling it after implementing core functionality, I might make it multithreaded and finally learn imgui to make a nice frontend, we'll see.
