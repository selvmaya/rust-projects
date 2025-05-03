const REFERENCE: &str = "uux fwnqeefæzr bq eayr xareah";
const NEXT: &str = "abcdefghijklmnopqrstuvwxyzæøå";

fn main() {
    let right_string = (0..NEXT.len()).find_map(|i| {
        let new_string = REFERENCE
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let my_index = NEXT.find(c).unwrap() as u32;
                    let shifted = my_index + i as u32;
                    let recenter = (shifted - 'a' as u32) % NEXT.len() as u32;
                    char::from_u32(recenter + 'a' as u32).unwrap()
                } else {
                    c
                }
            })
            .collect::<String>();
        if new_string[..3] == "ddc"[..3] {
            Some(new_string)
        } else {
            None
        }
    });
    match right_string {
        Some(right) => println!("Found a new: {}", right),
        None => println!("Nothing new was found."),
    }
}
