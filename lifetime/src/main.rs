fn next_language<'a>(languages: &'a[String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language(lanaguages: &[String]) -> &str {
    lanaguages.last().unwrap()
}

fn longest_language<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let languages = vec![
        "rust".to_string(),
        "go".to_string(),
        "typescript".to_string(),
    ];

    let result = longest_language("go", "rust");
    println!("{}", result);
}
