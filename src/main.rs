use std::io;

fn main() {
    let mut user_temp = String::new();
    let mut user_convert = String::new();

    loop{
        println!("What would you like to convert to F or C, Press Q to quit?");
        user_convert.clear();
        io::stdin()
            .read_line(&mut user_convert)
            .expect("Incorrect input");

        user_convert = user_convert.trim().to_string().to_uppercase();

        if user_convert == "Q" {
            println!("Bye!");
            break;
        }else if user_convert != "C" && user_convert != "F" {
            println!("Input F, f, c or C");
            continue;
        }

        println!("Input number to convert to {user_convert}");
        user_temp.clear();
        io::stdin()
            .read_line(&mut user_temp)
            .expect("Incorrect input");

        let user_temp: i32 = match user_temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_convert == "F" {
            temp_converter(user_temp, &user_convert)
        }else if user_convert == "C"{
            temp_converter(user_temp, &user_convert)
        }else{
            println!("Input F, f, c or C");
            break;
        }

        user_convert = "".to_string();

    };

}

fn temp_converter(temp: i32, conv: &str){
    let temp_num: i32;

    if conv == "F" {
        temp_num = (temp * (9/5)) + 32;
        println!("The Celsius number in Fahrenheit is {temp_num}");
    }else{
        temp_num = (temp - 32) / (9 / 5);
        println!("The Fahrenheit number in Celsius is {temp_num}");
    }

}