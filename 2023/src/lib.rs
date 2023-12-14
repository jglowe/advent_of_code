///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                  _ _ _
//                                 | (_) |
//                                 | |_| |__
//                                 | | | '_ \
//                                 | | | |_) |
//                                 |_|_|_.__/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for library functions for advent of code 2023
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn read_file(filename: &str) -> String {
    match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

