pub fn parse_token(slice: Option<&&str>) -> String {
    match slice {
        Some(s) => s.to_string(),
        None => String::from(""),
    }
}

pub fn parse_token_list(slice: Option<&&str>) -> Vec<String> {
    match slice {
        Some(s) => {
            let pre: Vec<&str> = s.split(",").collect();

            pre.into_iter().map(|s| s.to_string()).collect()
        }
        None => vec![],
    }
}
