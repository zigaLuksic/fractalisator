
//==============================================================================
// Field
//==============================================================================

#[derive(Copy, Clone)]
pub struct Field { 
  pub pixel_size : usize,
  pub re_center : f64, 
  pub im_center : f64, 
  pub radius : f64}

impl Default for Field {
  fn default() -> Self { Field{ 
      pixel_size : 1000, radius : 2.0,
      re_center : 0.0 , im_center : 0.0}
  }
}

//==============================================================================
// Custom types and enums
//==============================================================================

pub type FracPoint = (usize, f64, f64);

pub type RawFrac = Vec<FracPoint>;

pub type FracImage = Vec<u8>;

#[derive (Clone, Copy, Debug)]
pub enum IterationStyle { Julia, Mandelbrot }

#[derive (Clone, Copy, Debug)]
pub enum IteratorKind { Square, Cube, Ship }

// Should be generalised
#[derive (Clone, Copy, Debug)]
pub enum Color {Azul, Sunset, Sky }

//==============================================================================
// Generation arguments
//==============================================================================

#[derive(Copy, Clone)]
pub struct FracArgs{
  pub field : Field,
  pub c_im : f64, 
  pub c_re : f64, 
  pub iteration_bound : usize, 
  pub steps : usize,
  pub iteration_style : IterationStyle,
  pub iterator_kind : IteratorKind,
  }

impl Default for FracArgs{
  fn default() -> Self {FracArgs{
    field : Field::default(), 
    steps : 256, 
    iteration_bound : 10, 
    c_im : 0.,
    c_re : 0.,
    iteration_style : IterationStyle::Mandelbrot,
    iterator_kind : IteratorKind::Square,
  }}
}


#[derive(Copy, Clone)]
pub struct ImageArgs{
  pub color : Color,
  }

impl Default for ImageArgs{
  fn default() -> Self {ImageArgs{
    color : Color::Azul,
  }}
}
