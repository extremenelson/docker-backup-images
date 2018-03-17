use std::process::Command;

#[derive(PartialEq, Default, Clone, Debug)]
struct Image {
    name: String,
    tag: String,
    hash: String,
}

fn save() {
   let output = Command::new("docker")
            .args(&["images"])
            .output()
            .expect("Failed to execute process");

    let images = String::from_utf8_lossy(&output.stdout);
    let to_process = images.lines().skip(1);

    to_process
        .map(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();
            let parsed_name: Vec<&str> = columns[0].split("/").collect();
            Image {
                name: parsed_name[parsed_name.len()-1].to_string(),
                tag: columns[1].to_string(),
                hash: columns[2].to_string(),
            }
        })
        .for_each(|image| {
            let save_name: String = image.name + "-" + &image.tag + ".tgz";
            println!("Saving image with hash {} to {}", image.hash, save_name);
            let save = Command::new("docker")
                .args(&["save", "-o", &save_name, &image.hash])
                .output()
                .expect("Failed to save image.");

            println!("{}", String::from_utf8_lossy(&save.stdout));
        });
}

fn main() {
    save();
}
