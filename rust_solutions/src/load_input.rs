use std::fs::read_to_string;

pub fn load_input(day: usize) -> Result<Vec<String>, String> {
    let file: String = format!("../inputs/day_{day}.txt");
    let contents: String = match read_to_string(file) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Failed to read input for day {day}: {e:?}"))
    };

    Ok(contents.lines().map(|x| x.to_string()).collect::<Vec<String>>())
}
