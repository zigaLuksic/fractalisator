//==============================================================================
// Field
//==============================================================================

/// `Field` describes the area in which the fractal is drawn. 
///
/// The area of the complex space is a square with the center in `(center_re,
/// center_im)` and the side is of length `2 * radius`. 
///
/// `pixel_size` states the number of pixels per square side, which defines the
/// `pixel_size * pixel_size` point mesh for evaluation.
#[derive(Copy, Clone)]
pub struct Field { 
  pub pixel_size  : usize,
  pub center_re   : f64, 
  pub center_im   : f64, 
  pub radius      : f64,
}

impl Default for Field {
  fn default() -> Self { 
    Field{ 
      pixel_size  : 1000, 
      radius      : 2.0,
      center_re   : 0.0, 
      center_im   : 0.0,
    }
  }
}

//==============================================================================
// Custom types and enums
//==============================================================================

/// Resulting type of fractal drawing. The `usize` lists the number of
/// steps taken before the iteration terminated. The `f64` elements describe 
/// the complex value at the end of iteration (required for smoothing).
pub type FracPoint = (usize, f64, f64);

/// Type of a calculated fractal.
pub type RawFrac = Vec<FracPoint>;

/// Type of a colored fractal image. A flattened vector of BGRA pixels with
/// `u8` values.
pub type FracImage = Vec<u8>;

/// States whether the fractal calculation uses the Julia or Mandelbrot style
/// of point iteration.
#[derive (Clone, Copy, Debug, PartialEq, Eq)]
pub enum IterationStyle { Julia, Mandelbrot }

impl std::fmt::Display for IterationStyle {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
     IterationStyle::Julia => "Julia style", 
     IterationStyle::Mandelbrot => "Mandelbrot style", 
    })
  }
}

pub static ALL_ITERATIONS : &[IterationStyle] = &[
  IterationStyle::Julia,
  IterationStyle::Mandelbrot,
  ];

/// Describes the complex function used in iteration when calculating fractals.
#[derive (Clone, Copy, Debug, PartialEq, Eq)]
pub enum IteratorKind { Square, Cube, Inverse, Ship }

impl std::fmt::Display for IteratorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
     IteratorKind::Square => "z ^ 2", 
     IteratorKind::Cube => "z ^ 3", 
     IteratorKind::Inverse => "z ^ -2", 
     IteratorKind::Ship => "(|a| + i|b|) ^ 2", 
    })
  }
}

pub static ALL_ITERATORS : &[IteratorKind] = &[
  IteratorKind::Square,
  IteratorKind::Cube,
  IteratorKind::Inverse,
  IteratorKind::Ship,
  ];

// Used to describe gradients. `bgra` specifies the color in BGRA `u8` format
// and `at` is a float (expected between 0 and 1) specifying where the gradient
// reaches the chosen color if one colors the interval [0, 1].
#[derive (Clone, Copy, Debug)]
pub struct ColorPeak {
  pub bgra : (u8, u8, u8, u8),
  pub at : f64
}

/// Describes the color gradient chosen for the picture. The gradient starts
/// with `starting_color` and then linearly transitions between colors in
/// `peaks` and ends with `end_color`. Features an option to smooth step
/// transition (designed for Mandebrot).
#[derive (Clone, Debug)]
pub struct Gradient {
  pub start_color : (u8, u8, u8, u8),
  pub peaks : Vec<ColorPeak>,
  pub end_color : (u8, u8, u8, u8),
  pub smooth : bool,
}

// Color Presets (These might be only temporarily here)
#[derive (Clone, Copy)]
pub enum GradientPreset { Azul, Svarog, Emperor, Gaia }

impl Gradient {

  pub fn azul() -> Gradient {
    Gradient {
      start_color : (0, 0, 0, 255),
      peaks : vec!(
        ColorPeak{bgra: (220, 120, 30, 255), at: 0.3},
        ColorPeak{bgra: (255, 230, 220, 255), at: 0.7},
        ),
      end_color : (0, 0, 0, 255),
      smooth : true,
    }}

  pub fn svarog() -> Gradient {
    Gradient {
      start_color : (0, 0, 0, 255),
      peaks : vec!(
        ColorPeak{bgra: (0, 0, 0, 255), at: 0.2},
        ColorPeak{bgra: (30, 60, 200, 255), at: 0.4},
        ColorPeak{bgra: (45, 180, 220, 255), at: 0.6},
        ColorPeak{bgra: (75, 210, 250, 255), at: 0.8},
        ),
      end_color : (255, 255, 255, 255),
      smooth : true,
    }}

  pub fn emperor() -> Gradient {
    Gradient {
      start_color : (0, 0, 0, 255),
      peaks : vec!(
        ColorPeak{bgra: (0, 0, 0, 255), at: 0.2},
        ColorPeak{bgra: (160, 30, 80, 255), at: 0.4},
        ColorPeak{bgra: (200, 40, 100, 255), at: 0.6},
        ColorPeak{bgra: (75, 210, 250, 255), at: 0.8},
        ),
      end_color : (255, 255, 255, 255),
      smooth : true,
    }}

  pub fn gaia() -> Gradient {
    Gradient {
      start_color : (0, 0, 0, 255),
      peaks : vec!(
        ColorPeak{bgra: (60, 50, 50, 255), at: 0.1},
        ColorPeak{bgra: (180, 140, 60, 255), at: 0.5},
        ColorPeak{bgra: (210, 200, 100, 255), at: 0.7},
        ColorPeak{bgra: (150, 230, 250, 255), at: 0.9},
        ),
      end_color : (255, 255, 255, 255),
      smooth : true,
    }}
}


pub static DEFAULT_GRADIENT_PRESET : GradientPreset = GradientPreset::Gaia;

impl Default for Gradient {
    fn default() -> Self { Gradient::gaia_gradient() }
  }

//==============================================================================
// Fractal and Image arguments
//==============================================================================

/// Arguments required for calculating a fractal.
/// - `field` the part of the complax plane to be drawn
/// - `(c_re, c_im)` complex constant used for Julia style fractals
/// - `steps` number of iteration steps per point before assuming divergence
/// - `iter_bound` iif absolute value of the complex point reaches this boundary
///   during iteration, we assume divergence
/// - `iteration_style` specifies Julia or Mandelbrot iteraton
/// - `iterator_kind` specifies the function used in iteration
#[derive(Copy, Clone)]
pub struct FracArgs {
  pub field : Field,
  pub c_re  : f64, 
  pub c_im  : f64, 
  pub steps : usize,
  pub iter_bound      : f64, 
  pub iteration_style : IterationStyle,
  pub iterator_kind   : IteratorKind,
}

impl Default for FracArgs {
  fn default() -> Self {
    FracArgs {
      field : Field::default(), 
      c_re  : 0.,
      c_im  : 0.,
      steps : 256, 
      iter_bound      : 10., 
      iteration_style : IterationStyle::Julia,
      iterator_kind   : IteratorKind::Square,
    }
  }
}

/// Arguments required for coloring a fractal.
/// - `gradient` specifies the gradient which is used
/// - `better_resize` toggles the use of a resizer before the image is passed
///   to the UI for display
#[derive(Clone)]
pub struct ImageArgs {
  pub gradient : Gradient,
}

impl Default for ImageArgs {
  fn default() -> Self {
    ImageArgs {
      gradient : Gradient::default(),
    }
  }
}
