const WIDTH: usize = 6; // Половина висоти ромба

fn main() {
    let mut output = String::new();

    for i in 0..WIDTH {
        output += &format!("{:width$}{}\n", "", "*".repeat(2 * i + 1), width = WIDTH - i);
    }

    for i in (0..WIDTH - 1).rev() {
        output += &format!("{:width$}{}\n", "", "*".repeat(2 * i + 1), width = WIDTH - i);
    }

    print!("{}", output);
}
