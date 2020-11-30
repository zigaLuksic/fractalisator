
#[derive(Copy, Clone)]
pub struct Field { 
  pub pixel_size : usize,
  pub re_center : f64, 
  pub im_center : f64, 
  pub radius : f64}

impl Default for Field {
  fn default() -> Self { Field{ 
      pixel_size : 1000, radius : 2.0,
      re_center : 0.0 , im_center : 0.0}
  }
}

#[derive(Copy, Clone)]
pub struct FracArgs{
  pub field : Field,
  pub z_const : (f64, f64), 
  pub bound : usize, 
  pub steps : usize,
  }

impl Default for FracArgs{
  fn default() -> Self { FracArgs{
    field : Field::default(), steps : 256, 
    bound : 3, z_const : (-0.96656, 0.1225)}
}}


pub type FracPoint = (usize, f64, f64);
pub type RawFrac = Vec<FracPoint>;
pub type FracImage = Vec<u8>;

#[derive(Copy, Clone)]
pub struct ImageArgs{
  pub steps_off : usize,
  }
