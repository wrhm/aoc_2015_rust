use crate::util;

use std::time::Instant;

fn day_01_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let line = lines.first().unwrap();

    let mut flr = 0;
    let mut basement_pos = -1;
    for (i, c) in line.chars().enumerate() {
        if c == '(' {
            flr += 1;
        } else {
            flr -= 1;
            if basement_pos < 0 && flr < 0 {
                basement_pos = 1 + i as i32;
                // break;
            }
        }
    }
    let ans1 = flr;
    let ans2 = basement_pos;
    (ans1, ans2)
}

pub(crate) fn day_01(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_01_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 01: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_01_both_parts;

    #[test]
    fn unit_test() {
        let (ans1_a, _) = day_01_both_parts("(())");
        assert_eq!(ans1_a, 0);
        let (ans1_b, _) = day_01_both_parts("()()");
        assert_eq!(ans1_b, 0);
        let (ans1_c, _) = day_01_both_parts("(((");
        assert_eq!(ans1_c, 3);
        let (ans1_d, _) = day_01_both_parts("(()(()(");
        assert_eq!(ans1_d, 3);
        let (ans1_e, _) = day_01_both_parts("))(((((");
        assert_eq!(ans1_e, 3);
        let (ans1_f, _) = day_01_both_parts("())");
        assert_eq!(ans1_f, -1);
        let (ans1_g, _) = day_01_both_parts("))(");
        assert_eq!(ans1_g, -1);
        let (ans1_h, _) = day_01_both_parts(")))");
        assert_eq!(ans1_h, -3);
        let (ans1_i, _) = day_01_both_parts(")())())");
        assert_eq!(ans1_i, -3);
        let (ans1_j, ans2_j) = day_01_both_parts("()())");
        assert_eq!(ans1_j, -1);
        assert_eq!(ans2_j, 5);
    }
}
