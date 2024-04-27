const CRHISTMAS_ITEMS:[&str;12] = [
    "a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
fn main() {
    let mut song = String::new();
    for day in 0..CRHISTMAS_ITEMS.len(){
        let day_str = day_to_string(day);

        let mut lyrics = String::from(&format!("On the {} day of Christmas,\nmy true love gave to me",day_str));

        for previous_day in 0..day + 1{
            let previous_day = day - previous_day;
            let previous_christmas_item = CRHISTMAS_ITEMS[previous_day];

            if previous_day == 0 && day != 0 {
                lyrics.push_str(&format!("\nAnd {}",previous_christmas_item));
            }else if previous_day == 0 && day == 0 {
                lyrics.push_str(&format!("\n{}",previous_christmas_item)); 
            }else{
                lyrics.push_str(&format!("\n{},",previous_christmas_item));
            }
        }
        song.push_str(&format!("\n\n{}.",&lyrics));
    }
    println!("{}",song);
}

fn day_to_string(day:usize) -> String{
    let day = match day + 1 {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => return format!("{}th", day) 
    };
    day.to_string() 
}
