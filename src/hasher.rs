
pub(crate) struct Hasher{
}

impl Hasher {
    pub fn new() -> Self {
        Hasher {        }
    }

    /// The function `hash` takes a string input `u`, calculates its MurmurHash3 64-bit hash value, and
    /// returns it as a 128-bit unsigned integer.
    /// 
    /// Arguments:
    /// 
    /// * `u`: The parameter `u` in the `hash` function is a `String` type representing the input string
    /// that will be hashed using the `murmur_oaat64` function.
    /// 
    /// Returns:
    /// 
    /// The `hash` function is returning a 128-bit unsigned integer (`u128`).
    pub(crate) fn hash(&mut self, u: String) -> u128 {
        self.murmur_oaat64(&u).into()
    }

    /// The `murmur_oaat64` function in Rust implements the MurmurHash3 64-bit algorithm for hashing a
    /// given string key.
    /// 
    /// Arguments:
    /// 
    /// * `key`: The `key` parameter is a reference to a string slice (`&str`) in the `murmur_oaat64`
    /// function.
    /// 
    /// Returns:
    /// 
    /// The `murmur_oaat64` function returns a `u64` value, which is the final hash value calculated
    /// based on the input `key` string.
    pub fn murmur_oaat64(&self, key: &str) -> u64 {
        let mut h: u64 = 525201411107845655;
        key.bytes().for_each(|byte| {
            h ^= byte as u64;
            h = h.wrapping_mul(0x5bd1e9955bd1e995);
            h ^= h >> 47;
        });
        h
    }
    
}


#[test]
pub fn test_collisions() {
    fn read_lines_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
        let file: std::fs::File = std::fs::File::open(file_path)?;
        let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();
    
        for line in std::io::BufRead::lines(reader) {
            lines.push(line?);
        }
    
        Ok(lines)
    }
    let file_path: &str = "src\\words.txt"; // Update this with your file path

    let mut collision: usize = 0;
    let mut dictionary: std::collections::HashMap<u128, String> = std::collections::HashMap::new();
    let mut hasher: Hasher = Hasher::new();
    match read_lines_from_file(file_path) {
        Ok(lines) => {
            for line in lines.clone() {
                let hash = hasher.hash(line.clone());
                if let Some(meaning) = dictionary.get(&hash) { 
                    if *meaning == line {
                        println!("Duplicate found");
                        continue;
                    }
                    collision += 1;
                } else {
                    dictionary.insert(hash, line.clone());
                }
                
                //println!("{}", line);
            }
            println!("Len: {}", lines.len());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    println!("Number of collisions: {}", collision);
    assert!(collision == 0);
}
