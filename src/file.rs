use crate::config::QueryDetails;

pub fn file_search(contents: &String, user_input: &QueryDetails) -> String {
    let buffer = contents.lines();
    let mut result = String::new();
    let query = String::clone(&user_input.query);

    for line in buffer {
        if line.contains(&query) {
            result.push_str(line);
            result.push_str("\n");
        }
    }

    print!("{}", result);
    result
}


#[cfg(test)]
mod tests {

}