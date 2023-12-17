fn main() {
    let test_input1 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let result1 = answer(test_input1);
    assert_eq!(result1, 1320);
    println!("Test success! Here\'s the answer:");
    let answer = answer(include_str!("../../input.txt"));
    println!("{}", answer);
}

fn answer(input: &str) -> u32 {
    let list = input.split(",");
    let mut answer = 0;
    for string in list {
        let mut curr_val = 0;
        for ch in string.bytes() {
            if ch == 10 {
                //newline
                continue;
            }
            curr_val += ch as u32;
            curr_val *= 17;
            curr_val %= 256;
        }
        println!("{}", curr_val);
        answer += curr_val;
    }
    answer
}
