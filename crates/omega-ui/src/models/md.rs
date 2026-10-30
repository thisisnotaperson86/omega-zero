//! The Mega Drive cart, in its two shells: the Mega Drive's of Japan and Europe, and the
//! Genesis's of America.
//!
//! Both are drawn 4 units to the millimetre, and both stand wider than they are tall, as the
//! carts do, 109 by 70 mm and 17 mm thick, at the Master System cart's scale, the same 2.4
//! pixels to the millimetre. The Mega Drive cart, drawn from its 3D model, has its label in a
//! well that runs up to the top edge between two broad black shoulders, ribbed for the grip, a
//! foot with a step under it, and the console's name across the label's foot; the Genesis cart has three grip
//! ridges across its top and its label in a well under them. Each label well is the shape of
//! ScreenScraper's scans, 1.2 to 1. Both are black. The ridges, bar and wells are relief in the face, not
//! geometry. Both faces are drawn about 1.78 pixels to the millimetre, as the NES carts are.

use std::sync::OnceLock;

use omega_store::{Cart, Outline};

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const MEGA_DRIVE_SVG: &str = include_str!("../../assets/megadrive_cart.svg");
const MEGA_DRIVE_DETAIL_SVG: &str = include_str!("../../assets/megadrive_cart_detail.svg");
const GENESIS_SVG: &str = include_str!("../../assets/genesis_cart.svg");
const GENESIS_DETAIL_SVG: &str = include_str!("../../assets/genesis_cart_detail.svg");

/// The Mega Drive cart shells a model is built for.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MdShell {
    /// The Mega Drive's, of Japan and Europe.
    MegaDrive,
    /// The Genesis's, of America.
    Genesis,
}

/// How thick each cart is, as a share of its width: 20 mm on the Mega Drive cart's 112 and the
/// Genesis cart's 110.
pub const MEGA_DRIVE_THICK: f32 = 17.0 / 109.0;
pub const GENESIS_THICK: f32 = 17.0 / 109.0;

/// How big the cart stands on the shelf, as a share of its own size: its face is made at the
/// size it stands at, so its label lands a texel to a pixel there.
pub const SHELF_SCALE: f32 = 1.0;

/// The Mega Drive cart's outline, right half, in drawing units (436 by 280), clockwise from the
/// top: its round top corner in three cuts and its foot in two. The left half is its mirror.
const MEGA_DRIVE_HALF: &[(f32, f32)] = &[
    (423.0, 0.0),
    (429.5, 1.74),
    (434.26, 6.5),
    (436.0, 13.0),
    (436.0, 260.0),
    (433.32, 270.0),
    (426.0, 277.32),
    (416.0, 280.0),
];
const MEGA_DRIVE_UNITS: (f32, f32) = (436.0, 280.0);

/// The Genesis cart's, right half, in drawing units (436 by 280): its top corner in three cuts
/// and its foot in two.
const GENESIS_HALF: &[(f32, f32)] = &[
    (426.0, 0.0),
    (431.0, 1.34),
    (434.66, 5.0),
    (436.0, 10.0),
    (436.0, 274.0),
    (434.24, 278.24),
    (430.0, 280.0),
];
const GENESIS_UNITS: (f32, f32) = (436.0, 280.0);

/// The size of each face in pixels. Each is a multiple of four across, so it halves onto the art
/// grid in the middle of the row, and an even number down.
pub const MEGA_DRIVE_W: u32 = 264;
pub const MEGA_DRIVE_H: u32 = 170;
pub const GENESIS_W: u32 = 264;
pub const GENESIS_H: u32 = 170;

/// The label on each face, in its pixels: the inside of its well, 1.2 to 1 as ScreenScraper's
/// scans are. The Mega Drive cart's, from the 3D model of it, is 73 mm by 61, in a well 77 wide, between its
/// shoulders and open to its top edge, its lower corners round; the
/// Genesis cart's 73 by 61 under its grip ridges.
const MEGA_DRIVE_LABEL: (u32, u32, u32, u32) = (44, 4, 176, 147);
const MEGA_DRIVE_LABEL_ROUND: u32 = 5;
const GENESIS_LABEL: (u32, u32, u32, u32) = (44, 16, 176, 147);

/// How far in from the outline a clear shell's plastic lightens at its rim, in pixels.
const RIM: u32 = 5;

