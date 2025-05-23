//#[allow(unused_variables)]
fn main() {
    //Class 69 Project
    let distance = 1_337;
    let miles = distance as i16;
    let height = 81.777;
    println!("{:.3}", height);

    let with_milk = true;
    let with_sugar = true;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptble = with_milk || with_sugar;

    let distances: [i8; 4] = [5, 15, 30, 60];
    println!("{:#?}", distances);

    let combo = (miles, height, is_my_type_of_coffee, distances);
   println!("{:#?}", combo);

    
}

/*
//Class 66
    let month_days = 1..31;
    println!("{:?}", month_days); // exibe o range inteiro

    let month_days = 1..=31; // o último número é incluído
    println!("{month_days:?}"); 

    for number in month_days {
        println!("day {}", number);
    }

    let letters = 'b'..'f';
    for letter in letters {
        println!("letter {}", letter);
    }

    let colors = ["red", "green", "yellow", ];
    for color in colors {
        println!("color {}", color);
    }

*/

    /*
     //Class 65
    let employee = ("Molly", 35, "Marketing");
    let name = employee.0;
    let age = employee.1;
    let department = employee.2;

    //   let (name, age, department) = employee; // em apenas uma linha
    
    println!("Name: {}, Age: {}, Department: {}", name, age, department);

 
    println!("{:?}", employee); // exibe a tupla inteira
    println!("{:#?}", employee); // exibe a tupla inteira com formatação debug

     */
    /*
    //Class 64
    let mut seasons = ["spring", "summer", "fall", "winter"];
    println!("{}", 5);
    println!("{}", true);
    println!("{}", 3.14);
  
    dbg!(seasons); // escrita simples para debugar
     */
/*
//Class 63
    let mut seasons = ["spring", "summer", "fall", "winter"];
    println!("{}", 5);
    println!("{}", true);
    println!("{}", 3.14);
    println!("{seasons:?}"); // exibe o array inteiro
    println!("{seasons:#?}");// exibe o array inteiro com formatação debug
*/
/*
//Class 61
let mut seasons = ["spring", "summer", "fall", "winter"];
let first = seasons[0];
let second = seasons[1];
println!("The first season is {} and the second is {}", first, second);

seasons[2] = "Autumn";
println!("The third season is {}", seasons[2]);
*/

/*
    //Class 60
    let number = [4, 8, 15, 16, 23, 42];
    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Lenght: {}", apples.len());// exibe o tamanho do array
*/
/*
//Class 59
    let first_initial = 'S';
    let emoji: char = '😅'; //Tecla windows + . (ponto) para abrir o teclado de emojis
    println!(
        "{} {}",
        first_initial.is_alphabetic(), // verifica se é alfabético
        emoji.is_alphabetic()
    );
    println!(
        "{} {}",
        first_initial.is_uppercase(), emoji.is_uppercase() // verifica se é maiusculo
    );
    println!(
        "{} {}",
        first_initial.is_lowercase(), emoji.is_lowercase()); // verifica se é minusculo

*/
/*
//Class 58
let user_has_payed_for_subscription = true;
let user_is_admin = true;
let user_can_see_premium_experience = user_has_payed_for_subscription || user_is_admin;
println!("Can this user see my site? {}",user_can_see_premium_experience);

*/
/*
    //Class 57
    let purchase_tickets = true;
    let plane_on_time = false;
    let making_event = purchase_tickets && plane_on_time;
    println!("It is {} tha I arraived as expected.", making_event);
*/
/*
//Class 55
   println!("{}", !true);
   println!("{}", !false);

    let age = 13;
    let can_se_rated_r_movie = age >= 17;
    let cannot_see_r_rated_movie = !can_se_rated_r_movie;
    println!("I am {} years old. Can I not see a rated R movie?{}", age, cannot_see_r_rated_movie);

*/
/*
 //Class 54
    let is_handsome = true;
    let is_sille = false;
    println!("Handsome: {}. Silly: {}.", is_handsome, is_sille);

    let age: i32 = 40;
    let is_young = age < 35;
    println!("{}", is_young);
    println!("{} {} ", age.is_positive(), age.is_negative()); //true or false

*/

/*
    //Class 53
    let mut year = 2025;
    year += 1;
    println!("The new year is {}.", year);

    year -= 5;
    println!("The new year is {}.", year);

    year *= 2;
    println!("The new year is {}.", year);

    year /= 4;
    println!("The new year is {}.", year);
*/

/*
    // cLASS 52
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multi = 3 * 4;
    println!("Addition: {}, Subtraction {}, multi {}.", addition, subtraction, multi);

    let floor_division = 5/3;
    println!("{}", floor_division);

    let decimal_division = 5.0 / 3.0;
    println!("{:.2}", decimal_division);

    let remainder = 9 % 2;
    println!("{}", remainder);
*/

/*
 //Class 51
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    let miles_away = 100.329132;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    println!("{}", miles_away_int);
*/
/*
// Class 50
let pi: f64 = 3.1415;
println!("The current value of pi is {:.4}", pi);
 */

/*
    //Class 49
    let pi: f64 = 3.14151265984625731;
    println!("The current value of pi is {pi}");

    //Arredondamento retirando casas decimais
    println!("{}", pi.floor());
    println!("{}", pi.ceil()); // Arredonda para cima
    println!("{}", pi.round()); // Arredonda para cima somente a partir de "0.5"
*/
/*
    //Class 48
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space = "        my content   ";
    println!("{}", empty_space.trim());

    println!("{}",value.pow(2));
    println!("{}",value.pow(3));
*/

/*
//Aula 44 Integers
    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64000;

    let thirty_two_bit_signed = -21474836;
    let thirty_two_bit_unsigned: u32 = 4294697295;
    let some_value = 20i8;
*/
