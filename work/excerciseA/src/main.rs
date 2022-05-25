const STARTING_MISSILES: i32 = 8;
const STARTING_AMOUNT: i32 = 2;
fn main() {
    let (mut missiles,  ready)  = (STARTING_MISSILES, STARTING_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("Congratulation you are a rust programmer!")

}
