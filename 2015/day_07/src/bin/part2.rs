use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut changed: bool = true;
    let mut vire_set: HashMap<String, u16> = HashMap::new();
    vire_set.insert("b".to_string(), 956);

    while changed {
        changed = false;

    // let input = include_str!("./test.txt");
    let input = include_str!("./input.txt");
    let lines = input.lines();

    // let mut vire_set: HashMap<String, u16> = HashMap::new();

    for line in lines {
        if line.contains("OR") {
            // println!("performing OR");
            let rx = Regex::new(r"(\w+) OR (\w+) -> (\w+)").unwrap();
            if let Some(value) = rx.captures(line) {
                let vire_a: String = value[1].parse().unwrap();
                let vire_b: String = value[2].parse().unwrap();
                let vire_to: String = value[3].parse().unwrap();

                let mut vire_a_value: u16 = 0;
                if let Some(value) = vire_set.get_mut(&vire_a) {
                    vire_a_value = *value;
                }
                let mut vire_b_value: u16 = 0;
                if let Some(value) = vire_set.get_mut(&vire_b) {
                    vire_b_value = *value;
                }

                let mut new_signal_value: u16 = vire_a_value | vire_b_value;
                if let Some(value) = vire_set.get_mut(&vire_to) {
                    if &vire_to == "b" {
                        new_signal_value = 956;
                    }
                    *value = new_signal_value;
                    changed = true;
                } else {
                    vire_set.insert(vire_to.to_string(), new_signal_value);
                    changed = true;
                }                
            }
        }
        else if line.contains("AND") {
            // println!("performing AND");
            let rx = Regex::new(r"(\w+) AND (\w+) -> (\w+)").unwrap();
            if let Some(value) = rx.captures(line) {
                let vire_a: String = value[1].parse().unwrap();
                let vire_b: String = value[2].parse().unwrap();
                let vire_to: String = value[3].parse().unwrap();

                let mut vire_a_value: u16 = 0;
                if let Ok(value) = vire_a.parse::<u16>() {
                    vire_a_value = value;
                } else {
                    if let Some(value) = vire_set.get_mut(&vire_a) {
                        vire_a_value = *value;
                    }
                }

                let mut vire_b_value: u16 = 0;
                if let Some(value) = vire_set.get_mut(&vire_b) {
                    vire_b_value = *value;
                }

                let mut new_signal_value: u16 = vire_a_value & vire_b_value;
                if let Some(value) = vire_set.get_mut(&vire_to) {
                    if &vire_to == "b" {
                        new_signal_value = 956;
                    }
                    *value = new_signal_value;
                    changed = true;
                } else {
                    vire_set.insert(vire_to.to_string(), new_signal_value);
                    changed = true;
                }                
            }
        }
        else if line.contains("NOT") {
            // println!("performing NOT");
            let rx = Regex::new(r"NOT (\w+) -> (\w+)").unwrap();
            if let Some(value) = rx.captures(line) {
                let vire_from: String = value[1].parse().unwrap();
                let vire_to: String = value[2].parse().unwrap();

                let mut new_signal_value: u16 = 0;
                if let Some(value) = vire_set.get_mut(&vire_from) {
                    new_signal_value = !*value;
                }
                if let Some(value) = vire_set.get_mut(&vire_to) {
                    if &vire_to == "b" {
                        new_signal_value = 956;
                    }
                    *value = new_signal_value;
                    changed = true;
                } else {
                    vire_set.insert(vire_to.to_string(), new_signal_value);
                    changed = true;
                }                
            }
        }
        else if line.contains("LSHIFT") {
            // println!("performing LSHIFT");
            let rx = Regex::new(r"(\w+) LSHIFT (\d+) -> (\w+)").unwrap();
            if let Some(value) = rx.captures(line) {
                let signal_value: u16 = value[2].parse().unwrap();
                let vire_from: String = value[1].parse().unwrap();
                let vire_to: String = value[3].parse().unwrap();

                let mut new_signal_value: u16 = 0;
                if let Some(value) = vire_set.get_mut(&vire_from) {
                    new_signal_value = *value << &signal_value;
                }
                if let Some(value) = vire_set.get_mut(&vire_to) {
                    if &vire_to == "b" {
                        new_signal_value = 956;
                    }
                    *value = new_signal_value;
                    changed = true;
                } else {
                    vire_set.insert(vire_to.to_string(), new_signal_value);
                    changed = true;
                }
            }
        }
        else if line.contains("RSHIFT") {
            // println!("performing RSHIFT");
            let rx = Regex::new(r"(\w+) RSHIFT (\d+) -> (\w+)").unwrap();
            if let Some(value) = rx.captures(line) {
                let signal_value: u16 = value[2].parse().unwrap();
                let vire_from: String = value[1].parse().unwrap();
                let vire_to: String = value[3].parse().unwrap();

                let mut new_signal_value: u16 = 0;
                if let Some(value) = vire_set.get_mut(&vire_from) {
                    new_signal_value = *value >> &signal_value;
                }
                if let Some(value) = vire_set.get_mut(&vire_to) {
                    if &vire_to == "b" {
                        new_signal_value = 956;
                    }
                    *value = new_signal_value;
                    changed = true;
                } else {
                    vire_set.insert(vire_to.to_string(), new_signal_value);
                    changed = true;
                }
            }
        }
        else{
            // println!("performing direct signal change");
            let rx = Regex::new(r"(\w+) -> (\w+)").unwrap();
            if let Some(value) = rx.captures(line) {
                let val_1: String = value[1].parse().unwrap();
                let val_2: String = value[2].parse().unwrap();

                let mut new_signal_value: u16 = 0;
                if let Ok(value) = val_1.parse::<u16>() {
                    new_signal_value = value.clone();
                } {
                    if let Some(value) = vire_set.get_mut(&val_1) {
                        new_signal_value = value.clone();
                    }
                }

                if let Some(value) = vire_set.get_mut(&val_2) {
                    if &val_2 == "b" {
                        new_signal_value = 956;
                    }
                    *value = new_signal_value;
                    changed = true;
                } else {
                    vire_set.insert(val_2.to_string(), new_signal_value);
                    changed = true;
                }
            }
        }
    }

    // for vire in vire_set {
    //     println!("wire {} has value of {}", vire.0, vire.1);
    // }

    if let Some(value) = vire_set.get_mut("a") {
        println!("wire a has value of {}", *value);
    }
    }
}