/// The shell `cart` comes in: the one `cart_shell.ini` names, else the Mega Drive's for a game
/// of any region. The Genesis shell is still there, for a game the ini gives it to.
pub fn shell_of(cart: &Cart) -> MdShell {
    match cart.shell.map(|c| c.outline) {
        Some(Outline::Genesis) => MdShell::Genesis,
        _ => MdShell::MegaDrive,
    }
}

/// The cart's outline in drawing units, all the way round clockwise, and the drawing's size.
pub(crate) fn outline(shell: MdShell) -> (Vec<(f32, f32)>, (f32, f32)) {
    let (half, units) = match shell {
        MdShell::MegaDrive => (MEGA_DRIVE_HALF, MEGA_DRIVE_UNITS),
        MdShell::Genesis => (GENESIS_HALF, GENESIS_UNITS),
    };
    let mut all = half.to_vec();
    all.extend(half.iter().rev().map(|&(x, y)| (units.0 - x, y)));
    (all, units)
}

/// How thick `shell` is, as a share of its width.
pub(crate) fn thick(shell: MdShell) -> f32 {
    match shell {
        MdShell::MegaDrive => MEGA_DRIVE_THICK,
        MdShell::Genesis => GENESIS_THICK,
    }
}

/// The cart's silhouette `w` by `h`, as coverage: opaque inside the outline.
pub fn silhouette(shell: MdShell, w: u32, h: u32) -> Vec<u8> {
    let svg = match shell {
        MdShell::MegaDrive => MEGA_DRIVE_SVG,
        MdShell::Genesis => GENESIS_SVG,
    };
    rasterise_svg(svg, w, h).unwrap_or_else(|| vec![255; (w * h) as usize])
}

/// The size of the cart's face in pixels.
pub(crate) fn face_box(shell: MdShell) -> (u32, u32) {
    match shell {
        MdShell::MegaDrive => (MEGA_DRIVE_W, MEGA_DRIVE_H),
        MdShell::Genesis => (GENESIS_W, GENESIS_H),
    }
}

fn mask(shell: MdShell) -> &'static [u8] {
    static MEGA_DRIVE: OnceLock<Vec<u8>> = OnceLock::new();
    static GENESIS: OnceLock<Vec<u8>> = OnceLock::new();
    let lock = match shell {
        MdShell::MegaDrive => &MEGA_DRIVE,
        MdShell::Genesis => &GENESIS,
    };
    lock.get_or_init(|| {
        let (w, h) = face_box(shell);
        silhouette(shell, w, h)
    })
}

fn depth(shell: MdShell) -> &'static [u8] {
    static MEGA_DRIVE: OnceLock<Vec<u8>> = OnceLock::new();
    static GENESIS: OnceLock<Vec<u8>> = OnceLock::new();
    let lock = match shell {
        MdShell::MegaDrive => &MEGA_DRIVE,
        MdShell::Genesis => &GENESIS,
    };
    lock.get_or_init(|| {
        let (w, h) = face_box(shell);
        depth_map(mask(shell), w as usize, h as usize)
    })
}

fn detail(shell: MdShell) -> &'static Detail {
    static MEGA_DRIVE: OnceLock<Detail> = OnceLock::new();
    static GENESIS: OnceLock<Detail> = OnceLock::new();
    let (lock, svg) = match shell {
        MdShell::MegaDrive => (&MEGA_DRIVE, MEGA_DRIVE_DETAIL_SVG),
        MdShell::Genesis => (&GENESIS, GENESIS_DETAIL_SVG),
    };
    lock.get_or_init(|| {
        let (w, h) = face_box(shell);
        rasterise_detail(svg, w, h).unwrap_or_else(|| Detail::blank(w, h))
    })
}

/// How the cart's face is laid out. A clear shell shows the board inside: across the cart, its
/// contacts along its foot.
pub(crate) fn spec(shell: MdShell) -> Spec {
    let (w, h) = face_box(shell);
    let (label, label_round) = match shell {
        MdShell::MegaDrive => (MEGA_DRIVE_LABEL, MEGA_DRIVE_LABEL_ROUND),
        MdShell::Genesis => (GENESIS_LABEL, 0),
    };
    Spec {
        w,
        h,
        label,
        label_round,
        mask: mask(shell),
        depth: depth(shell),
        detail: detail(shell),
        max_h: (label.3 - 2 * PAD) as f32,
        rim: RIM,
        board: Board {
            x: (0.1, 0.9),
            top: 0.12,
            pins: (0.15, 0.85),
            contacts_from: 0.9,
            traces_from: 0.82,
        },
    }
}
