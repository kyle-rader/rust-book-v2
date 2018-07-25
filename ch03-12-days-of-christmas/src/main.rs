use std::iter::Iterator;

fn main() {

    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Waiting",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for day in 0..gifts.len() {
        let human_day = day + 1;
        println!("\nOn the {}{} day of Christmas my true love gave to me", human_day, num_suffix(human_day as u32));
        for gift_index in (0..human_day).rev() {
            let prefix = if gift_index == 0 && human_day > 1 {
                "and "
            } else {
                ""
            };

            let suffix = if gift_index == 0 { "." } else { ", " };

            println!("{}{}{}", prefix, gifts[gift_index], suffix);
        }
    }
}

fn num_suffix(x: u32) -> &'static str {
    let i = x % 10;
    let j = x % 100;

    if i == 1 && j != 11 {
        "st"
    } else if i == 2 && j != 12 {
        "nd"
    } else if i == 3 && j != 13 {
        "rd"
    } else {
        "th"
    }
}