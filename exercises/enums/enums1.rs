// enums1.rs
//
// No hints this time! :)


#[derive(Debug)]
enum Message {
    Quit,
    Echo=2,
    Move,
    ChangeColor,
    // TODO: define a few types of messages as used below
}

fn main() {
    let a:Message=Message::Quit;
    let b:Message=Message::Echo;
    let c=Message::Move;
    let d=Message::ChangeColor;
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
