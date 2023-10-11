use f256::f256;

fn main() {
    let Some(denominator_string) = std::env::args().skip(1).next() else {
        return eprintln!("Program requires a limit to the denominator");
    };

    let Ok(denominator_limit) = u128::from_str_radix(denominator_string.as_str(), 10) else {
        return eprintln!("Denominator must be a positive integer");
    };

    if denominator_limit < 2 {
        return println!("1");
    }

    let mut total: f256 = 1.into();
    let mut add_or_sub = false;
    for i in 2..=denominator_limit {
        println!("{}1/{i}", if add_or_sub { "+" } else { "-" });
        total = if add_or_sub { total + f256::from(1)/f256::from(i) } else { total - f256::from(1)/f256::from(i) };
        add_or_sub = !add_or_sub; // xor would work too but no boolean bitwise ops
    }

    println!("{total}");
}
