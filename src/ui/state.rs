//==============================================================================
// Open crates and libraries
//==============================================================================
use fractal::definitions::*;
use fractal::draw;
use fractal::color;

//==============================================================================
// Structs for keeping args and image data
//==============================================================================

/// Describes the fractal. Includes the fractals arguments and a rendered raw
///  fractal picture to avoid redrawing unless necessary.
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

/// Describes the image. Includes the coloring arguments and a colored picture
/// to avoid redrawing unless necessary.
pub struct ImageState{
  pub args : ImageArgs,
  pub image : FracImage,
  pub current_preset : GradientPreset,
}

impl Default for ImageState{
  fn default() -> Self { 
    let default_frac_state = FractalState::default();
    // draw and color default fractal
    let frac = default_frac_state.fractal;
    let steps = default_frac_state.args.steps;
    let img_args = ImageArgs::default();
    let gradient = img_args.gradient.clone();
    // create
    ImageState{
      args : img_args,
      current_preset : GradientPreset::Azul,
      image : color::color_fractal(&frac, steps, gradient)}
  }
}
