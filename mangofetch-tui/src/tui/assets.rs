/// Mango fruit art — rendered in orange/yellow.
/// Each tuple: (left_pad, orange_segment, highlight_segment, orange_segment2)
/// Rendered as plain strings; coloring is done in ui.rs.
#[allow(dead_code)]
pub const MANGO_STEM: &[&str] = &["           \\|            ", "         ,-' '-.         "];

// Mango body (rendered in orange gradient)
// ░ zones are rendered in a lighter gold for the shine highlight
#[allow(dead_code)]
pub const MANGO_BODY: &[&str] = &[
    "        ▄▄▄████▄▄▄        ",
    "      ▄████████████▄      ",
    "    ▄████████████████▄    ",
    "   ████████████████████   ",
    "   ██████████████░░████   ",
    "   ██████████░░████████   ",
    "   ██████████████░░████   ",
    "    ▀████████████████▀    ",
    "      ▀████████████▀      ",
    "        ▀▀▀████▀▀▀        ",
];

/// MANGOFETCH spelled in 5-row block letters.
/// One entry per row (0-4).
#[allow(dead_code)]
pub const BLOCK_TITLE: &[&str] = &[
    "█▄ ▄█  ▄█▄  █▄  █  ████  ███  █████ █████ █████  ████ █   █ ",
    "█▀▄▀█ █   █ ██▄ █ █     █   █ █     █       █   █     █   █ ",
    "█   █ █████ █ ▀▄█ █  ██ █   █ ████  ████    █   █     █████ ",
    "█   █ █   █ █   █ █   █ █   █ █     █       █   █     █   █ ",
    "█   █ █   █ █   █  ████  ███  █     █████   █    ████ █   █ ",
];

pub const STYLIZED_MANGO: &[&str] = &[
    "      ╭──╮     ",
    "    ╭╯    │    ",
    "  ▄████████▄   ",
    " ████████████  ",
    " ████████████  ",
    "  ▀████████▀   ",
    "    ▀████▀     ",
];
