//==============================================================================
// Open crates and libraries
//==============================================================================
use fractal::definitions::*;
use fractal::draw;
use fractal::color;

//==============================================================================
// Structs used by app
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
  fn default() -> Self { 
    let default_frac_state = FractalState::default();
    let frac = default_frac_state.fractal;
    let steps = default_frac_state.args.steps;
    let img_args = ImageArgs::default();
    let gradient = img_args.gradient.clone();
    ImageState{
      args : img_args,
      image : color::color_fractal(&frac, steps, gradient)}
  }
}
