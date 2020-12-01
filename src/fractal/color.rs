//==============================================================================
// Open crates and libraries
//==============================================================================
use fractal::definitions::*;

use resize::Pixel::RGBA;
use resize::Type::Lanczos3;

//==============================================================================
// Color Function Creation
//==============================================================================

/// Creates a function that calculates the RGBA color of a fractal point by
/// using `gradient`. To properly scale the gradient `max_steps` is required.
pub fn color_with_gradient(
  gradient : Gradient, 
  max_steps : usize)
-> impl Fn(usize, f64, f64) -> (u8, u8, u8, u8) {
  let Gradient{start_color, peaks, end_color, smooth} = gradient;

  // Sort peaks by `at`
  let mut peaks = peaks.to_vec();
  peaks.sort_by(|a, b| a.at.partial_cmp(&b.at).unwrap());

  // Adjust type
  let max_steps = max_steps as f64;

  // Definition of gradient
  move |n, re, im|{

    // Apply smoothing
    let n =
      if smooth {
        let abs_val = re * re + im * im;
        let smooth1 = abs_val.ln() / f64::ln(2.);
        let smooth2 = smooth1.ln() / f64::ln(2.);
        (n as f64) - smooth2
      } else {
        n as f64
      };

    // Transfer `n` to interval [0, 1]
    let x = (n / (max_steps as f64)).max(0.0).min(1.0);

    // Get the two bounding peaks
    let mut peak1 = ColorPeak{at: 0., bgra: start_color};
    let mut peak2 = ColorPeak{at: 0., bgra: start_color};
    let mut i = 0;
    while i < peaks.len() && peak2.at <= x {
      peak1 = peak2;
      peak2 = peaks[i];
      i += 1;
    }
    if i == peaks.len() && peak2.at <= x {
      peak1 = peak2;
      peak2 = ColorPeak{at: 1.0, bgra: end_color};
    }

    // Extract peak colors at f64
    let (b1, g1, r1, a1) = peak1.bgra;
    let (b1, g1, r1, a1) = (b1 as f64, g1 as f64, r1 as f64, a1 as f64);
    let (b2, g2, r2, a2) = peak2.bgra;
    let (b2, g2, r2, a2) = (b2 as f64, g2 as f64, r2 as f64, a2 as f64);
    // Combine colors
    let f = (x - peak1.at) / (peak2.at - peak1.at);

    let b = ((b1 * (1. - f) + b2 * f).round() as isize).min(255).max(0);
    let g = ((g1 * (1. - f) + g2 * f).round() as isize).min(255).max(0);
    let r = ((r1 * (1. - f) + r2 * f).round() as isize).min(255).max(0);
    let a = ((a1 * (1. - f) + a2 * f).round() as isize).min(255).max(0);

    (b as u8, g as u8, r as u8, a as u8)
  }

}

//==============================================================================
// Fractal Coloring
//==============================================================================

/// Takes fractal data `fractal` and colors it with the color `color`. To
/// correctly render the gradient it is required to know `max_steps`.
pub fn color_fractal(
  fractal : &RawFrac, 
  max_steps : usize, 
  gradient : Gradient ) 
-> FracImage {
  // initialize vector with enough fields (4 times for BGRA format)
  let mut colored = Vec::with_capacity(4 * fractal.len());

  // create coloring function
  let color_fun = color_with_gradient(gradient, max_steps);

  // make closure for coloring a single data point
  let add_colored_pixel = |vec : &mut FracImage, data : FracPoint| {
    let (n, re, im) = data;
    let (b, g, r, a) = color_fun(n, re, im);
    vec.push(b);
    vec.push(g);
    vec.push(r);
    vec.push(a);
  };

  // color entire fractal
  fractal.iter().for_each(|&data| add_colored_pixel(&mut colored, data));

  colored
}

/// Resizes fractal `image` from the dimensions `from` to dimensions `to` by
/// using a resizer. Greatly improves quality of image preview.
pub fn resize_fractal_image(
  image : &FracImage, 
  from : usize, 
  to : usize) 
-> FracImage {
  let mut resized = vec![0;to*to*4];
  let mut resizer = resize::new(from, from, to, to, RGBA, Lanczos3);

  resizer.resize(image, &mut resized);

  resized
}
