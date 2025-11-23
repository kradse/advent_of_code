use md5;

fn main() {
    let secret = "yzbqklnj";

    let mut searching = true;
    let mut index = 0;
    while searching {
        index += 1;

        let value = format!("{}{}", secret, index);
        let converted = md5::compute(value.clone());
        let hex = format!("{:?}", converted);

        if hex.starts_with("00000") {
            println!("Found: {:?}", value.clone());
            searching = false;
        }
    }
}
