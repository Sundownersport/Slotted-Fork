const NAMES: [&str; 4] = ["Grayscale", "DMG Green", "GB Pocket", "GB Light"];

const SCREENS: [&str; 4] = ["grayscale", "measured", "pocket measured", "light"];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct GbPalette(u8);

impl GbPalette {
    pub const COUNT: usize = NAMES.len();
    pub const DEFAULT: GbPalette = GbPalette(1);

    pub fn all() -> impl Iterator<Item = GbPalette> {
        (0..Self::COUNT as u8).map(GbPalette)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn core_name(self) -> &'static str {
        NAMES[self.index()]
    }

    pub fn screen(self) -> &'static str {
        SCREENS[self.index()]
    }

    pub fn label(self) -> &'static str {
        self.core_name()
    }

    pub fn next(self) -> GbPalette {
        GbPalette(((self.index() + 1) % Self::COUNT) as u8)
    }

    pub fn prev(self) -> GbPalette {
        GbPalette(((self.index() + Self::COUNT - 1) % Self::COUNT) as u8)
    }

    pub fn parse(name: &str) -> Option<GbPalette> {
        Self::all().find(|p| p.core_name() == name)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GbColors {
    Auto,
    Palette(GbPalette),
}

impl GbColors {
    pub fn all() -> impl Iterator<Item = GbColors> {
        std::iter::once(GbColors::Auto).chain(GbPalette::all().map(GbColors::Palette))
    }

    pub fn index(self) -> usize {
        match self {
            GbColors::Auto => 0,
            GbColors::Palette(p) => p.index() + 1,
        }
    }

    pub fn next(self) -> GbColors {
        match self {
            GbColors::Auto => GbColors::Palette(GbPalette(0)),
            GbColors::Palette(p) if p.index() + 1 == GbPalette::COUNT => GbColors::Auto,
            GbColors::Palette(p) => GbColors::Palette(p.next()),
        }
    }

    pub fn prev(self) -> GbColors {
        match self {
            GbColors::Auto => GbColors::Palette(GbPalette((GbPalette::COUNT - 1) as u8)),
            GbColors::Palette(p) if p.index() == 0 => GbColors::Auto,
            GbColors::Palette(p) => GbColors::Palette(p.prev()),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            GbColors::Auto => "Auto",
            GbColors::Palette(p) => p.label(),
        }
    }

    pub fn core_name(self) -> &'static str {
        match self {
            GbColors::Auto => "Auto",
            GbColors::Palette(p) => p.core_name(),
        }
    }
}
