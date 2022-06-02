use crate::my_views::BufferView;
use crate::my_views::UltraHexaView;
use cursive::theme::{BaseColor::*, BorderStyle, Color::*, Palette, Theme};
use cursive::traits::Nameable;
use cursive::view::SizeConstraint;
use cursive::views::{Dialog, DummyView, LinearLayout, ResizedView};
use cursive::{Cursive, CursiveRunner};

pub struct AppState {
    pub is_running: bool,
}
pub fn setup_tui(system: &mut crate::nes::NES) -> cursive::CursiveRunner<cursive::CursiveRunnable> {
    //main structs
    let mut cur = cursive::default();
    let mut our_runner = cur.into_runner();

    //app state and our cpu
    let app_state = AppState { is_running: true };

    //our TUI needs an app state so we can update our cpu accordingly
    our_runner.set_user_data(app_state);
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
    our_runner.set_theme(our_theme);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let log_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        BufferView::new(75).with_name("log"),
    );

    let cpu_state = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DummyView);

    let ppu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DummyView);
    let apu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Fixed(7), DummyView);

    let ram_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&system.cpu.WRAM).with_name("ram_view"),
    );
    let rom_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&system.cart.prg_rom).with_name("rom_view"),
    );
    let chr_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&system.cart.prg_rom).with_name("chr_view"),
    );

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
        .child(Dialog::around(ram_view).title("VRAM"))
        .child(Dialog::around(rom_view).title("ROM"));
    //.child(Dialog::around(chr_view).title("CHR"));

    our_runner.add_layer(
        LinearLayout::vertical()
            .child(top_level)
            .child(bottom_level),
    );
    ///////////////////////////////////////////////////////////////////////////////////////////////

    //add global keybinds

    //TODO: im like 99% sure this is leaking memory, but calling cur.quit() and or our_runner.quit()
    //just doesnt do anything lmfao
    our_runner.add_global_callback('q', |_cur| {
        panic!("panicked out");
    });

    //global callback to toggle appState's running variable
    our_runner.add_global_callback('w', |cur| {
        cur.with_user_data(|data: &mut AppState| {
            data.is_running = !data.is_running;
        });
    });

    return our_runner;
}
