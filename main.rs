use std::u128::MAX;

fn main() {
    let mut base: u128 = 0;
    let mut buffer: u128;
    let mut adder: u128;
    let mut resulter: u128;

    while base < MAX {
        if base % 1000000 == 0 {
            println!("Testing {}+", base);
        }

        buffer = base;
        adder = 0;

        while buffer > 0 {
            adder += buffer % 10;
            buffer /= 10;
        }

        resulter = adder;

        let my_str: String = format!("{:?}", adder);
        let reversed = my_str.chars().rev().collect::<String>();
        adder = reversed.parse().unwrap();

        resulter = resulter.overflowing_mul(adder).0;

        if base == resulter {
            println!(
                "===================== > Found {} = {} * {}",
                base, my_str, reversed
            );
        }

        base += 1;
    }
}
