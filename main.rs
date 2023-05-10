use std::u128::MAX;

fn main() {
    let mut base: u128 = 0;
    let mut buffer: u128;
    let mut adder: u128;
    let mut rev_adder: u128;
    let mut resulter: u128;
    let mut temp: u128;

    while base < MAX {
        if base % 1_000_000_000 == 0 {
            println!("Testing {}+", base);
        }

        buffer = base;
        adder = 0;

        while buffer > 0 {
            adder += buffer % 10;
            buffer /= 10;
        }

        resulter = adder;
        temp = adder;
        rev_adder = 0;

        while temp > 0 {
            if temp != adder {
                rev_adder *= 10;
            }
            rev_adder += temp % 10;
            temp /= 10;
        }

        resulter = resulter.overflowing_mul(rev_adder).0;

        if base == resulter {
            println!(
                "===================== > Found {} = {} * {}",
                base, adder, rev_adder
            );
        }

        base += 1;
    }
}
