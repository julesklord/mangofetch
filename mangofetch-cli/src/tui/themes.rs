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

    /// 🍐 Guava — pink flesh & green skin
    pub fn guava() -> Self {
        Self {
            accent: Color::Rgb(255, 110, 130),   // pink flesh
            secondary: Color::Rgb(140, 210, 80), // green skin
            highlight: Color::Rgb(60, 20, 30),
            background: Color::Rgb(22, 12, 15),
            surface: Color::Rgb(45, 22, 28),
            surface_dim: Color::Rgb(30, 15, 18),
            text: Color::Rgb(245, 230, 235),
            text_dim: Color::Rgb(150, 110, 120),
            success: Color::Rgb(100, 230, 100),
            warning: Color::Rgb(255, 220, 60),
            error: Color::Rgb(255, 70, 70),
            progress: Color::Rgb(255, 110, 130),
        }
    }

    /// 🟠 Papaya — vibrant orange & dark seeds
    pub fn papaya() -> Self {
        Self {
            accent: Color::Rgb(255, 130, 40),    // papaya orange
            secondary: Color::Rgb(255, 190, 60), // light orange
            highlight: Color::Rgb(45, 25, 10),
            background: Color::Rgb(15, 12, 8),
            surface: Color::Rgb(35, 25, 18),
            surface_dim: Color::Rgb(25, 18, 12),
            text: Color::Rgb(250, 240, 230),
            text_dim: Color::Rgb(140, 120, 100),
            success: Color::Rgb(120, 210, 80),
            warning: Color::Rgb(255, 200, 50),
            error: Color::Rgb(255, 80, 40),
            progress: Color::Rgb(255, 130, 40),
        }
    }

    /// 🌺 Passionfruit — bright yellow & deep purple
    pub fn passionfruit() -> Self {
        Self {
            accent: Color::Rgb(255, 230, 40),    // bright yellow
            secondary: Color::Rgb(160, 60, 220), // passion purple
            highlight: Color::Rgb(40, 10, 55),
            background: Color::Rgb(15, 8, 20),
            surface: Color::Rgb(35, 15, 45),
            surface_dim: Color::Rgb(25, 10, 32),
            text: Color::Rgb(245, 235, 255),
            text_dim: Color::Rgb(150, 120, 180),
            success: Color::Rgb(100, 240, 150),
            warning: Color::Rgb(255, 200, 60),
            error: Color::Rgb(255, 60, 100),
            progress: Color::Rgb(255, 230, 40),
        }
    }

    /// 🍒 Lychee — translucent white & red skin
    pub fn lychee() -> Self {
        Self {
            accent: Color::Rgb(245, 245, 245),  // white flesh
            secondary: Color::Rgb(230, 60, 80), // red skin
            highlight: Color::Rgb(55, 25, 30),
            background: Color::Rgb(18, 12, 14),
            surface: Color::Rgb(42, 28, 32),
            surface_dim: Color::Rgb(28, 18, 22),
            text: Color::Rgb(250, 240, 242),
            text_dim: Color::Rgb(160, 130, 135),
            success: Color::Rgb(120, 240, 120),
            warning: Color::Rgb(255, 210, 80),
            error: Color::Rgb(255, 80, 80),
            progress: Color::Rgb(230, 60, 80),
        }
    }

    /// ⭐ Starfruit — bright yellow & green edges
    pub fn starfruit() -> Self {
        Self {
            accent: Color::Rgb(255, 255, 0),     // bright yellow
            secondary: Color::Rgb(173, 255, 47), // green yellow
            highlight: Color::Rgb(40, 40, 0),
            background: Color::Rgb(12, 12, 0),
            surface: Color::Rgb(30, 30, 10),
            surface_dim: Color::Rgb(20, 20, 5),
            text: Color::Rgb(255, 255, 220),
            text_dim: Color::Rgb(180, 180, 100),
            success: Color::Rgb(50, 205, 50),
            warning: Color::Rgb(255, 165, 0),
            error: Color::Rgb(255, 69, 0),
            progress: Color::Rgb(255, 255, 0),
        }
    }

    /// 🫐 Açai — deep purple & black
    pub fn acai() -> Self {
        Self {
            accent: Color::Rgb(75, 0, 130),      // indigo
            secondary: Color::Rgb(138, 43, 226), // blue violet
            highlight: Color::Rgb(20, 0, 40),
            background: Color::Rgb(10, 5, 15),
            surface: Color::Rgb(25, 10, 35),
            surface_dim: Color::Rgb(15, 5, 20),
            text: Color::Rgb(230, 210, 250),
            text_dim: Color::Rgb(140, 110, 180),
            success: Color::Rgb(0, 255, 127),
            warning: Color::Rgb(255, 140, 0),
            error: Color::Rgb(255, 20, 147),
            progress: Color::Rgb(138, 43, 226),
        }
    }

    /// 🍇 Mangosteen — dark purple & pure white
    pub fn mangosteen() -> Self {
        Self {
            accent: Color::Rgb(255, 255, 255), // white flesh
            secondary: Color::Rgb(80, 0, 80),  // dark purple skin
            highlight: Color::Rgb(40, 0, 40),
            background: Color::Rgb(20, 5, 20),
            surface: Color::Rgb(50, 10, 50),
            surface_dim: Color::Rgb(35, 5, 35),
            text: Color::Rgb(255, 240, 255),
            text_dim: Color::Rgb(180, 140, 180),
            success: Color::Rgb(144, 238, 144),
            warning: Color::Rgb(255, 215, 0),
            error: Color::Rgb(255, 105, 180),
            progress: Color::Rgb(255, 255, 255),
        }
    }

    /// 🥝 Kiwi — fuzzy brown & bright green
    pub fn kiwi() -> Self {
        Self {
            accent: Color::Rgb(154, 205, 50),   // yellow green
            secondary: Color::Rgb(139, 69, 19), // saddle brown
            highlight: Color::Rgb(30, 40, 10),
            background: Color::Rgb(15, 15, 5),
            surface: Color::Rgb(40, 35, 15),
            surface_dim: Color::Rgb(25, 22, 10),
            text: Color::Rgb(240, 255, 210),
            text_dim: Color::Rgb(160, 150, 100),
            success: Color::Rgb(50, 205, 50),
            warning: Color::Rgb(255, 215, 0),
            error: Color::Rgb(255, 99, 71),
            progress: Color::Rgb(154, 205, 50),
        }
    }
}
