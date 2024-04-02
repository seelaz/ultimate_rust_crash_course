use variables::greet;
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {

    let (missiles, ready) = (STARTING_MISSILES,READY_AMOUNT);
    //let test = 0;
    println!("Firing {} of my {} missiles...",ready,missiles);
    //missiles = missiles - ready;
    println!("{} missiles left",missiles - ready);
    //println!("Hello, world!");

    greet();
}
