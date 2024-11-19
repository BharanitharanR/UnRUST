fn main() {

// Integer - with wrapping
let int_dt:u32= 2;
let  int_dt:u32= int_dt.pow(32);

println!("This will show 0 -  {int_dt}");

// Float  - with wrapping
let flt_dt:f32= 2.00;
let  flt_dt:f32= flt_dt.powf(32.0);
println!("This will show 0 -  {flt_dt}");
}
