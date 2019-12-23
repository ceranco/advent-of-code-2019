mod app;
use crate::app::*;

fn is_password_valid_p1(num: i32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    debug_assert_eq!(digits.len(), 6);

    let mut does_not_decrease = true;
    let mut same_adjacent_digits = false;

    for (idx, digit) in digits.iter().enumerate().skip(1) {
        let prev = digits[idx - 1];
        if *digit < prev {
            does_not_decrease = false;
            break;
        }

        if *digit == prev {
            same_adjacent_digits = true;
        }
    }

    does_not_decrease && same_adjacent_digits
}

fn is_password_valid_p2(num: i32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    debug_assert_eq!(digits.len(), 6);

    let mut does_not_decrease = true;
    let mut same_adjacent_digits = false;
    let mut has_valid_adjacent_digits_chain = false;
    let mut same_digit_chain_counter = 0;

    for (idx, digit) in digits.iter().enumerate().skip(1) {
        let prev = digits[idx - 1];
        if *digit < prev {
            does_not_decrease = false;
            break;
        }

        if *digit == prev {
            same_adjacent_digits = true;
            same_digit_chain_counter += 1;
        } else {
            if 0 < same_digit_chain_counter && same_digit_chain_counter < 2 {
                has_valid_adjacent_digits_chain = true;
            }
            same_digit_chain_counter = 0;
        }
    }
    if 0 < same_digit_chain_counter && same_digit_chain_counter < 2 {
        has_valid_adjacent_digits_chain = true;
    }

    does_not_decrease && same_adjacent_digits && has_valid_adjacent_digits_chain
}

fn main() {
    // get the options
    let opt: Opt = app().get_matches().into();

    // max sure the options are valid
    if opt.min > opt.max {
        println!("Invalid value for <max>: The number was smaller the <min>");
        std::process::exit(1)
    }

    // check how many numbers in the range are eligible passwords
    // TODO: this seems great for parallelization (maybe rayon?)
    let algo = if opt.repeat_limit {
        is_password_valid_p2
    } else {
        is_password_valid_p1
    };
    let num_valid = (opt.min..=opt.max).fold(0, |ctr, num| if algo(num) { ctr + 1 } else { ctr });

    println!(
        "Number of elgible passwords in range [{}, {}]: {}",
        opt.min, opt.max, num_valid
    );
}
