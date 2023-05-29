/*
* Given n pairs of parentheses, write a function to generate all
* combinations of well-formed parentheses.
*
* n = 1; ["()"]
* n = 2; ["(()), ()()"]
* n = 3; ["((()))", "()(())", "(()())", "(())()", "()(())", "()()()"]
*
*/

mod well_formed_paren {
    use std::collections::HashMap;

    fn gen_paren_helper(n: i32, cache: &mut HashMap<i32, Vec<String>>) -> Vec<String> {
        if n <= 0 {
            return vec!["".to_string()];
        }
        if n == 1 {
            return vec!["()".to_string()];
        }
        if let Some(val) = cache.get(&n) {
            return val.to_vec();
        }
        let mut res: Vec<String> = Vec::new();
        for k in 0..n {
            let res_k = gen_paren_helper(k, cache);
            let res_n_minus_k = gen_paren_helper(n - k - 1, cache);
            for str_k in res_k {
                for str_n_k in res_n_minus_k.iter() {
                    res.push(format!("({str_k}){str_n_k}"));
                }
            }
        }
        cache.insert(n, res.clone());
        res
    }

    pub fn gen_paren(n: i32) -> Vec<String> {
        let mut cache: HashMap<i32, Vec<String>> = HashMap::new();
        gen_paren_helper(n, &mut cache)
    }
}

fn main() {
    let res1 = crate::well_formed_paren::gen_paren(4);
    println!("{:?}", res1);
}
