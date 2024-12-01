use std::fs;

pub fn load_file_str(filepath: &str) -> String {
    fs::read_to_string(filepath).expect("failed to load file to string")
}

pub fn load_file_lines(filepath: &str) -> Vec<String> {
    let file_str = load_file_str(filepath);
    file_str.split("\n").map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file_str_works() {
        let result = load_file_str("example.txt");
        assert_eq!(result, "hello\nworld")
    }

    #[test]
    fn test_load_file_lines_works() {
        let result = load_file_lines("example.txt");
        assert_eq!(result, vec!["hello", "world"]);
    }
}
