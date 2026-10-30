use slot_store::{
    core_for, core_for_platform, read_selected_cores, write_selected_core, Core, Platform,
};
use tempfile::tempdir;

fn root_with(ini: Option<&str>) -> tempfile::TempDir {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("Config")).unwrap();
    if let Some(text) = ini {
        std::fs::write(d.path().join("Config/selected_core.txt"), text).unwrap();
    }
    d
}

#[test]
fn absent_file_means_everything_defaults() {
    let d = root_with(None);
    assert!(read_selected_cores(d.path()).is_empty());
    assert_eq!(core_for(d.path(), "Emerald"), Core::Armgba);
}

#[test]
fn a_listed_stem_gets_its_core() {
    let d = root_with(Some("Pokemon - Emerald Version (USA, Europe) = armgba\n"));
    assert_eq!(
        read_selected_cores(d.path()).get("Pokemon - Emerald Version (USA, Europe)"),
        Some(&Core::Armgba)
    );
}

#[test]
fn malformed_lines_are_ignored_rather_than_fatal() {
    let d = root_with(Some(concat!(
        "\n",
        "# a comment\n",
        "; another comment\n",
        "[cores]\n",
        "no equals sign here\n",
        "Emerald = notacore\n",
        "  Spaced Out   =   armgba  \n",
        "= armgba\n",
        "Trailing =\n",
    )));
    let map = read_selected_cores(d.path());
    assert_eq!(
        map.get("Spaced Out"),
        Some(&Core::Armgba),
        "whitespace not trimmed"
    );
    assert_eq!(
        map.get("Emerald"),
        None,
        "an unknown core name must not be stored"
    );
    assert_eq!(core_for(d.path(), "Emerald"), Core::Armgba);
    assert_eq!(map.len(), 1, "only the one good line should survive");
}

#[test]
fn a_retired_core_name_falls_back_to_armgba() {
    let d = root_with(Some("Emerald = gpsp\nTetris = mgba\n"));
    assert!(read_selected_cores(d.path()).is_empty());
    assert_eq!(core_for(d.path(), "Emerald"), Core::Armgba);
    assert_eq!(
        core_for_platform(d.path(), "Tetris", Platform::Gb),
        Core::Armgba
    );
}

#[test]
fn core_names_round_trip() {
    for c in Core::ALL {
        assert_eq!(Core::parse(c.as_str()), Some(c));
    }
    assert_eq!(
        Core::parse("ARMGBA"),
        Some(Core::Armgba),
        "case is not the user's problem"
    );
    assert_eq!(Core::parse("nonsense"), None);
    assert_eq!(Core::default(), Core::Armgba);
}

#[test]
fn writing_a_core_replaces_that_carts_line_and_leaves_the_rest_alone() {
    let d = root_with(Some(concat!(
        "# my notes\n",
        "\n",
        "Emerald = mgba\n",
        "Metroid Fusion = armgba\n",
    )));
    write_selected_core(d.path(), "Emerald", Core::Armgba).unwrap();

    let text = std::fs::read_to_string(d.path().join("Config/selected_core.txt")).unwrap();
    assert!(
        text.contains("# my notes"),
        "a hand-written comment was destroyed"
    );
    assert!(
        text.contains("Metroid Fusion = armgba"),
        "another cart's entry was lost"
    );
    assert!(text.contains("Emerald = armgba"));
    assert_eq!(
        text.matches("Emerald").count(),
        1,
        "the old line was left behind"
    );
}

#[test]
fn every_platform_runs_on_armgba() {
    let d = tempfile::tempdir().unwrap();
    for (stem, platform) in [
        ("Tetris", Platform::Gb),
        ("Tetris Chromatic", Platform::Gbc),
        ("Emerald", Platform::Gba),
    ] {
        assert!(Core::Armgba.runs(platform));
        assert_eq!(core_for_platform(d.path(), stem, platform), Core::Armgba);
    }
}

#[test]
fn the_picker_board_has_one_socket() {
    assert_eq!(Core::ALL.len(), 1);
}
