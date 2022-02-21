use wasmedge_counter_interface::*;

fn main() {
    let mut counts: u32;
    counts = get_count();
    println!("Start with counter={:?}", counts);
    println!("Click three times");
    click();
    click();
    click();
    counts = get_count();
    println!("Now counter={:?}", counts);
    println!("Counter forwarded by 3");
    forward_by(3);
    counts = get_count();
    println!("Now counter={:?}", counts);
}
