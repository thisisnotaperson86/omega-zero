use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Platform {
    Nes,
    Snes,
    Gb,
    Gbc,
    #[default]
    Gba,
    Md,
    Sms,
    Gg,
    Ps1,
    Pce,
    PceCd,
    Ngpc,
}

impl Platform {
    /// Every system, shelved by maker: Nintendo's, Sega's, Sony's, NEC's and SNK's.
    pub const ALL: [Platform; 12] = [
        Platform::Nes,
        Platform::Snes,
        Platform::Gb,
        Platform::Gbc,
        Platform::Gba,
        Platform::Md,
        Platform::Sms,
        Platform::Gg,
        Platform::Ps1,
        Platform::Pce,
        Platform::PceCd,
        Platform::Ngpc,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Platform::Gba => "Game Boy Advance",
            Platform::Gb => "Game Boy",
            Platform::Gbc => "Game Boy Color",
            Platform::Snes => "Super Nintendo",
            Platform::Ps1 => "PlayStation",
            Platform::Pce => "PC Engine",
            Platform::Nes => "Nintendo Entertainment System",
            Platform::Md => "Mega Drive",
            Platform::Gg => "Game Gear",
            Platform::Sms => "Master System",
            Platform::PceCd => "PC Engine CD",
            Platform::Ngpc => "Neo Geo Pocket Color",
        }
    }

    pub fn dir_name(self) -> &'static str {
        match self {
            Platform::Gba => "GBA",
            Platform::Gb => "GB",
            Platform::Gbc => "GBC",
            Platform::Snes => "SNES",
            Platform::Ps1 => "PS1",
            Platform::Pce => "PCE",
            Platform::Nes => "NES",
            Platform::Md => "MD",
            Platform::Gg => "GG",
            Platform::Sms => "SMS",
            Platform::PceCd => "PCECD",
            Platform::Ngpc => "NGPC",
        }
    }

    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Platform::Gba => &["gba"],
            Platform::Gb | Platform::Gbc => &["gb", "gbc"],
            Platform::Snes => &["sfc", "smc"],
            Platform::Ps1 => &["cue", "chd", "pbp", "m3u"],
            Platform::Pce => &["pce"],
            Platform::Nes => &["nes"],
            Platform::Md => &["md", "gen", "bin"],
            Platform::Gg => &["gg"],
            Platform::Sms => &["sms"],
            Platform::PceCd => &["cue", "chd", "m3u"],
            Platform::Ngpc => &["ngp", "ngc", "ngpc"],
        }
    }

    pub fn picture(self) -> (u32, u32) {
        match self {
            Platform::Gba => (240, 160),
            Platform::Gb | Platform::Gbc => (160, 144),
            Platform::Snes => (256, 224),
            Platform::Ps1 => (320, 240),
            Platform::Pce => (256, 240),
            Platform::Nes => (256, 224),
            Platform::Md => (320, 224),
            Platform::Gg => (160, 144),
            Platform::Sms => (256, 192),
            Platform::PceCd => (256, 240),
            Platform::Ngpc => (160, 152),
        }
    }

    /// Whether the frontend keeps L1/R1 while a game of this system plays, for its picture
    /// size: only the Game Boys and the Game Gear, whose games have no shoulder buttons.
    pub fn owns_shoulders(self) -> bool {
        matches!(self, Platform::Gb | Platform::Gbc | Platform::Gg)
    }

    /// Whether held X/Y are turbo A/B: on the handhelds, the NES and the Master System, whose
    /// games have no X or Y.
    pub fn turbo_xy(self) -> bool {
        matches!(
            self,
            Platform::Gba
                | Platform::Gb
                | Platform::Gbc
                | Platform::Nes
                | Platform::Gg
                | Platform::Sms
                | Platform::Ngpc
        )
    }

    /// Whether the core's colour correction for a handheld's screen applies.
    pub fn colour_correction(self) -> bool {
        matches!(self, Platform::Gba | Platform::Gb | Platform::Gbc)
    }

    /// Whether the handheld LCD's ghosting applies: the cores that run the handhelds' games,
    /// mGBA and gpSP, can blend each frame with the last, as Genesis Plus GX can a Game Gear's.
    pub fn lcd_ghosting(self) -> bool {
        matches!(
            self,
            Platform::Gba | Platform::Gb | Platform::Gbc | Platform::Gg
        )
    }

    /// Whether held L2 rewinds: not on the PlayStation, whose games use L2 and whose states
    /// are too big for the rewind buffer.
    pub fn rewinds(self) -> bool {
        self != Platform::Ps1
    }

    /// Whether L2/R2 belong to the game rather than to rewind and fast-forward.
    pub fn game_owns_l2_r2(self) -> bool {
        self == Platform::Ps1
    }

    pub fn accepts(self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                self.extensions()
                    .iter()
                    .any(|k| ext.eq_ignore_ascii_case(k))
            })
    }
}
