fn main() {
    let string = String::from("фывфывфывasdfasdf\r\n");
    let len = string.as_str().len();
    let str : String = string.chars().rev().take(4).collect();
    println!("Length {}", len);
    println!("{:?}", str);
}
