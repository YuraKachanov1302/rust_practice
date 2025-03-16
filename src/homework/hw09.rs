fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let n = ((n % len) + len) % len; // Обробляємо негативні та надлишкові зсуви
    let split_point = len as usize - n as usize;

    let rotated = format!("{}{}", &s[split_point..], &s[..split_point]);
    rotated
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(
            rotate(s.clone(), *n),
            exp.to_string()
        );
    });
}
