use ratatui::prelude::*;

pub struct Theme {
    pub accent: Color,
    pub secondary: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl Theme {
    pub fn mango() -> Self {
        Self {
            accent: Color::Rgb(255, 165, 0),    // Orange
            secondary: Color::Rgb(255, 215, 0), // Gold
            background: Color::Rgb(20, 20, 20), // Dark Gray
            surface: Color::Rgb(40, 40, 40),    // Lighter Gray
            text: Color::White,
            success: Color::Rgb(0, 255, 127),   // Spring Green
            warning: Color::Rgb(255, 255, 0),   // Yellow
            error: Color::Rgb(255, 69, 0),      // Red-Orange
        }
    }

    pub fn pitaya() -> Self {
        Self {
            accent: Color::Rgb(255, 20, 147),   // Deep Pink
            secondary: Color::Rgb(147, 112, 219), // Medium Purple
            background: Color::Rgb(25, 10, 25),
            surface: Color::Rgb(45, 25, 45),
            text: Color::White,
            success: Color::Rgb(50, 205, 50),
            warning: Color::Rgb(255, 215, 0),
            error: Color::Rgb(220, 20, 60),
        }
    }

    pub fn coconut() -> Self {
        Self {
            accent: Color::Rgb(245, 245, 220),  // Beige
            secondary: Color::Rgb(139, 69, 19),  // Brown
            background: Color::Rgb(15, 15, 15),
            surface: Color::Rgb(35, 35, 35),
            text: Color::Rgb(230, 230, 230),
            success: Color::Rgb(34, 139, 34),
            warning: Color::Rgb(218, 165, 32),
            error: Color::Rgb(178, 34, 34),
        }
    }
}
