use crate::my_views::{BufferView, CpuView, UltraHexaView};
use crate::nes::NES;
use cursive::theme::{BaseColor::*, BorderStyle, Color::*, Palette, Theme};
use cursive::traits::Nameable;
use cursive::view::SizeConstraint;
use cursive::views::{Dialog, DummyView, LinearLayout, ResizedView};
use cursive::{Cursive, CursiveExt};

pub fn setup_tui(system: &mut crate::nes::NES) -> cursive::Cursive {
    //main structs
    let mut cur = Cursive::new();

    /*
        Background => Dark(Blue)
        Shadow => Dark(Black)
        View => Dark(White)
        Primary => Dark(Black)
        Secondary => Dark(Blue)
        Tertiary => Light(White)
        TitlePrimary => Dark(Red)
        TitleSecondary => Dark(Yellow)
        Highlight => Dark(Red)
        HighlightInactive => Dark(Blue)
        HighlightText => Dark(White)
    */

    let mut our_palette = Palette::default();
    our_palette.set_color("Background", Dark(Yellow));
    our_palette.set_color("View", Dark(Black));
    our_palette.set_color("Primary", Dark(White));
    our_palette.set_color("TitlePrimary", Dark(Cyan));

    let our_theme = Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: our_palette,
    };
    cur.set_theme(our_theme);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let log_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        BufferView::new(75).with_name("log"),
    );

    let cpu_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        CpuView::new(&system.cpu).with_name("cpu"),
    );

    let cpu_state = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, cpu_view);

    let ppu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DummyView);
    let apu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Fixed(7), DummyView);

    let ram_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&system.wram.contents).with_name("wram"),
    );
    let rom_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&system.ppu.cart.prg_rom).with_name("rom"),
    );
    /*let chr_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&system.cart.prg_rom).with_name("chr_view"),
    );*/

    //add views to layer and add layer to screen
    let top_level = LinearLayout::horizontal()
        .child(Dialog::around(log_view).title("LOGS"))
        .child(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(Dialog::around(cpu_state).title("CPU"))
                        .child(Dialog::around(ppu_view).title("PPU")),
                )
                .child(Dialog::around(apu_view).title("APU")),
        );

    let bottom_level = LinearLayout::horizontal()
        .child(Dialog::around(ram_view).title("WRAM"))
        .child(Dialog::around(rom_view).title("ROM"));
    //.child(Dialog::around(chr_view).title("CHR"));

    cur.add_layer(
        LinearLayout::vertical()
            .child(top_level)
            .child(bottom_level),
    );
    ///////////////////////////////////////////////////////////////////////////////////////////////

    //add global keybinds

    //TODO: im like 99% sure this is leaking memory, but calling cur.quit() and or our_runner.quit()
    //just doesnt do anything lmfao
    cur.add_global_callback('q', |s| {
        //panic!("panicked out");
        s.quit()
    });

    //add panic key
    cur.add_global_callback('p', |_cur| panic!("lol"));

    cur
}

//runner function
//TODO: make this be able to mutate the NES state and return it to the caller thread
pub fn run(siv: &mut cursive::Cursive, new_logs: &mut Vec<String>, system: &mut NES) {
    //append any logs that we have accumulated since our last call to the debugger
    siv.call_on_name("log", |view: &mut BufferView| view.update(new_logs));

    //cpu view
    siv.call_on_name("cpu", |view: &mut CpuView| {
        view.update(system.cpu.fmt_for_tui())
    });
    //wram
    siv.call_on_name("wram", |view: &mut UltraHexaView| {
        view.set_data(&mut system.wram.contents.to_vec());
    });
    //rom
    siv.call_on_name("rom", |view: &mut UltraHexaView| {
        view.set_data(&mut system.ppu.cart.prg_rom);
    });
    //apu
    //ppu

    siv.run();
}
