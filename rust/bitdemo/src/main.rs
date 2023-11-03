use bitdemo::rsqrt;

fn main() {
    let number = 763.25f32;
    let bits = number.to_bits();
    println!("bits:  {bits:b}");
    println!("f32:   {number}");
    println!("u32:   {bits}");

    let rsqrt = rsqrt(number);
    println!("rsqrt: {rsqrt}");
}
