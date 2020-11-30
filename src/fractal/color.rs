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
      (n - peak1) / (peak2 - peak1)
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

pub fn sunset_gradient(n : usize, re : f64, im : f64, max_steps : usize) 
  -> (u8, u8, u8, u8) {
  let max_steps = max_steps as f64;
  // Smooth and scale
  let val = re * re + im * im;
  let smooth1 = val.ln() / f64::ln(2.);
  let smooth2 = smooth1.ln() / f64::ln(2.);
  let n = (n as f64 - smooth2).max(0.0).min(max_steps);
  let peak1 = 1. * max_steps / 6.;
  let peak2 = 2. * max_steps / 6.;
  let peak3 = 3. * max_steps / 6.;
  let peak4 = 4. * max_steps / 6.;
  let peak5 = 5. * max_steps / 6.;
  // Factors
  let f1 = 
    if n < peak1 {
      n / peak1
    } else if n > peak1 && n < peak2 {
      1. - ( (n - peak1) / (peak2 - peak1) )
    } else {0.0};
  let f2 = 
    if n > peak1 && n < peak2 {
      (n - peak1) / (peak2 - peak1)
    } else if n > peak2 && n < peak3 {
      1. - ( (n - peak2) / (peak3 - peak2) )
    } else {0.0};
  let f3 = 
    if n > peak2 && n < peak3 {
      (n - peak2) / (peak3 - peak2)
    } else if n > peak3 && n < peak4 {
      1. - ( (n - peak3) / (peak4 - peak3) )
    } else {0.0};
  let f4 = 
    if n > peak3 && n < peak4 {
      (n - peak3) / (peak4 - peak3)
    } else if n > peak4 && n < peak5 {
      1. - ( (n - peak4) / (peak5 - peak4) )
    } else {0.0};
  let f5 = 
    if n > peak4 && n < peak5 {
      (n - peak4) / (peak5 - peak4)
    } else if n > peak5 && n < max_steps {
      1. - ( (n - peak5) / (max_steps - peak5) )
    } else {0.0};
  // Colors
  let (r1, g1, b1) = (0.0, 0.0, 0.0);
  let (r2, g2, b2) = (200.0, 30.0, 10.0);
  let (r3, g3, b3) = (210.0, 140.0, 20.0);
  let (r4, g4, b4) = (256.0, 220.0, 30.0);
  let (r5, g5, b5) = (256.0, 240.0, 200.0);

  // Generate pixel
  let r = min(max((r1 * f1 + r2 * f2 + r3 * f3 + r4 * f4 + r5 * f5) as usize, 1), 255);
  let g = min(max((g1 * f1 + g2 * f2 + g3 * f3 + g4 * f4 + g5 * f5) as usize, 1), 255);
  let b = min(max((b1 * f1 + b2 * f2 + b3 * f3 + b4 * f4 + b5 * f5) as usize, 1), 255);
  (b as u8, g as u8, r as u8, 255)
}

pub fn gold_sky_gradient(n : usize, re : f64, im : f64, max_steps : usize) 
  -> (u8, u8, u8, u8) {
  let max_steps = max_steps as f64;
  // Smooth and scale
  let val = re * re + im * im;
  let smooth1 = val.ln() / f64::ln(2.);
  let smooth2 = smooth1.ln() / f64::ln(2.);
  let n = (n as f64 - smooth2).max(0.0).min(max_steps);
  let peak1 = 1. * max_steps / 6.;
  let peak2 = 2. * max_steps / 6.;
  let peak3 = 3. * max_steps / 6.;
  let peak4 = 4. * max_steps / 6.;
  let peak5 = 5. * max_steps / 6.;
  // Factors
  let f1 = 
    if n < peak1 {
      n / peak1
    } else if n > peak1 && n < peak2 {
      1. - ( (n - peak1) / (peak2 - peak1) )
    } else {0.0};
  let f2 = 
    if n > peak1 && n < peak2 {
      (n - peak1) / (peak2 - peak1)
    } else if n > peak2 && n < peak3 {
      1. - ( (n - peak2) / (peak3 - peak2) )
    } else {0.0};
  let f3 = 
    if n > peak2 && n < peak3 {
      (n - peak2) / (peak3 - peak2)
    } else if n > peak3 && n < peak4 {
      1. - ( (n - peak3) / (peak4 - peak3) )
    } else {0.0};
  let f4 = 
    if n > peak3 && n < peak4 {
      (n - peak3) / (peak4 - peak3)
    } else if n > peak4 && n < peak5 {
      1. - ( (n - peak4) / (peak5 - peak4) )
    } else {0.0};
  let f5 = 
    if n > peak4 && n < peak5 {
      (n - peak4) / (peak5 - peak4)
    } else if n > peak5 && n < max_steps {
      1. - ( (n - peak5) / (max_steps - peak5) )
    } else {0.0};
  // Colors
  let (r1, g1, b1) = (0.0, 0.0, 0.0);
  let (r2, g2, b2) = (40.0, 50.0, 180.0);
  let (r3, g3, b3) = (110.0, 140.0, 220.0);
  let (r4, g4, b4) = (256.0, 220.0, 80.0);
  let (r5, g5, b5) = (256.0, 240.0, 200.0);

  // Generate pixel
  let r = min(max((r1 * f1 + r2 * f2 + r3 * f3 + r4 * f4 + r5 * f5) as usize, 1), 255);
  let g = min(max((g1 * f1 + g2 * f2 + g3 * f3 + g4 * f4 + g5 * f5) as usize, 1), 255);
  let b = min(max((b1 * f1 + b2 * f2 + b3 * f3 + b4 * f4 + b5 * f5) as usize, 1), 255);
  (b as u8, g as u8, r as u8, 255)
}

//==============================================================================
// Fractal Coloring
//==============================================================================

pub fn color_fractal(fractal : &RawFrac, max_steps : usize, color : Color) -> FracImage {
  let mut colored = Vec::with_capacity(fractal.len());
  let add_colored_pixel = |vec : &mut FracImage, data : FracPoint| {
    let (n, re, im) = data;
    let (b, g, r, a) = match color {
      Color::Azul => { azul_gradient(n, re, im, max_steps) }
      Color::Sunset => { sunset_gradient(n, re, im, max_steps) }
      Color::Sky => { gold_sky_gradient(n, re, im, max_steps) }
    };
    vec.push(b);
    vec.push(g);
    vec.push(r);
    vec.push(a);
  };
  fractal.iter().for_each(|&data| add_colored_pixel(&mut colored, data));
  colored
  }

