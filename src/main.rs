use nrnc::reaclib::parse_reaclib_2;

fn main() {
    let mut buf = include_bytes!("data/reaclib") as &[u8];
    println!("{:?}", parse_reaclib_2(&mut buf));
}
