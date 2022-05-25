const MISSILE:i32 = 8;
const READY:i32 = 2;
const STARTING_MISSILES: i32 = 8;
const STARTING_AMOUNT: i32 = 2;
fn main() {
    let (mut missiles,  ready)  = (8, 2);
    println!("Firing {} of my {} missiles...", READY, MISSILE);
    missiles = missiles - ready;
    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("Congratulation you are a rust programmer!")

}
