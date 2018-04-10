use std::fs;

fn main() {
    let paths = fs::read_dir("../").unwrap();

    for path in paths {
        let mut newpath = path.unwrap().path();
        newpath.push(".git");
        if newpath.exists() {
            fs::remove_dir_all(newpath).expect("Whoops");
        }
    }
}
