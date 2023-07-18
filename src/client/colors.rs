use std::fmt::Display;

pub struct LinearGradient {
    pub color_a: String,
    pub color_a_percent: u8,
    
    pub color_b: String,
    pub color_b_percent: u8,

    pub is_repeating: bool,
}

impl ToString for LinearGradient {
    fn to_string(&self) -> String {
        let mut col_a = self.color_a.clone();

        if self.color_a_percent < 100 {
            col_a.push_str(&format!(" {}%", self.color_a_percent))
        }

        let mut col_b = self.color_b.clone();
        if self.color_b_percent < 100 {
            col_b.push_str(&format!(" {}%", self.color_b_percent))
        }
        
        format!("{}linear-gradient({col_a}, {col_b})", if self.is_repeating { "repeating-" } else { "" })
    }
}

impl LinearGradient {
    pub fn new(color_a: String, color_a_percent: u8, color_b: String, color_b_percent: u8) -> Self {
        Self {
            color_a,
            color_a_percent,
            color_b,
            color_b_percent,
            is_repeating: false
        }
    }

    pub fn new_repeating(color_a: String, color_a_percent: u8, color_b: String, color_b_percent: u8) -> Self {
        Self {
            color_a,
            color_a_percent,
            color_b,
            color_b_percent,
            is_repeating: true
        }
    }
}

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ToString for Rgb {
    fn to_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ToString for Rgba {
    fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub enum ColorShape {
    Circle,
    Ellipse
}

impl Display for ColorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::Ellipse => write!(f, "Ellipse"),
        }
    }
}

pub struct RadialGradient {
    pub shape: ColorShape,
    pub shape_position: u8,
    
    pub color_a: String,
    pub color_b: String,

    pub is_repeating: bool,
}

impl ToString for RadialGradient {
    fn to_string(&self) -> String {
        let mut shape = self.shape.to_string().to_lowercase();
        
        if self.shape_position < 100 {
            shape.push_str(&format!(" at {}%", self.shape_position));
        }

        let repeat = if self.is_repeating {
            "repeating-"
        } else {
            ""
        };

        format!("{}radial-gradient({}, {}, {})", repeat, shape, self.color_a, self.color_b)
    }
}

impl RadialGradient {
    pub fn new(shape: ColorShape, shape_position: u8, color_a: String, color_b: String) -> Self {
        Self {
            shape,
            shape_position,
            color_a,
            color_b,
            is_repeating: false
        }
    }

    pub fn new_repeating(shape: ColorShape, shape_position: u8, color_a: String, color_b: String) -> Self {
        Self {
            shape,
            shape_position,
            color_a,
            color_b,
            is_repeating: true
        }
    }
}