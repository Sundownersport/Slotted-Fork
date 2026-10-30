use std::path::{Path, PathBuf};

use slot_retro::{LibretroCore, MockCore, RetroCore};
use slot_store::{Blending, Core, GbColors, Platform};

use crate::root;

const CORE_ENV: &str = "SLOT_CORE";

pub fn dylib_name(core: Core) -> String {
    format!(
        "{}_libretro.{}",
        core.as_str(),
        std::env::consts::DLL_EXTENSION
    )
}

fn candidates(root: &Path, core: Core) -> Vec<PathBuf> {
    if let Some(named) = std::env::var_os(CORE_ENV) {
        return vec![PathBuf::from(named)];
    }
    let name = dylib_name(core);
    let mut paths = vec![root.join("System").join(&name)];
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
    {
        paths.push(dir.join(&name));
        paths.push(dir.join("vendor").join(&name));
    }
    paths.push(Path::new("vendor").join(&name));
    paths
}

pub struct Opened {
    pub core: Box<dyn RetroCore>,
    pub named: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn open_core(
    root: &Path,
    core: Core,
    platform: Platform,
    serial: &str,
    colour: bool,
    blending: Blending,
    palette: Option<GbColors>,
) -> Opened {
    let paths = candidates(root, core);
    match open_named(
        root, core, platform, serial, colour, blending, palette, &paths,
    ) {
        Some(core) => Opened { core, named: true },
        None => {
            report_missing(core, &paths);
            Opened {
                core: Box::new(MockCore::new()),
                named: false,
            }
        }
    }
}

pub fn open_core_for(
    root: &Path,
    core: Core,
    serial: &str,
    colour: bool,
    paths: &[PathBuf],
) -> Box<dyn RetroCore> {
    open_core_for_palette(root, core, Platform::Gba, serial, colour, None, paths)
}

pub fn open_core_for_palette(
    root: &Path,
    core: Core,
    platform: Platform,
    serial: &str,
    colour: bool,
    palette: Option<GbColors>,
    paths: &[PathBuf],
) -> Box<dyn RetroCore> {
    open_named(
        root,
        core,
        platform,
        serial,
        colour,
        Blending::Off,
        palette,
        paths,
    )
    .unwrap_or_else(|| {
        report_missing(core, paths);
        Box::new(MockCore::new())
    })
}

#[allow(clippy::too_many_arguments)]
fn open_named(
    root: &Path,
    core: Core,
    platform: Platform,
    serial: &str,
    colour: bool,
    blending: Blending,
    palette: Option<GbColors>,
    paths: &[PathBuf],
) -> Option<Box<dyn RetroCore>> {
    let bios = root::bios_dir(root);
    let saves = root::saves_dir(root);
    for path in paths {
        if !path.exists() {
            continue;
        }
        match LibretroCore::open_with(path, &bios, &saves) {
            Ok(mut opened) => {
                apply_core_options(&mut opened, platform, serial, colour, palette);
                apply_blending(&mut opened, core, blending);
                eprintln!("slot: core {}", path.display());
                return Some(Box::new(opened));
            }
            Err(e) => eprintln!("slot: {}: {e}", path.display()),
        }
    }
    None
}

pub fn palette_for(
    chosen: GbColors,
    platform: Platform,
    rom: &Path,
    link: Option<u8>,
) -> Option<GbColors> {
    (link.is_none() && platform.hardware() != Platform::Gba && dmg_only(rom)).then_some(chosen)
}

pub fn dmg_only(rom: &Path) -> bool {
    use std::io::Read;
    let mut header = [0u8; 0x150];
    std::fs::File::open(rom)
        .and_then(|mut f| f.read_exact(&mut header))
        .is_ok_and(|_| header[0x143] & 0x80 == 0)
}

fn report_missing(core: Core, paths: &[PathBuf]) {
    eprintln!(
        "slot: no {} core found, running the mock test pattern instead. Looked in: {}",
        core.as_str(),
        paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

pub fn colour_option(
    _which: Core,
    platform: Platform,
    on: bool,
) -> Option<(&'static str, &'static str)> {
    let key = match platform.hardware() {
        Platform::Gba => "armgba_color_correction",
        _ => "armgba_gb_color_correction",
    };
    Some((key, if on { "enabled" } else { "disabled" }))
}

pub fn blend_option(which: Core, blending: Blending) -> (&'static str, &'static str) {
    let value = match blending.on(which) {
        Blending::Off => "disabled",
        Blending::Mix => "half",
        Blending::Ghosting => "enabled",
    };
    ("armgba_lcd_ghosting", value)
}

pub fn apply_blending(core: &mut LibretroCore, which: Core, blending: Blending) {
    let (key, value) = blend_option(which, blending);
    core.set_option(key, value);
}

pub fn gb_screen_options(palette: Option<GbColors>) -> [(&'static str, &'static str); 2] {
    match palette {
        Some(GbColors::Palette(p)) => [
            ("armgba_gb_model", "Game Boy"),
            ("armgba_gb_screen", p.screen()),
        ],
        _ => [
            ("armgba_gb_model", "Game Boy Color"),
            ("armgba_gb_screen", "measured"),
        ],
    }
}

pub fn apply_core_options(
    core: &mut LibretroCore,
    platform: Platform,
    serial: &str,
    colour: bool,
    palette: Option<GbColors>,
) {
    core.set_option("armgba_restart_buttons", "off");
    core.set_option("armgba_link_mode", "separate");
    core.set_option("armgba_link_port", serial);
    for (key, value) in gb_screen_options(palette) {
        core.set_option(key, value);
    }
    if let Some((key, value)) = colour_option(Core::Armgba, platform, colour) {
        core.set_option(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn blending_is_armgbas_lcd_ghosting() {
        for (blending, want) in [
            (Blending::Off, "disabled"),
            (Blending::Mix, "half"),
            (Blending::Ghosting, "enabled"),
        ] {
            assert_eq!(
                blend_option(Core::Armgba, blending),
                ("armgba_lcd_ghosting", want),
                "{blending:?}"
            );
        }
    }

    #[test]
    fn colour_correction_picks_the_consoles_own_option() {
        assert_eq!(
            colour_option(Core::Armgba, Platform::Gba, true),
            Some(("armgba_color_correction", "enabled"))
        );
        assert_eq!(
            colour_option(Core::Armgba, Platform::Gb, false),
            Some(("armgba_gb_color_correction", "disabled"))
        );
    }

    fn lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn candidates_search_armgbas_own_filename_only() {
        let _g = lock();
        std::env::remove_var(CORE_ENV);
        let paths = candidates(Path::new("/root"), Core::Armgba);
        assert!(!paths.is_empty());
        for path in &paths {
            let name = path.file_name().unwrap().to_str().unwrap();
            assert!(
                name.starts_with("armgba_libretro"),
                "{name} is not armGBA's"
            );
        }
    }

    #[test]
    fn candidates_search_the_roots_own_system_directory() {
        let _g = lock();
        std::env::remove_var(CORE_ENV);
        let root = Path::new("/some/content/root");
        assert_eq!(
            candidates(root, Core::Armgba)[0],
            root.join("System").join(dylib_name(Core::Armgba)),
        );
    }

    #[test]
    fn the_env_override_wins() {
        let _g = lock();
        std::env::set_var(CORE_ENV, "/dev/null/named-core");
        let paths = candidates(Path::new("/root"), Core::Armgba);
        std::env::remove_var(CORE_ENV);
        assert_eq!(paths, vec![PathBuf::from("/dev/null/named-core")]);
    }
}
