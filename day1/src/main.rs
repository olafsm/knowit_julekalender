/*
* Knowit Julekalender 2021
* Day 1
* @olafsm  
*/
fn main() {
    let mut input = String::from(include_str!("../input.txt"));
    eprintln!("input.chars().len() = {:?}", input.chars().count());
    let mut total = 0;

    // replace ambiguous first
    input = input.replace("femti","50");
    input = input.replace("tretten","13");
    input = input.replace("tretti","30");
    input = input.replace("tolv","12");
    input = input.replace("nitten","19");

    
    let mut s = String::from("");
    for c in input.chars() {
        s.push(c);
        let val = get_value_from_string(&s);
        if val != 0 {
            total += val; 
            s = String::from("");
        }
    }
    eprintln!("total = {:?}", total);
}

fn get_value_from_string(s:&str) -> i32 {
    match s {
        "en" => 1,
        "to" => 2,
        "tre" => 3,
        "fire" => 4 ,
        "fem" => 5,
        "seks" => 6,
        "sju" => 7,
        "åtte" => 8,
        "ni" => 9,
        "ti" => 10,
        "elleve" => 11,
        "fjor" => 4,
        "syt" => 7,
        "at" => 8,
        "tjue" => 20,
        "førti" => 40,
        "ten" => 10,
        "12" => 12,
        "50" => 50,
        "13" => 13,
        "30" => 30,
        "19" => 19,
        _ => 0,
    }
}
