//==============================================================================
// Open crates and libraries
//==============================================================================
use fractal::definitions::*;
use fractal::draw;
use fractal::color;

//==============================================================================
// Types used by the app
//==============================================================================
pub struct FractalState{
  pub args : FracArgs,
  pub fractal : RawFrac,
}

impl Default for FractalState{
  fn default() -> Self { 
    let default_args = FracArgs::default(); 
    FractalState{
      args : default_args,
      fractal : draw::compute_fractal(default_args)
    }
  }
}

pub struct ImageState{
  pub args : ImageArgs,
  pub image : FracImage,
}

impl Default for ImageState{
  fn default() -> Self { ImageState{
    args : ImageArgs{ steps_off : 0 },
    image : color::color_fractal(&FractalState::default().fractal)
  }}
}
