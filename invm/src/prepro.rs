pub fn filter(query: String) -> String {
    let mut new_query = String::new();
    let mut in_comment = false;
    for i in 0..query.len() {

        if query.get(i..=i) == Some(";") {
            in_comment = true;
        }

        let c = query.get(i..i+1).unwrap();
        if c == "\n" {
            in_comment = false;
        }

        if !in_comment {
            new_query.push_str(c);
        }
    }
    new_query
}
