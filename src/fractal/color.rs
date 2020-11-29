//==============================================================================
// Open crates and libraries
//==============================================================================
use image::{Bgra, ImageBuffer};

use std::cmp::min;
use std::cmp::max;

//==============================================================================
// Color Functions
//==============================================================================

pub fn azul_gradient(n : usize, re : f64, im : f64) -> (u8, u8, u8, u8) {
  // Smooth and scale
  let val = re * re + im * im;
  let smooth1 = val.ln() / f64::ln(2.);
  let smooth2 = smooth1.ln() / f64::ln(2.);
  let n = (n as f64 - smooth2).max(0.0).min(255.5);
  let n = n * 3.0;
  // Factors
  let n1 = if n < 256.0 {n} else if n < 512.0 {512.0 - n} else {0.0};
  let n2 = if n < 256.0 {0.0} else if n < 512.0 {n - 256.0} else {(768.0 - n).max(0.0)};

  // Colors
  let (r1, g1, b1) = (30.0, 120.0, 220.0);
  let (r2, g2, b2) = (256.0, 256.0, 256.0);
  // Calculate
  let (f1, f2) = (n1 / 256.0 , n2 / 256.0 );
  // Save
  let r = min(max((r1 * f1 + r2 * f2) as usize, 1), 255);
  let g = min(max((g1 * f1 + g2 * f2) as usize, 1), 255);
  let b = min(max((b1 * f1 + b2 * f2) as usize, 1), 255);
  (b as u8, g as u8, r as u8, 255)
}

//==============================================================================
// Fractal Coloring
//==============================================================================

pub fn color_fractal(fractal : &Vec<(usize, f64, f64)>) -> Vec<u8> {
  let mut colored = Vec::with_capacity(fractal.len());
  fn add_colored_pixel(vec : &mut Vec<u8>, data : (usize, f64, f64)) {
    let (n, re, im) = data;
    let (b, g, r, a) = azul_gradient(n, re, im);
    vec.push(b);
    vec.push(g);
    vec.push(r);
    vec.push(a);
  }
  fractal.iter().for_each(|&data| add_colored_pixel(&mut colored, data));
  colored
  }

