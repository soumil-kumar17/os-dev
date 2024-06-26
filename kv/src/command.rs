use std::collections::HashMap;

pub fn process_command(command: &str, cmd_table: &mut HashMap<u32, String>) {
    let mut parts = command.split(',');

    if let Some(cmd) = parts.next() {
        match cmd.trim() {
            "put" => put_command(parts, cmd_table),
            "get" => get_command(parts, cmd_table),
            "delete" => delete_command(parts, cmd_table),
            "clear" => clear_command(cmd_table),
            "all" => all_command(cmd_table),
            _ => eprintln!("Invalid command: {}", command),
        }
    }
}

//need the yielded iterator value(&str) to live atleast as long as the fn hence lifetime annotation.
fn put_command<'a, I>(mut args: I, cmd_table: &mut HashMap<u32, String>)
where
    I: Iterator<Item = &'a str>,
{
    if let (Some(key), Some(value)) = (args.next(), args.next()) {
        match key.parse::<u32>() {
            Ok(key) => {
                cmd_table.insert(key, value.to_string());
                println!("Inserted: key {:?} -> value {:?}", key, value);
            }
            Err(_) => eprintln!("Invalid key for 'put' command: key must be a u32"),
        }
    } else {
        eprintln!("Invalid 'put' command format");
    }
}

fn get_command<'a, I>(mut args: I, cmd_table: &HashMap<u32, String>)
where
    I: Iterator<Item = &'a str>,
{
    if let Some(key) = args.next() {
        match key.parse::<u32>() {
            Ok(key) => match cmd_table.get(&key) {
                Some(value) => println!("Retrieved: key {:?} -> value {:?}", key, value),
                None => println!("Key {:?} not found", key),
            },
            Err(_) => eprintln!("Invalid key for 'get' command: key must be a u32"),
        }
    } else {
        eprintln!("Invalid 'get' command format");
    }
}

fn delete_command<'a, I>(mut args: I, cmd_table: &mut HashMap<u32, String>)
where
    I: Iterator<Item = &'a str>,
{
    if let Some(key) = args.next() {
        match key.parse::<u32>() {
            Ok(key) => {
                if cmd_table.remove(&key).is_some() {
                    println!("Deleted: key {:?}", key);
                } else {
                    println!("Key {:?} not found", key);
                }
            }
            Err(_) => eprintln!("Invalid key for 'delete' command: key must be a u32"),
        }
    } else {
        eprintln!("Invalid 'delete' command format");
    }
}

fn clear_command(cmd_table: &mut HashMap<u32, String>) {
    cmd_table.clear();
    println!("Cleared all entries");
}

fn all_command(cmd_table: &HashMap<u32, String>) {
    println!("All entries:");
    for (key, value) in cmd_table {
        println!("key {:?} -> value {:?}", key, value);
    }
}
