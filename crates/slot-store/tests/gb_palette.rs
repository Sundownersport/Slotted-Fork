use slot_store::{GbColors, GbPalette, SlotState};

#[test]
fn the_list_is_armgbas_four_game_boy_screens() {
    let names: Vec<&str> = GbPalette::all().map(GbPalette::core_name).collect();
    assert_eq!(names, ["Grayscale", "DMG Green", "GB Pocket", "GB Light"]);
    let screens: Vec<&str> = GbPalette::all().map(GbPalette::screen).collect();
    assert_eq!(screens, ["grayscale", "measured", "pocket measured", "light"]);
}

#[test]
fn next_walks_the_list_and_wraps() {
    assert_eq!(GbPalette::DEFAULT.core_name(), "DMG Green");
    assert_eq!(GbPalette::DEFAULT.next().core_name(), "GB Pocket");
    let last = GbPalette::all().last().unwrap();
    assert_eq!(last.next().core_name(), "Grayscale");
    assert_eq!(GbPalette::parse("not a palette"), None);
}

#[test]
fn prev_walks_back_and_wraps() {
    assert_eq!(GbPalette::DEFAULT.prev().core_name(), "Grayscale");
    let first = GbPalette::all().next().unwrap();
    assert_eq!(first.prev().core_name(), "GB Light");
    for p in GbPalette::all() {
        assert_eq!(p.next().prev(), p);
    }
}

fn named(name: &str) -> GbPalette {
    GbPalette::parse(name).unwrap()
}

#[test]
fn colours_run_auto_then_every_palette_and_wrap() {
    let all: Vec<GbColors> = GbColors::all().collect();
    assert_eq!(all.len(), 5);
    assert_eq!(all[0], GbColors::Auto);
    assert_eq!(all[1], GbColors::Palette(named("Grayscale")));
    assert_eq!(GbColors::Auto.prev(), GbColors::Palette(named("GB Light")));
    assert_eq!(GbColors::Palette(named("GB Light")).next(), GbColors::Auto);
    assert_eq!(GbColors::Auto.next(), GbColors::Palette(named("Grayscale")));
    assert_eq!(GbColors::Auto.label(), "Auto");
    assert_eq!(GbColors::Auto.core_name(), "Auto");
    assert_eq!(GbColors::Palette(named("GB Pocket")).label(), "GB Pocket");
    for (i, c) in all.iter().enumerate() {
        assert_eq!(c.index(), i, "{c:?}");
    }
}

#[test]
fn palettes_off_reads_as_auto_and_auto_keeps_the_palette_underneath() {
    let off = SlotState {
        gb_palettes: false,
        gb_palette: named("DMG Green"),
        ..SlotState::default()
    };
    assert_eq!(off.gb_colors(), GbColors::Auto);

    let mut on = SlotState {
        gb_palettes: true,
        gb_palette: named("GB Light"),
        ..SlotState::default()
    };
    assert_eq!(on.gb_colors(), GbColors::Palette(named("GB Light")));
    on.set_gb_colors(GbColors::Auto);
    assert!(!on.gb_palettes);
    assert_eq!(on.gb_palette, named("GB Light"));
    on.set_gb_colors(GbColors::Palette(named("GB Pocket")));
    assert!(on.gb_palettes);
    assert_eq!(on.gb_palette, named("GB Pocket"));
}
