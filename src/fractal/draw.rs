//==============================================================================
// Open crates and libraries
//==============================================================================
use rayon::prelude::*;

use fractal::definitions::*;

//==============================================================================
// Pixels to complex
//==============================================================================

/// Assuming `i` and `j` are indices within `field.pixel_size`, the function
/// returns the complex number that the pixel `(i, j)` represents in `field`.
pub fn point_to_complex (field : &Field, i : usize, j : usize) -> (f64, f64) {
  let Field {pixel_size, center_re, center_im, radius} = field;
  let size = 2.0 * radius;

  let x_rel = i as f64 / *pixel_size as f64;
  let re = (center_re - radius) + size * x_rel;

  let y_rel = j as f64 / *pixel_size as f64;
  let im = (center_im - radius) + size * y_rel;

  (re, im)
}

//==============================================================================
// Fractal generators
//==============================================================================

/// Accepts a starting point of the iteration `z = (z_re, z_im)` and the
/// iteration constant `c = (c_re, c_im)`. Using the function `f` it iterates
/// `z = f(z, c)` until a termination condition is met (`max_steps` or the
/// absolute value of `z` goes beyond `iter_bound`).
fn iterate_point (
  z_re : f64, z_im : f64,
  c_re : f64, c_im : f64,
  max_steps : usize, iter_bound : f64,
  f : impl Fn(f64, f64, f64, f64) -> (f64, f64) )
-> FracPoint {

  // Loop mutables
  let mut step = 0; 
  let mut re = z_re;
  let mut im = z_im;

  // Iterate `z = f(z, c)`
  while step < max_steps && (re * re + im * im) < iter_bound {
    let (new_re, new_im) = f(re, im, c_re, c_im);
    re = new_re;
    im = new_im;
    step += 1 
  }

  (step, re, im)
}

/// Calculates a Julia style iteration for the function `f` using fractal
/// arguments `args` (which include the constant `c`). The starting point of
/// iteration is `z`.
fn julia_iterate (
  z : (f64, f64), 
  f : impl Fn(f64, f64, f64, f64) -> (f64, f64), 
  args : &FracArgs) 
-> FracPoint {

  let (z_re, z_im) = z;
  let (c_re, c_im) = (args.c_re, args.c_im);

  iterate_point(z_re, z_im, c_re, c_im, args.steps, args.iter_bound, f)
}

/// Calculates a Mandelbrot style iteration for the function `f` using fractal
/// arguments `args`. The starting point of iteration is `(0., 0.)` and the
/// constant `c` is set to `z`.
fn mandelbrot_iterate (
  z : (f64, f64), 
  f : impl Fn(f64, f64, f64, f64) -> (f64, f64), 
  args : &FracArgs) 
-> FracPoint {

  let (z_re, z_im) = (0.0, 0.0);
  let (c_re, c_im) = z;
  
  iterate_point(z_re, z_im, c_re, c_im, args.steps, args.iter_bound, f)
}

//==============================================================================
// Image generators
//==============================================================================

/// Iteration function `f(z, c) = z ^ 2 + c`
fn square_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
    let new_re = (re * re) - (im * im) + c_re;
    let new_im = (2.0 * re * im) + c_im;
    (new_re, new_im)
  }

/// Iteration function `f(z, c) = z ^ 3 + c`
fn cube_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
    let new_re = (re * re * re) - 3.0 * (re * im * im) + c_re;
    let new_im = (3.0 * re * re * im - im * im * im) + c_im;
    (new_re, new_im)
  }

/// Iteration function `f(z, c) = z ^ -2 + c`
fn inverse_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
  let size = (re * re + im * im).max(f64::MIN_POSITIVE);
  square_iterator(re / size, im / size, c_re, c_im)
}

/// Iteration function for the famous Burning Ship fractal. The function is
/// `f((re, im), c) = (|re|, |im|) ^ 2 + c`.
fn ship_iterator(re : f64, im : f64, c_re : f64, c_im : f64) -> (f64, f64) {
    let im = im.abs();
    let re = re.abs();
    let new_re = (re * re) - (im * im) + c_re;
    let new_im = (2.0 * re * im) + c_im;
    (new_re, new_im)
  }

/// Function that sets the values of a mutable matrix row `row` to fractal
/// points obtained from `iter_fun` using fractal arguments `args`. The row
/// number is required to calculate complex points that the row represents.
/// 
/// This function is run in parallel on all matrix rows to speed up rendering.
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

/// Renders raw fractal data as specified by fractal arguments `args`. The rows
/// are calculated in parallel.
pub fn compute_fractal(args : FracArgs) -> RawFrac {
  let px_size = args.field.pixel_size;
  let mut matrix = vec![(0, 0., 0.); px_size * px_size];

  let iteration_fn = match args.iteration_style {
    IterationStyle::Julia => {julia_iterate}
    IterationStyle::Mandelbrot => {mandelbrot_iterate}
  };
  let iterator_fn = match args.iterator_kind {
    IteratorKind::Square => {square_iterator}
    IteratorKind::Cube => {cube_iterator}
    IteratorKind::Inverse => {inverse_iterator}
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
