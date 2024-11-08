fn print_array(arr: &[u8]) {
    for num in arr {
        print!("{} ", num);
    }
    print!("\n");
}

fn print_2d_array(arr_2d:&[[u8; 3]; 3]) {
    if arr_2d.len() > 0 {
        for i in 0..arr_2d.len() {
            for j in 0..arr_2d[0].len() {
                print!("{} ", arr_2d[i][j]);
            }
        }
        print!("\n");
    }
}

fn print_learning_section_header(message: &str) {
    println!("\n<><><><><> {} <><><><><>", message.to_uppercase());
}

fn square(number : i32) -> i32 {
    println!("Squaring : {number}");
    number * number // Don't put a ; if the return value is at the end of a function
}

fn square_return(number : i32) -> i32 {
    println!("Squaring : {number}");
    let ret = number * number;
    println!("Square Result: {ret}");
    return ret;
}

// return multiple values with a tuple
fn square_return_tuple(number : i32) -> (i32, i32) {
    println!("Squaring : {number}");
    let ret = (number, number * number);
    println!("Squaring: {}, Result: {}", ret.0, ret.1);
    return ret;
}

// <><><><><><><><> MAIN FUNCTIONS <><><><><><><><><>
fn string_manip() {
    print_learning_section_header("SIMPLE STRING DISPLAY AND MANIPULATION");
    let mut message = String::from("HELLO WORLD MY FRIENDS");
    message.push('.');
    message.push_str("\nI Appended this Message To the End!");
    println!("This is a message from a string var: {}", message);
}

fn playing_with_numbers_and_variables(){
    print_learning_section_header("PLAYING WITH NUMBER VARIABLES");
    let dec_number: u16 =  6900;
    println!("Decimal Number: {}", dec_number);

    let mut pi: f64 = 3.141592653589793;
    println!("The Floating Point Number PI: {}", pi);
    // Added formater for 3 decimals of percision
    println!("Add 1 to PI: {:.3}", pi + 1.0);

    pi += 7.0;
    println!("Mut PI by 7.0: {}", pi);

    // This is a couple of ways to cast a dec to a float in Rust
    let _dec_number_as_float = f64::from(dec_number);
    let adding_flaot_and_dec: f64 = pi + dec_number as f64;
    println!("Mut PI by dec number: {}", adding_flaot_and_dec);
}

fn bitwise_operation_examples() {
    print_learning_section_header("BITWISE OPPERATIONS");
    let binary_number = 0b1100_0101u8;
    println!("Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number);
    println!("NOT OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", !binary_number);
    println!("AND OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number & 0b1100_0000);
    println!("OR OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number | 0b0000_1010);
    println!("XOR OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number ^ 0b1111_1111);
    println!("AND OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number & 0b1100_0000);
    println!("RIGHT SHIFT >> OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number >> 4);
    println!("LEFT SHIFT << OPERATOR Dec: {0}, Hex: 0x{0:02X}, Binary: 0b{0:08b}", binary_number << 2);
}

fn char_example() {
    print_learning_section_header("CHAR's NOTE: THEY ARE 4 BYTES FOR UNICODE");
    let letter = 'T';
    let finger = '\u{261D}';
    println!("Letter: {}, Finger: {}", letter, finger);
}

fn avg_example() {
    print_learning_section_header("CALC AVG OF THREE NUMBERS");
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let avg = (a as f64 + b + c as f64)/3.0;
    assert_eq!(avg, 45.1);
    println!("a: {}, b: {}, c: {}, AVG: {}", a, b, c, avg);
}

fn array_examples() {
    print_learning_section_header("ARRAYS");
    let mut arr_numbers :[u8; 6]= [1,2,3,4,5,6];
    arr_numbers[2] = 69;
    print_array(&arr_numbers);
    arr_numbers = [0,0,0,0,0,0];
    print_array(&arr_numbers);
    arr_numbers = [69;6]; // This will set 6 values of the array to 69
    print_array(&arr_numbers);

    let array_2d:[[u8; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];
    print_2d_array( &array_2d);
}

fn tuple_examples() {
    print_learning_section_header("TUPLES");
    let mut tuple_var: (u8, f32, char, f64) = (15,3.14,'a',10.695847);
    tuple_var.0 += 15;
    tuple_var.1 += 1.0;

    let (ta,tb,tc, td) = tuple_var;
    println!("TUPLE: {ta}, {tb}, {tc}, {td}");
}

fn functions_examples(){
    print_learning_section_header("FUNCTIONS");
    let square_result = square(13);
    println!("Result of Square: {square_result}");
    square_return(69);
    square_return_tuple(13);
}

fn loop_examples() {
    print_learning_section_header("LOOP EXAMPLES");
    // loop just goes for infinity
    let mut counter = 0;
    let result = loop { // you can also turn the loop into a expression and return a value
        counter += 1;
        if counter >= 100 {
            break counter;
        }
    };
    println!("Counter Loop Value: {result}");

    counter = 0;
    while counter < 10 {
        counter += 1;
    }
    println!("Counter While Value: {counter}");

    println!("Regualr Iterator for loop");
    let word = ['h', 'e', 'l','l','o'];
    for c in word {
        print!("{c}");
    }
    println!();

    println!("for loop with index and value");
    for (index ,c) in word.iter().enumerate() {
        println!("Index: {index}, Char: {c}");
    }

    println!("for loop using ranges 0-4");
    for n in 0.. 4 {
        print!("{}", word[n]);
    }
    println!();

    println!("Print a 2D Matrix");
    let mut matrix = [[1,2,3],[4,5,6],[7,8,9]];
    for row in matrix.iter_mut() { // you have to use iter_mut to change the values
        for item in row.iter_mut() {
            *item += 10;
            print!("{}\t", item);
        }
        println!();
    }

}

fn get_max_min_mean() {
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max: i32 = i32::min_value();
    let mut min: i32 = i32::max_value();
    let mut total: i32 = 0;
    let mut mean: f64 = 0.0;

    for n in numbers {
        total += n;

        if max < n {
            max = n;
        }

        if min > n {
            min = n;
        }
    }

    mean = total as f64 / numbers.len() as f64;

    println!("MAX: {max}, MIN: {min}, MEAN: {mean}");
}

fn main() {
    string_manip();
    playing_with_numbers_and_variables();
    bitwise_operation_examples();
    char_example();
    avg_example();
    array_examples();
    tuple_examples();
    functions_examples();    
    loop_examples();   
    get_max_min_mean();
}
