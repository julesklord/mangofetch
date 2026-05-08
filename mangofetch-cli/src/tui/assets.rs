/// Mango fruit art — rendered in orange/yellow.
/// Each tuple: (left_pad, orange_segment, highlight_segment, orange_segment2)
/// Rendered as plain strings; coloring is done in ui.rs.
// Stem & leaf (rendered in green)
pub const MANGO_STEM: &[&str] = &[
    "            │            ",
    "           ╭╯            ",
    "          ╭╯             ",
];

// Mango body (rendered in orange gradient)
// ░ zones are rendered in a lighter gold for the shine highlight
pub const MANGO_BODY: &[&str] = &[
    "       ▄▄▄████████▄▄▄       ",
    "     ▄████████████████▄     ",
    "   ▄██░░░░██████████████▄   ",
    "  ████░░░░███████████████▌  ",
    "  ████░░░░███████████████▌  ",
    "  ██████████████████████▌   ",
    "  ▐██████████████████████   ",
    "   ▀████████████████████▀   ",
    "    ▀██████████████████▀    ",
    "      ▀████████████▀        ",
    "        ▀████████▀          ",
    "           ▀▀▀▀             ",
];

/// MANGOFETCH spelled in 5-row block letters.
/// One entry per row (0-4).
pub const BLOCK_TITLE: &[&str] = &[
    "█▄ ▄█  ▄█▄  █▄  █  ████  ███  █████ █████ █████  ████ █   █ ",
    "█▀▄▀█ █   █ ██▄ █ █     █   █ █     █       █   █     █   █ ",
    "█   █ █████ █ ▀▄█ █  ██ █   █ ████  ████    █   █     █████ ",
    "█   █ █   █ █   █ █   █ █   █ █     █       █   █     █   █ ",
    "█   █ █   █ █   █  ████  ███  █     █████   █    ████ █   █ ",
];

#[allow(dead_code)]
pub const STYLIZED_MANGO: &[&str] = &[
    "      ╭──╮     ",
    "    ╭╯    │    ",
    "  ▄████████▄   ",
    " ████████████  ",
    " ████████████  ",
    "  ▀████████▀   ",
    "    ▀████▀     ",
];
