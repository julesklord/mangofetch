use ratatui::prelude::*;

/// A complete color palette for the TUI.
pub struct Theme {
    /// Primary brand / interactive accent color
    pub accent: Color,
    /// Secondary / complementary accent (tab highlight, headings)
    pub secondary: Color,
    /// Subtle highlight for hover / selected rows
    pub highlight: Color,
    /// Full-screen background
    pub background: Color,
    /// Slightly lighter surface for panels, borders, statusline
    pub surface: Color,
    /// Dim surface for subtle separators
    pub surface_dim: Color,
    /// Default readable text
    pub text: Color,
    /// Muted text (timestamps, secondary info)
    pub text_dim: Color,
    /// Success / completed state
    pub success: Color,
    /// Warning / paused state
    pub warning: Color,
    /// Error state
    pub error: Color,
    /// Active / downloading state (progress bar fill)
    pub progress: Color,
}

impl Theme {
    /// 🥭 Mango — warm orange & gold on near-black
    pub fn mango() -> Self {
        Self {
            accent: Color::Rgb(255, 168, 38),    // mango orange
            secondary: Color::Rgb(255, 220, 80), // golden yellow
            highlight: Color::Rgb(60, 45, 15),   // dark warm surface
            background: Color::Rgb(16, 16, 18),
            surface: Color::Rgb(30, 28, 24),
            surface_dim: Color::Rgb(22, 20, 17),
            text: Color::Rgb(240, 230, 210),
            text_dim: Color::Rgb(120, 108, 85),
            success: Color::Rgb(80, 220, 120),
            warning: Color::Rgb(255, 200, 50),
            error: Color::Rgb(255, 90, 70),
            progress: Color::Rgb(255, 168, 38),
        }
    }

    /// 🍇 Pitaya — deep pink & purple on dark violet
    pub fn pitaya() -> Self {
        Self {
            accent: Color::Rgb(255, 60, 155),
            secondary: Color::Rgb(180, 100, 255),
            highlight: Color::Rgb(55, 20, 55),
            background: Color::Rgb(18, 10, 22),
            surface: Color::Rgb(35, 18, 42),
            surface_dim: Color::Rgb(25, 12, 30),
            text: Color::Rgb(240, 225, 255),
            text_dim: Color::Rgb(120, 85, 140),
            success: Color::Rgb(80, 220, 130),
            warning: Color::Rgb(255, 210, 60),
            error: Color::Rgb(240, 50, 80),
            progress: Color::Rgb(255, 60, 155),
        }
    }

    /// 🥥 Coconut — warm beige & brown on deep charcoal
    pub fn coconut() -> Self {
        Self {
            accent: Color::Rgb(235, 195, 130),
            secondary: Color::Rgb(170, 100, 50),
            highlight: Color::Rgb(45, 38, 25),
            background: Color::Rgb(14, 13, 11),
            surface: Color::Rgb(32, 28, 22),
            surface_dim: Color::Rgb(22, 19, 14),
            text: Color::Rgb(235, 225, 205),
            text_dim: Color::Rgb(110, 95, 70),
            success: Color::Rgb(100, 195, 80),
            warning: Color::Rgb(218, 165, 32),
            error: Color::Rgb(200, 60, 50),
            progress: Color::Rgb(200, 150, 80),
        }
    }

    /// 🧛 Dracula — classic dark purple + cyan
    pub fn dracula() -> Self {
        Self {
            accent: Color::Rgb(189, 147, 249),    // purple
            secondary: Color::Rgb(139, 233, 253), // cyan
            highlight: Color::Rgb(50, 40, 70),
            background: Color::Rgb(40, 42, 54),
            surface: Color::Rgb(52, 55, 70),
            surface_dim: Color::Rgb(44, 46, 60),
            text: Color::Rgb(248, 248, 242),
            text_dim: Color::Rgb(98, 114, 164),
            success: Color::Rgb(80, 250, 123),
            warning: Color::Rgb(241, 250, 140),
            error: Color::Rgb(255, 85, 85),
            progress: Color::Rgb(189, 147, 249),
        }
    }
}
