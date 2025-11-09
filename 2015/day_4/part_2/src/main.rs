use md5;

const INPUT: &str = "ckczppom";

fn main() {
    let mut x: u32 = 0;
    loop {
        let digest = format!("{:x}", md5::compute(format!("{}{}", INPUT, x)));
        if digest.starts_with("000000") {
            println!("lowest number is: {}", x);
            break;
        }
        x += 1;
    }
}
