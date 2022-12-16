use std::collections::HashSet;

pub struct Datastream {
    message: String,
}

impl Datastream {
    pub fn new(message: &str) -> Datastream {
        Datastream {
            message: String::from(message),
        }
    }

    pub fn first_packet(&self) -> Option<usize> {
        let packet_group = 4;
        let index = self.index_of_first_unique_chars(packet_group)?;
        Some(index + packet_group)
    }

    pub fn first_message(&self) -> Option<usize> {
        let packet_group = 14;
        let index = self.index_of_first_unique_chars(packet_group)?;
        Some(index + packet_group)
    }

    fn index_of_first_unique_chars(&self, amount: usize) -> Option<usize> {
        for i in 0..(self.message.len() - amount + 1) {
            let slice = &self.message[i..i + amount];

            if Datastream::has_unique_chars(slice) {
                return Some(i);
            }
        }

        None
    }

    fn has_unique_chars(string: &str) -> bool {
        let mut has_unique_chars = true;

        let mut set = HashSet::new();

        for chr in string.chars() {
            has_unique_chars = has_unique_chars & set.insert(chr);
        }

        has_unique_chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_chars() {
        assert!(Datastream::has_unique_chars("abcd"));
        assert!(Datastream::has_unique_chars("abcde"));
        assert!(Datastream::has_unique_chars("abcdefghi"));
        assert!(!Datastream::has_unique_chars("bbcde"));
        assert!(!Datastream::has_unique_chars("fejkxnmk"));
        assert!(!Datastream::has_unique_chars("hehe"));
    }

    #[test]
    fn split() {
        let ds = Datastream::new("abcdefg");
        assert_eq!(ds.index_of_first_unique_chars(4), Some(0));
    }

    #[test]
    fn first_packet() {
        let ds = Datastream::new("aaabcdefg");
        assert_eq!(ds.first_packet(), Some(6));
    }

    #[test]
    fn first_message() {
        assert_eq!(
            Datastream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
                .first_message()
                .unwrap(),
            19
        );
    }
}
