use alacritty_terminal::vte::ansi::{self, NamedColor};
use egui::Color32;

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub foreground: String,
    pub background: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
    pub bright_foreground: Option<String>,
    pub dim_foreground: String,
    pub dim_black: String,
    pub dim_red: String,
    pub dim_green: String,
    pub dim_yellow: String,
    pub dim_blue: String,
    pub dim_magenta: String,
    pub dim_cyan: String,
    pub dim_white: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            foreground: String::from("#d8d8d8"),
            background: String::from("#181818"),
            black: String::from("#181818"),
            red: String::from("#ac4242"),
            green: String::from("#90a959"),
            yellow: String::from("#f4bf75"),
            blue: String::from("#6a9fb5"),
            magenta: String::from("#aa759f"),
            cyan: String::from("#75b5aa"),
            white: String::from("#d8d8d8"),
            bright_black: String::from("#6b6b6b"),
            bright_red: String::from("#c55555"),
            bright_green: String::from("#aac474"),
            bright_yellow: String::from("#feca88"),
            bright_blue: String::from("#82b8c8"),
            bright_magenta: String::from("#c28cb8"),
            bright_cyan: String::from("#93d3c3"),
            bright_white: String::from("#f8f8f8"),
            bright_foreground: None,
            dim_foreground: String::from("#828482"),
            dim_black: String::from("#0f0f0f"),
            dim_red: String::from("#712b2b"),
            dim_green: String::from("#5f6f3a"),
            dim_yellow: String::from("#a17e4d"),
            dim_blue: String::from("#456877"),
            dim_magenta: String::from("#704d68"),
            dim_cyan: String::from("#4d7770"),
            dim_white: String::from("#8e8e8e"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerminalTheme {
    foreground: Color32,
    background: Color32,
    normal: [Color32; 8],
    bright: [Color32; 8],
    bright_foreground: Color32,
    dim_foreground: Color32,
    dim: [Color32; 8],
    indexed: [Color32; 256],
}

impl Default for TerminalTheme {
    fn default() -> Self {
        Self::new(Box::new(ColorPalette::default()))
    }
}

impl TerminalTheme {
    pub fn new(palette: Box<ColorPalette>) -> Self {
        let normal = [
            hex_to_color32(&palette.black),
            hex_to_color32(&palette.red),
            hex_to_color32(&palette.green),
            hex_to_color32(&palette.yellow),
            hex_to_color32(&palette.blue),
            hex_to_color32(&palette.magenta),
            hex_to_color32(&palette.cyan),
            hex_to_color32(&palette.white),
        ];
        let bright = [
            hex_to_color32(&palette.bright_black),
            hex_to_color32(&palette.bright_red),
            hex_to_color32(&palette.bright_green),
            hex_to_color32(&palette.bright_yellow),
            hex_to_color32(&palette.bright_blue),
            hex_to_color32(&palette.bright_magenta),
            hex_to_color32(&palette.bright_cyan),
            hex_to_color32(&palette.bright_white),
        ];
        let dim = [
            hex_to_color32(&palette.dim_black),
            hex_to_color32(&palette.dim_red),
            hex_to_color32(&palette.dim_green),
            hex_to_color32(&palette.dim_yellow),
            hex_to_color32(&palette.dim_blue),
            hex_to_color32(&palette.dim_magenta),
            hex_to_color32(&palette.dim_cyan),
            hex_to_color32(&palette.dim_white),
        ];

        let foreground = hex_to_color32(&palette.foreground);
        let bright_foreground = palette
            .bright_foreground
            .as_ref()
            .map(|s| hex_to_color32(s))
            .unwrap_or(foreground);

        let mut indexed = [Color32::BLACK; 256];
        indexed[0..8].copy_from_slice(&normal);
        indexed[8..16].copy_from_slice(&bright);
        for r in 0..6u8 {
            for g in 0..6u8 {
                for b in 0..6u8 {
                    let idx = 16 + r * 36 + g * 6 + b;
                    indexed[idx as usize] = Color32::from_rgb(
                        if r == 0 { 0 } else { r * 40 + 55 },
                        if g == 0 { 0 } else { g * 40 + 55 },
                        if b == 0 { 0 } else { b * 40 + 55 },
                    );
                }
            }
        }
        for i in 0..24u8 {
            let value = i * 10 + 8;
            indexed[(232 + i) as usize] = Color32::from_rgb(value, value, value);
        }

        Self {
            foreground,
            background: hex_to_color32(&palette.background),
            normal,
            bright,
            bright_foreground,
            dim_foreground: hex_to_color32(&palette.dim_foreground),
            dim,
            indexed,
        }
    }

    pub fn get_color(&self, c: ansi::Color) -> Color32 {
        match c {
            ansi::Color::Spec(rgb) => Color32::from_rgb(rgb.r, rgb.g, rgb.b),
            ansi::Color::Indexed(index) => {
                self.indexed.get(index as usize).copied().unwrap_or(Color32::BLACK)
            },
            ansi::Color::Named(nc) => match nc {
                NamedColor::Foreground => self.foreground,
                NamedColor::Background => self.background,
                NamedColor::Black => self.normal[0],
                NamedColor::Red => self.normal[1],
                NamedColor::Green => self.normal[2],
                NamedColor::Yellow => self.normal[3],
                NamedColor::Blue => self.normal[4],
                NamedColor::Magenta => self.normal[5],
                NamedColor::Cyan => self.normal[6],
                NamedColor::White => self.normal[7],
                NamedColor::BrightBlack => self.bright[0],
                NamedColor::BrightRed => self.bright[1],
                NamedColor::BrightGreen => self.bright[2],
                NamedColor::BrightYellow => self.bright[3],
                NamedColor::BrightBlue => self.bright[4],
                NamedColor::BrightMagenta => self.bright[5],
                NamedColor::BrightCyan => self.bright[6],
                NamedColor::BrightWhite => self.bright[7],
                NamedColor::BrightForeground => self.bright_foreground,
                NamedColor::DimForeground => self.dim_foreground,
                NamedColor::DimBlack => self.dim[0],
                NamedColor::DimRed => self.dim[1],
                NamedColor::DimGreen => self.dim[2],
                NamedColor::DimYellow => self.dim[3],
                NamedColor::DimBlue => self.dim[4],
                NamedColor::DimMagenta => self.dim[5],
                NamedColor::DimCyan => self.dim[6],
                NamedColor::DimWhite => self.dim[7],
                _ => self.background,
            },
        }
    }
}

fn hex_to_color32(hex: &str) -> Color32 {
    hex_to_color(hex).unwrap_or_else(|_| panic!("invalid color {}", hex))
}

fn hex_to_color(hex: &str) -> anyhow::Result<Color32> {
    let is_solid = hex.len() == 7;
    let is_transparent = hex.len() == 9;
    if !is_solid && !is_transparent || !hex.starts_with('#') {
        return Err(anyhow::format_err!("input string is in non valid format"));
    }

    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;
    let a = if is_transparent {
        u8::from_str_radix(&hex[7..9], 16)?
    } else {
        255
    };

    Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
}
