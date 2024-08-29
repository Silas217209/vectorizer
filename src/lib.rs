pub struct ColorImage {
    pub pixels: Vec<u8>,
    pub width: u64,
    pub height: u64,
}
impl ColorImage {
    fn to_orig(&self) -> vtracer::ColorImage {
        vtracer::ColorImage {
            pixels: self.pixels.clone(),
            width: self.width as usize,
            height: self.height as usize,
        }
    }
}

pub enum ColorMode {
    Color,
    Binary,
}
impl ColorMode {
    fn to_orig(&self) -> vtracer::ColorMode {
        match self {
            ColorMode::Color => vtracer::ColorMode::Color,
            ColorMode::Binary => vtracer::ColorMode::Binary,
        }
    }
}

pub enum Hierarchical {
    Stacked,
    Cutout,
}
impl Hierarchical {
    fn to_orig(&self) -> vtracer::Hierarchical {
        match self {
            Hierarchical::Stacked => vtracer::Hierarchical::Stacked,
            Hierarchical::Cutout => vtracer::Hierarchical::Cutout,
        }
    }
}

pub enum PathSimplifyMode {
    None,
    Polygon,
    Spline,
}
impl PathSimplifyMode {
    fn to_orig(&self) -> visioncortex::PathSimplifyMode {
        match self {
            PathSimplifyMode::None => visioncortex::PathSimplifyMode::None,
            PathSimplifyMode::Polygon => visioncortex::PathSimplifyMode::Polygon,
            PathSimplifyMode::Spline => visioncortex::PathSimplifyMode::Spline,
        }
    }
}

pub struct Config {
    pub color_mode: ColorMode,
    pub hierarchical: Hierarchical,
    pub filter_speckle: u64,
    pub color_precision: i32,
    pub layer_difference: i32,
    pub mode: PathSimplifyMode,
    pub corner_threshold: i32,
    pub length_threshold: f64,
    pub max_iterations: u64,
    pub splice_threshold: i32,
    pub path_precision: Option<u32>,
}

impl Config {
    fn to_orig(&self) -> vtracer::Config {
        vtracer::Config {
            color_mode: self.color_mode.to_orig(),
            hierarchical: self.hierarchical.to_orig(),
            filter_speckle: self.filter_speckle as usize,
            color_precision: self.color_precision,
            layer_difference: self.layer_difference,
            mode: self.mode.to_orig(),
            corner_threshold: self.corner_threshold,
            length_threshold: self.length_threshold,
            max_iterations: self.max_iterations as usize,
            splice_threshold: self.splice_threshold,
            path_precision: self.path_precision,
        }
    }
}

pub fn convert(img: ColorImage, config: Config) -> String {
    let r = vtracer::convert(img.to_orig(), config.to_orig());

    if r.is_err() {
        return String::new();
    }

    format!("{}", r.unwrap())
}
