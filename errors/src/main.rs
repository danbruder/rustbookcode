fn main() {
    //panic!("crash and burn!");
    //let v = vec![1, 2, 3];

    //v[99];
    //use std::fs::File;

    //let file = File::open("hello.txt");

    //let f = match file {
    //Ok(file) => file,
    //Err(error) => panic!("There was a problem opening the file: {:?}", error),
    //};

    //use std::fs::File;
    //use std::io::ErrorKind;

    //let f = File::open("hello.txt");

    //let f = match f {
    //Ok(file) => file,
    //Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //Ok(fc) => fc,
    //Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //},
    //Err(error) => panic!("There was a problem opening the file: {:?}", error),
    //};
    //use std::fs::File;
    //let f = File::open("hello.txt")?;

    // Guess
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Gues must be in range");
            }

            Guess { value }
        }
        pub fn value(&self) -> u32 {
            self.value
        }
    }
}
