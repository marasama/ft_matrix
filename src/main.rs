use zort::vector::Vector;

fn main() {
   let a = Vector::from([2.111, 2.431, 5.124, 7.535, 12.654, 234.671]);

   println!("{:.3}", a);
   println!("{:.3?}", a);
}
