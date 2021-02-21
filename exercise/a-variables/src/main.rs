const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let _fr = 0;
    let mut missiles = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
    arr();

    let d = Data::new(1.0, 2.0);
    d.sum();
    println!("{:?}", d)
}

fn arr() {
    let mut buf = [0; 37];
    let mut i = 0;

    i = i + 3;

    buf[i] = 1;
    println!("{}", buf[i])
}
#[derive(Debug)]
struct Data {
    x: f32,
    y: f32,
}
impl Data {
    pub fn sum(&self) -> f32 {
        self.x + self.y
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
