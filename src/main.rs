use std::{fs, error::Error, path::Path, io::prelude::*, };


fn read_file(file_path: &str) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(why)    => Err(why.description().to_string()),
    }
}

fn main() {
    let path = Path::new("out.txt");
    let display = path.display();
    let raw = match read_file("list.txt") {//.split("\n").collect::<Vec<&str>>();
        //Ok(content) => content.split("\n").collect::<Vec<&str>>(),
        Ok(content) => content,
        Err(why)    => panic!("{}", why),
    };
    let mut out = String::new();
    for line in raw.split("\n").collect::<Vec<&str>>() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        let ip = match split_line.len() {
            //4 => split_line[2],
            4 => out.push_str(format!("route {}/32 reject;\n", split_line[2]).as_str()),
            _ => {}
        };

    }
    
    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(out.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
