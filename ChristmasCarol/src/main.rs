fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for (day_index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love gave to me:", day);

        for gift_index in (0..=day_index).rev() {
            if day_index > 0 && gift_index == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift_index]);
        }

        println!(); // 空行分隔每一节歌词
    }
}