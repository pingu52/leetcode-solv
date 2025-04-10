impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::new();

        for i in 1..=n {
            let s = format!(
                "{}{}",
                (i % 3 == 0).then(|| "Fizz").unwrap_or(""),
                (i % 5 == 0).then(|| "Buzz").unwrap_or("")
            );

            let val = s
                .is_empty()
                .then(|| i.to_string())
                .unwrap_or(s);

            answer.push(val);
        }

        answer
    }
}
