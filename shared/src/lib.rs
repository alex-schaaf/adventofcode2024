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

pub struct Vec2d<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: PartialEq> Vec2d<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Vec2d {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), &'static str> {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn _idx_to_xy(&self, idx: usize) -> (usize, usize) {
        return (idx % self.width, idx / self.width);
    }

    pub fn find_first(&self, element: T) -> Option<(usize, usize)> {
        if let Some(index) = self.data.iter().position(|e| e == &element) {
            Some(self._idx_to_xy(index))
        } else {
            None
        }
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }
}
