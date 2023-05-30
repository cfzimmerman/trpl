/*
* Note, along with some other crates in this repo, I'm just using these
* LeetCode questions to practice syntax in-between chapters.
*
* Write a function to find the longest common prefix string amongst an array of strings.
* If there is no common prefix, return an empty string "".
*
*/

fn longest_common_prefix(strs: Vec<String>) -> String {
    let first: &String = strs
        .first()
        .expect("invariant violated, strs should have length >= 1");
    let mut res = String::new();
    let mut index = 0;
    loop {
        let temp: char = match first.chars().nth(index) {
            Some(val) => val,
            None => break,
        };
        if !(strs.iter().all(|st| match st.chars().nth(index) {
            None => false,
            Some(ch) => ch == temp,
        })) {
            break;
        }
        res.push(temp);
        index += 1;
    }
    res
}

fn main() {
    let inputs = vec![
        vec!["dog", "racecar", "car"],
        vec!["flower", "flow", "flight"],
    ];
    for input in inputs.iter() {
        println!(
            "{:?}",
            longest_common_prefix(input.iter().map(|str| str.to_string()).collect())
        );
    }
}
