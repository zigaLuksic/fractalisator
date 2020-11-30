//==============================================================================
// Open crates and libraries
//==============================================================================
use rayon::prelude::*;

use fractal::definitions::*;

//==============================================================================
// Pixels to complex
//==============================================================================
pub fn point_to_complex (field : &Field, i : usize, j : usize) -> (f64, f64) {
  let Field {pixel_size, re_center, im_center, radius} = field;
  let dist = 2.0 * radius;

  let x_relative = i as f64 / *pixel_size as f64;
  let x_position = (re_center - radius) + dist * x_relative;

  let y_relative = j as f64 / *pixel_size as f64;
  let y_position = (im_center - radius) + dist * y_relative;

  (x_position, y_position)
}


//==============================================================================
// Fractal generators
//==============================================================================
#[allow(dead_code)]
fn julia_iterate (
  z : (f64, f64), 
  f : impl Fn(f64, f64, f64, f64) -> (f64, f64), 
  args : &FracArgs) 
-> FracPoint {
  // Loop mutables
  let (z_re, z_im) = z;
  let mut step = 0;
  let mut re = z_re;
  let mut im = z_im;
  // Loop constants
  let boundary = (args.iteration_bound * args.iteration_bound) as f64;

  while step < args.steps && (re * re + im * im) < boundary {
    let (new_re, new_im) = f(re, im, args.c_re, args.c_im);
    re = new_re;
    im = new_im;
    step += 1 
  }
  (step, re, im)
}


// Old version with hardcoded square iteratior

// fn mandelbrot_iterate (z : (f64, f64), args : &FracArgs) -> FracPoint {
//   // Loop mutables
//   let mut step = 0;
//   let mut re = 0.0;
//   let mut im = 0.0;
//   // Loop constants
//   let (const_re, const_im) = z;
//   let boundary = (args.bound * args.bound) as f64;

//   while step < args.steps && (re * re + im * im) < boundary {
//     let new_re = (re * re) - (im * im) + const_re;
//     im = (2.0 * re * im) + const_im;
//     re = new_re;
//     step += 1 
//   }
//   (step, re, im)
// }


fn mandelbrot_iterate (
  z : (f64, f64), 
  f : impl Fn(f64, f64, f64, f64) -> (f64, f64), 
  args : &FracArgs) 
-> FracPoint {
  // Loop mutables
  let mut step = 0;
  let mut re = 0.0;
  let mut im = 0.0;
  // Loop constants
  let (const_re, const_im) = z;
  let boundary = (args.iteration_bound * args.iteration_bound) as f64;

  while step < args.steps && (re * re + im * im) < boundary {
    let (new_re, new_im) = f(re, im, const_re, const_im);
    re = new_re;
    im = new_im;
    step += 1 
  }
  (step, re, im)
}

//==============================================================================
// Image generators
//==============================================================================

fn square_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
    let new_re = (re * re) - (im * im) + c_re;
    let new_im = (2.0 * re * im) + c_im;
    (new_re, new_im)
  }

fn cube_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
    let new_re = (re * re * re) - 3.0 * (re * im * im) + c_re;
    let new_im = (3.0 * re * re * im - im * im * im) + c_im;
    (new_re, new_im)
  }

fn ship_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
    let im = im.abs();
    let re = re.abs();
    let new_re = (re * re) - (im * im) + c_re;
    let new_im = (2.0 * re * im) + c_im;
    (new_re, new_im)
  }


fn compute_row(
  row : &mut [FracPoint],
  row_num : usize,
  args : FracArgs,
  iter_fun : impl Fn(f64, f64) -> (usize, f64, f64),
){
  for col_num in 0..args.field.pixel_size{
    let (re, im) = point_to_complex(&args.field, col_num, row_num);
    let (n, re, im) = iter_fun(re, im);
    row[col_num] = (n, re, im);
  }
}

pub fn compute_fractal(args : FracArgs) 
-> RawFrac
{
  let px_size = args.field.pixel_size;
  let mut matrix = vec![(0, 0., 0.); px_size * px_size];
  let iteration_fn = match args.iteration_style {
    IterationStyle::Julia => {julia_iterate}
    IterationStyle::Mandelbrot => {mandelbrot_iterate}
  };
  let iterator_fn = match args.iterator_kind {
    IteratorKind::Square => {square_iterator}
    IteratorKind::Cube => {cube_iterator}
    IteratorKind::Ship => {ship_iterator}
  };
  {// Mutable borrow scope
  let rows : Vec<(usize, &mut [(usize, f64, f64)])> = 
  matrix.chunks_mut(px_size).enumerate().collect();

  let iter_fun = |re, im|{iteration_fn((re, im), iterator_fn, &args)};
  rows.into_par_iter()
  .for_each(|(row_num, row)|{compute_row(row, row_num, args, iter_fun)});
  }

  matrix
}
