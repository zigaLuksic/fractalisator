
#[derive(Copy, Clone)]
pub struct Field { 
  pub pixel_size : usize,
  pub re_center : f64, pub im_center : f64, 
  pub radius : f64}


#[derive(Copy, Clone)]
pub struct FracArgs{
  pub z_const : (f64, f64), 
  pub bound : usize, 
  pub steps : usize,
  pub steps_off : usize,
  }
