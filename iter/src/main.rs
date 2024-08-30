
fn print_elements(elements: &Vec<String>) {
    elements.iter().for_each(|el| println!("{}", el)); 
}

fn shorten_string(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(2));
}

fn to_uppercase(elements: &mut Vec<String>) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(fallback.to_string(), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        "red".to_string(),
        "green".to_string(),
        "blue".to_string()
    ];

    // shorten_string(&mut colors);
    // let uppercase = to_uppercase(&mut colors);
    // print_elements(&uppercase);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded);

    let found_color = find_color_or(&colors, "reasd", "orange");
    println!("{}", found_color);
}
