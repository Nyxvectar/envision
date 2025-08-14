/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/13/2025
 */

fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    println!("a = {}, b = {}", a, b);
    b = true;
    assert_eq!(a, b);
    panic!("手动panic")
}
