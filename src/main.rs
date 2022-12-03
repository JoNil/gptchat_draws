static INPUT: &str = "A Y
B X
C Z";

pub fn a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => panic!("{line}"),
        })
        .sum::<i32>()
}

pub fn main() {
    let res = a(INPUT);

    println!("Res: {}", res);
}

#[test]
fn test_a() {
    assert_eq!(a(INPUT), 15);
}
