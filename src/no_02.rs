// Write a function that returns a string in which firstname is swapped with last name.

// Example(Input --> Output)

// "john McClane" --> "McClane john"

// ::<>はturbofishと言う
fn name_shuffler(s: &str) -> String {
    s.rsplit(" ").collect::<Vec<&str>>().join(" ")
}

fn name_shuffler(s: &str) -> String {
    let names: Vec<&str> = s.split(' ').collect();
    names[1].to_string() + " " + &names[0].to_string()
}
