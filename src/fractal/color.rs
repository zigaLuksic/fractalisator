//==============================================================================
// Open crates and libraries
//==============================================================================
use fractal::definitions::*;

use std::cmp::min;
use std::cmp::max;

//==============================================================================
// Color Functions
//==============================================================================

pub fn azul_gradient(n : usize, re : f64, im : f64, max_steps : usize) 
  -> (u8, u8, u8, u8) {
  let max_steps = max_steps as f64;
  // Smooth and scale
  let val = re * re + im * im;
  let smooth1 = val.ln() / f64::ln(2.);
  let smooth2 = smooth1.ln() / f64::ln(2.);
  let n = (n as f64 - smooth2).max(0.0).min(max_steps);
  let peak1 = 1. * max_steps / 3.;
  let peak2 = 2. * max_steps / 3.;
  // Factors
  let f1 = 
    if n < peak1 {
      n / peak1
    } else if n < peak2 {
      1. - ( (n - peak1) / (peak2 - peak1) )
    } else {0.0};
  let f2 = 
    if n < peak2 {
      n / peak2
    } else if n < max_steps {
      1. - ( (n - peak2) / (max_steps - peak2) )
    } else {0.0};

  // Colors
  let (r1, g1, b1) = (30.0, 120.0, 220.0);
  let (r2, g2, b2) = (256.0, 256.0, 256.0);

  // Generate pixel
  let r = min(max((r1 * f1 + r2 * f2) as usize, 1), 255);
  let g = min(max((g1 * f1 + g2 * f2) as usize, 1), 255);
  let b = min(max((b1 * f1 + b2 * f2) as usize, 1), 255);
  (b as u8, g as u8, r as u8, 255)
}

//==============================================================================
// Fractal Coloring
//==============================================================================

pub fn color_fractal(fractal : &RawFrac, max_steps : usize) -> FracImage {
  let mut colored = Vec::with_capacity(fractal.len());
  let add_colored_pixel = |vec : &mut FracImage, data : FracPoint| {
    let (n, re, im) = data;
    let (b, g, r, a) = azul_gradient(n, re, im, max_steps);
    vec.push(b);
    vec.push(g);
    vec.push(r);
    vec.push(a);
  };
  fractal.iter().for_each(|&data| add_colored_pixel(&mut colored, data));
  colored
  }

