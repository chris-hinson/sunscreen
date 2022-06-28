use crate::cpu::Cpu;
use cursive::direction::Direction;
use cursive::event::*;
use cursive::theme;
use cursive::theme::BaseColor::*;
use cursive::theme::Color::Dark;
use cursive::theme::ColorType;
use cursive::view::CannotFocus;
use cursive::Printer;
use cursive::Vec2;
use cursive::View;
use std::borrow::Borrow;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Let's define a buffer view, that shows the last lines from a stream.
//NOTE: this was stolen from the cursive logs.rs example, but i made it not async bc i dont like async
pub struct BufferView {
    buffer: Vec<String>,
}

impl BufferView {
    // Creates a new view with the given buffer size
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(size, String::new());
        BufferView { buffer }
    }

    //appends a series of lines to the buffer
    pub fn update(&mut self, v: &mut Vec<String>) {
        self.buffer.append(v);
    }
}

impl View for BufferView {
    //i literally do not know what this function does and im too afraid to ask
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        //self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the latest up to (size) lines of the buffer
        for (i, line) in self.buffer.iter().rev().take(printer.size.y).enumerate() {
            printer.print((0, printer.size.y - 1 - i), line);
        }
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct UltraHexaView {
    //the core data of this view
    data: Vec<u8>,
    //tells us which line should be at the top of the view
    base_line: usize,
    //tells us which index of our data is currently selected
    index: usize,
    //tells us how many lines we are drawing at any one time
    num_lines: usize,
    //vector of indexes of interest (watchpoints) so we can highlight them
    watchpoints: Vec<usize>,
}
#[allow(dead_code)]
impl UltraHexaView {
    //pub fn new(num_lines: usize) -> Self {
    pub fn new() -> Self {
        UltraHexaView {
            data: Vec::new(),
            base_line: 0,
            index: 0,
            num_lines: 0,
            watchpoints: Vec::new(),
        }
    }

    pub fn new_from_iter<'a>(d: impl IntoIterator<Item = &'a u8>) -> Self {
        UltraHexaView {
            data: d.into_iter().copied().collect(),
            base_line: 0,
            index: 0,
            num_lines: 0,
            watchpoints: Vec::new(),
        }
    }
    pub fn new_from_iter_with_watch<'a>(
        d: impl IntoIterator<Item = &'a u8>,
        watchpoints: impl IntoIterator<Item = &'a usize>,
    ) -> Self {
        UltraHexaView {
            data: d.into_iter().copied().collect(),
            base_line: 0,
            index: 0,
            num_lines: 0,
            watchpoints: watchpoints.into_iter().copied().collect(),
        }
    }
    pub fn set_data(&mut self, dat: &mut [u8]) {
        self.data = dat.to_vec();
    }
    pub fn add_watch(&mut self, new_points: &mut Vec<usize>) {
        self.watchpoints.append(new_points);
        self.watchpoints.dedup();
    }

    pub fn update_data<V: Borrow<(usize, u8)>, I: IntoIterator<Item = V>>(&mut self, new_vals: I) {
        for v in new_vals {
            self.data[v.borrow().0] = v.borrow().1;
        }
    }
    pub fn update_index(&mut self, new: (usize, u8)) {
        match self.data.get_mut(new.0) {
            Some(v) => *v = new.1,
            None => {}
        }
    }

    //tries to move cursor down a row, return
    fn go_down(&mut self) -> EventResult {
        //if we're already at the very bottom, do not consume this event so we can go to next view
        if self.index == self.data.len() - 1 {
            return EventResult::Ignored;
        }

        //go down a row or stick at end of data
        if self.data.len() - self.index > 16 {
            self.index += 16;
        } else {
            self.index = self.data.len() - 1;
        }

        //if we are attempting to go off the bottom of our sliding view, move our sliding view
        if ((self.index / 16) as usize) > (self.base_line + self.num_lines - 1) {
            self.base_line += 1;
        }

        //tell cursive that we have consumed this event
        EventResult::Consumed(None)
    }
    fn go_up(&mut self) -> EventResult {
        //if we're already at the very bottom, do not consume this event so we can go to next view
        if self.index == 0 {
            return EventResult::Ignored;
        }

        if self.index >= 16 {
            self.index -= 16;
        } else {
            self.index = 0;
        }

        //if we are attempting to go off the top of our sliding window, move our sliding window
        if ((self.index / 16) as usize) < self.base_line {
            self.base_line = self.base_line.saturating_sub(1);
        }

        EventResult::Consumed(None)
    }
    fn go_right(&mut self) -> EventResult {
        if self.index % 16 == 15 {
            return EventResult::Ignored;
        }

        if self.index < self.data.len() - 1 {
            self.index += 1
        }
        EventResult::Consumed(None)
    }
    fn go_left(&mut self) -> EventResult {
        if self.index % 16 == 0 {
            return EventResult::Ignored;
        }

        self.index = self.index.saturating_sub(1);
        EventResult::Consumed(None)
    }
}

