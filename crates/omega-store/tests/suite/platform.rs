use omega_store::Platform;

#[test]
fn every_platform_has_a_directory() {
    let names: Vec<&str> = Platform::ALL.iter().map(|p| p.dir_name()).collect();
    assert_eq!(
        names,
        vec!["NES", "SNES", "GB", "GBC", "GBA", "MD", "SEGACD", "SMS", "GG", "PS1", "PCE", "PCECD", "NGPC"]
    );
}

#[test]
fn every_platform_is_a_shelf_of_its_own() {
    assert_eq!(
        Platform::ALL,
        [
            Platform::Nes,
            Platform::Snes,
            Platform::Gb,
            Platform::Gbc,
            Platform::Gba,
            Platform::Md,
            Platform::SegaCd,
            Platform::Sms,
            Platform::Gg,
            Platform::Ps1,
            Platform::Pce,
            Platform::PceCd,
            Platform::Ngpc
        ]
    );
    for (i, p) in Platform::ALL.iter().enumerate() {
        assert!(
            !Platform::ALL[..i].contains(p),
            "{p:?} is named twice in the ring, so two shelves would hold one platform"
        );
    }
}

#[test]
fn each_platform_takes_only_its_own_extensions() {
    assert!(Platform::Gba.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(!Platform::Gba.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gbc")));
    assert!(!Platform::Gb.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(Platform::Gba.accepts(std::path::Path::new("Shrek.GBA")));
    assert!(Platform::Pce.accepts(std::path::Path::new("Bonk's Adventure.pce")));
    assert!(!Platform::Pce.accepts(std::path::Path::new("Bonk's Adventure.sgx")));
    assert!(!Platform::Snes.accepts(std::path::Path::new("Bonk's Adventure.pce")));
    assert!(Platform::Nes.accepts(std::path::Path::new("Super Mario Bros. (World).nes")));
    assert!(!Platform::Nes.accepts(std::path::Path::new("Zelda no Densetsu.fds")));
}

#[test]
fn a_pc_engine_cd_game_is_found_by_its_cue_and_never_by_its_bin() {
    let d = tempfile::tempdir().unwrap();
    let dir = d.path().join("Games/PCECD/Ys Book I & II");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("Ys Book I & II.cue"), "FILE \"Ys Book I & II.bin\" BINARY\n").unwrap();
    std::fs::write(dir.join("Ys Book I & II.bin"), [0u8; 16]).unwrap();
    std::fs::create_dir_all(d.path().join("Games/PCE")).unwrap();
    std::fs::write(d.path().join("Games/PCE/Bonk.pce"), [0u8; 16]).unwrap();

    let carts = omega_store::scan(d.path()).unwrap();
    let cd: Vec<_> = carts
        .iter()
        .filter(|c| c.platform == Platform::PceCd)
        .map(|c| c.stem.as_str())
        .collect();
    assert_eq!(cd, ["Ys Book I & II"]);
    let cards: Vec<_> = carts
        .iter()
        .filter(|c| c.platform == Platform::Pce)
        .map(|c| c.stem.as_str())
        .collect();
    assert_eq!(cards, ["Bonk"]);
}
