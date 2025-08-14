/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/13/2025
 */

fn main() {
    let a = 255_u8;
    let b = a.wrapping_add(1);
    let c = 0.1_f32;
    let mut d = 0.2_f32;
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("{}",abc.2);
    println!("{}",b);
    println!("{}",c+d);
}
