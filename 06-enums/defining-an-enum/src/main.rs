enum IpAddrKind {
    V4,
    V6
}

/*
    This enum has four variants with different types:
    
    `Quit` has no data associated with it at all.
    `Move` has named fields, like a struct does.
    `Write` includes a single `String`.
    `ChangeColor` includes three `i32` values.
*/
enum Message {
    Quit,
    Mov { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("from inside Message");
    }
}

fn main() {
    /*
        Enum Values

        use a double colon to seperate the two
    */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let m = Message::Write(String::from("Hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}