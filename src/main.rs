mod report;
mod supplies;

fn main() {
    println!("Programming class in Texas");

    let party: u32 = 4;
    let days: u32 = 30;

    let food = supplies::food_needed (party, days, 3);

    report::print_report(party, days, food);
}
