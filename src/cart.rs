use crate::{case::Case, hasher::Hasher};

/// The `Cart` struct in Rust contains a `Hasher`, a vector of keys, and a vector of `Case` elements.
/// 
/// Properties:
/// 
/// * `hasher`: The `hasher` property in the `Cart` struct is of type `Hasher`. It is used to compute
/// hash values for the keys stored in the cart.
/// * `keys`: The `keys` property in the `Cart` struct is a vector of `u64` values. It is used to store
/// the keys associated with the elements in the cart.
/// * `elements`: The `elements` property in the `Cart` struct is a vector that stores elements of type
/// `Case`.
pub(crate) struct Cart {
    hasher: Hasher,
    keys: Vec<u64>,
    elements: Vec<Case>,
}

impl Cart {
    /// The `new` function in Rust initializes a new instance of a struct with empty vectors for keys
    /// and elements.
    /// 
    /// Returns:
    /// 
    /// A new instance of the struct with the `hasher` field initialized with a new `Hasher`, and empty
    /// `keys` and `elements` vectors.
    pub fn new() -> Self {
        Self { 
            hasher: Hasher::new(),
            keys: Vec::new(),
            elements: Vec::new() 
        }
    }
    /// The `insert` function in Rust inserts an element with a specified key into a data structure.
    /// 
    /// Arguments:
    /// 
    /// * `element`: The `element` parameter in the `insert` function represents the value that you want
    /// to insert into the data structure. In this case, it is a `String` type, which means it can hold
    /// a sequence of characters. When you call the `insert` function, you provide the `element
    /// * `key`: The `key` parameter in the `insert` function is a `String` type. It is used as a key to
    /// associate with the element being inserted into the data structure.
    pub fn insert(&mut self, element: String, key: String) {
        let ac_key: u128 = self.hasher.hash(key);
        self.keys.push(ac_key.try_into().unwrap());
        let mut case: Case = Case::new();
        case.set(ac_key.try_into().unwrap(), element);
        self.elements.push(case);
    }
    /// The function `get_primitive` retrieves a primitive value associated with a given key from a data
    /// structure.
    /// 
    /// Arguments:
    /// 
    /// * `key`: The `key` parameter is of type `u64`, which represents an unsigned 64-bit integer.
    /// 
    /// Returns:
    /// 
    /// The function `get_primitive` returns an `Option<Case>`.
    pub fn get_primitive(&self, key: u64) -> Option<Case>{
        let index = self.keys.iter().position(|&r| r == key);
        match index {
            Some(k) => Some(self.elements.get(k)),
            None => None,
        };
        None
    }
    /// This Rust function retrieves a value associated with a key from a data structure, returning an
    /// `Option` containing the value if the key is found.
    /// 
    /// Arguments:
    /// 
    /// * `key`: The `key` parameter in the `get` function is a `String` type, which represents the key
    /// used to retrieve a value from the data structure.
    /// 
    /// Returns:
    /// 
    /// The `get` function returns an `Option<Case>`.
    pub fn get(&mut self, key: String) -> Option<Case>{
        let ac_key = self.hasher.hash(key) as u64;
        let index = self.keys.iter().position(|&r| r == ac_key);
        match index {
            Some(k) => Some(self.elements.get(k)),
            None => None,
        };
        None
    }
    /// The `remove` function in Rust removes a key from a data structure based on its hashed value.
    /// 
    /// Arguments:
    /// 
    /// * `key`: The `key` parameter in the `remove` function is a `String` type, representing the key
    /// that needs to be removed from the data structure.
    pub fn remove(&mut self, key: String){
        let ac_key = self.hasher.hash(key) as u64;
        let index = self.keys.iter().position(|&r| r == ac_key);
        match index {
            Some(k) => {
                self.remove_unsafe(k)
            },
            None => return,
        };
    }
    /// The function `remove_primitive` removes a primitive value from a collection in Rust.
    /// 
    /// Arguments:
    /// 
    /// * `key`: The `key` parameter is the value of type `u64` that you want to remove from the list of
    /// keys in the data structure.
    pub fn remove_primitive(&mut self, key: u64){
        let index = self.keys.iter().position(|&r| r == key);
        match index {
            Some(k) => {
                self.remove_unsafe(k)
            },
            None => return,
        };
    }
    /// The `remove_unsafe` function removes an element at a specified index from two collections.
    /// 
    /// Arguments:
    /// 
    /// * `index`: The `index` parameter specifies the position of the element to be removed from the
    /// `elements` and `keys` collections.
    pub fn remove_unsafe(&mut self, index: usize) {
        self.elements.remove(index);
        self.keys.remove(index);
    }
    /// This Rust function prints the contents of a cart, including keys and elements.
    pub fn print(&self) {
        println!("Cart Contents:");
        println!("\tKeys\t\t\tElements");
        for i in 0..self.keys.len() {
            println!("{}) {}:\t\t{:?}", i, self.keys[i], self.elements[i].get_packet());
        }
    }
    
}