impl View for UltraHexaView {
    //wait until the view gets laid out to tell our struct how many lines we are printing
    //this needs to be a struct field bc we need to know it for our move functions
    fn layout(&mut self, size: Vec2) {
        self.num_lines = size.y;
    }

    fn draw(&self, printer: &Printer) {
        //print num_lines number of lines
        for line in 0..self.num_lines {
            //print the base address of this line
            printer.print(
                (0, line),
                &format!("{:04X}: ", (self.base_line + line) * 16),
            );
            //print 16 bytes of data
            let line_base_index = (self.base_line + line) * 16;
            for offset in 0..16 {
                let val_index = line_base_index + offset;
                match self.data.get(val_index) {
                    Some(v) => {
                        //if this data exists, check if we're at our currently highlighted number
                        if val_index == self.index {
                            printer.with_color(theme::ColorStyle::highlight(), |printer| {
                                printer.print((6 + 3 * offset, line), &format!("{:02X}", v));
                            });
                            //so that we're not Highlighting the space after the number
                            printer.print((6 + 3 * offset + 2, line), " ");
                        //if this data exists and we have a watchpoint on it, print it purple
                        } else if self.watchpoints.contains(&val_index) {
                            printer.with_color(
                                theme::ColorStyle::new(
                                    ColorType::Color(Dark(Red)),
                                    ColorType::Color(Dark(Black)),
                                ),
                                |printer| {
                                    printer.print((6 + 3 * offset, line), &format!("{:02X}", v));
                                },
                            );
                            //so that we're not Highlighting the space after the number
                            printer.print((6 + 3 * offset + 2, line), " ");
                        } else {
                            printer.print((6 + 3 * offset, line), &format!("{:02X} ", v));
                        }
                    }
                    None => {} //just dont do anything if the data doesnt exist lol
                };
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(k) => match k {
                Key::Left => self.go_left(),
                Key::Right => self.go_right(),
                Key::Up => self.go_up(),
                Key::Down => self.go_down(),
                _ => EventResult::Ignored,
            },
            //ignore any events that are not up/down/left/right
            //fuck u mouse users
            _ => EventResult::Ignored,
        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::consumed())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

//TODO: is there any way we can cheapen this to the point that we can call it every frame?
//would need some way to only change some values?
#[allow(non_snake_case)]
pub struct CpuView {
    pub state: Vec<String>,
}

impl CpuView {
    pub fn new(init: &Cpu) -> Self {
        CpuView {
            state: init.fmt_for_tui(),
        }
    }
    pub fn update(&mut self, cpu_state: Vec<String>) {
        self.state = cpu_state;
    }
}

impl View for CpuView {
    fn draw(&self, printer: &Printer) {
        for (i, v) in self.state.iter().enumerate() {
            printer.print((0, i), v);
        }
    }
}
