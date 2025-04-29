use std::io;

fn main() {

    let mut stop = true;
    while stop {

        println!("Choose an option:");
    println!("1: Add something to a string.");
    println!("2: Remove something to a string.");
    println!("3: Lowercase the string.");
    println!("4: Uppercase the string.");
    println!("5: Ocorrence in string.");
    println!("6: Repeat letter according order in alphabet.");
    println!("Please, type an option.");
    println!("Type 0 to stop.")

    let mut input= String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro");

    let input = input.parse().unwrap();

    match input {

        0 => {
            stop = false;
            panic!;
        }

        1 => {
            add_string();
        }

        2 => {
            let mut result = remove_string();
            println!("Final String: {}", result);
        }

        3 => {
            let mut str = String::new();
            io::stdin()
                .read_line(&mut str)
                .expect("Erro");
            
            let str = str.to_lowercase();
            println!("Final String: {}", str);
        }

        4 => {
            let mut str = String::new();
            io::stdin()
                .read_line(&mut str)
                .expect("Erro");

            let str = str.to_uppercase();
            println!("Final String: {}", str);
        }

        5 => {
            let mut result = occur_string();
            println!("Final String: {}", result);
        }

        6 => {
            let mut result = repeat_string();
            println!("Final String: {}", result);
        }
    } 
    } 

}



fn add_string() {

    println!("Enter a string:");
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Erro");
    println!("Enter what you wanna add to the string");
    let mut add = String::new();
    io::stdin()
        .read_line(&mut add)
        .expect("Erro");
    let mut option = String::new();
    println!("Type the index of string for the modification (from 0: the first one, to the length of word - 1):");
    let mut index =String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Erro");
    let index = index.parse().unwrap();

    println!("Final string: {}{}{}", str[0..index], add, str[index..str.len()]);
}


fn occur_string() {

    println!("Enter a string: ");
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Erro");

    println!("Enter what you wanna search in the string: ");
    let mut sub = String::new();
    io::stdin()
        .read_line(&mut sub)
        .expect("Erro");

    let mut op = String::new();
    println!("Type '1' if you wanna know how many times it occurs in string.");
    println!("Type '2' if you wanna know if it occurs in string.");
    println!("Type '0' to stop.");


    io::stdin()
        .read_line(&mut op)
        .expect("Erro");
    let op = op.parse().unwrap();

    if op == 1 {
        let result: i32 = string_search_count(str, sub);
        println!("It occurs {} times.", result);
        }
    else if op == 2 {

        let result = string_search(str, sub);

        if result == true {
            println!("It's in the string.");
        else
            println!("It's not in the string.");
        }
    }
    else if op == 0 {
        break;
    }
    else {
        println("ERROR: Not an option.");
        break;
    }
}

fn string_search(str: String, sub: String) -> bool {

    let len_sub = sub.len();
    let len_str = str.len();
    
    for i in 0..len_str {
        
        for j in 0..len_sub()
    }
}

