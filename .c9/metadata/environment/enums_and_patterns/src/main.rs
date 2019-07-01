{"changed":true,"filter":false,"title":"main.rs","tooltip":"/enums_and_patterns/src/main.rs","value":"enum IpAddr {\n    V4(u8, u8, u8, u8),\n    V6(String),\n}\n\n#[derive(Debug)]\nenum Message {\n    Quit,\n    Move { x: i32, y: i32 },\n    Write(String),\n    ChangeColor(i32, i32, i32),\n}\n\nimpl Message {\n    fn call(&self) {\n        println!(\"{:#?}\", &self);\n    }\n}\n\nfn main() {\n    let home = IpAddr::V4(127, 0, 0, 1);\n    let loopback = IpAddr::V6(String::from(\"::1\"));\n    \n    let m = Message::Write(String::from(\"hello\"));\n    m.call();\n\n}\n\n// The Option type\nenum Option<T> {\n    Some(T),\n    None,\n}\n\nfn option_examples() {\n    let some_number = Some(5);\n    let some_string = Some(\"a string\");\n\n    // If we use None rather than Some, we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type\n    let absent_number: Option<i32> = None;\n}","undoManager":{"mark":83,"position":88,"stack":[[{"start":{"row":0,"column":0},"end":{"row":1,"column":0},"action":"insert","lines":["",""],"id":2},{"start":{"row":1,"column":0},"end":{"row":2,"column":0},"action":"insert","lines":["",""]}],[{"start":{"row":0,"column":0},"end":{"row":0,"column":1},"action":"insert","lines":["e"],"id":3},{"start":{"row":0,"column":1},"end":{"row":0,"column":2},"action":"insert","lines":["n"]},{"start":{"row":0,"column":2},"end":{"row":0,"column":3},"action":"insert","lines":["u"]},{"start":{"row":0,"column":3},"end":{"row":0,"column":4},"action":"insert","lines":["m"]}],[{"start":{"row":0,"column":4},"end":{"row":0,"column":5},"action":"insert","lines":[" "],"id":4},{"start":{"row":0,"column":5},"end":{"row":0,"column":6},"action":"insert","lines":["I"]},{"start":{"row":0,"column":6},"end":{"row":0,"column":7},"action":"insert","lines":["p"]}],[{"start":{"row":0,"column":7},"end":{"row":0,"column":9},"action":"insert","lines":["[]"],"id":5}],[{"start":{"row":0,"column":7},"end":{"row":0,"column":9},"action":"remove","lines":["[]"],"id":6}],[{"start":{"row":0,"column":7},"end":{"row":0,"column":8},"action":"insert","lines":["A"],"id":7},{"start":{"row":0,"column":8},"end":{"row":0,"column":9},"action":"insert","lines":["d"]},{"start":{"row":0,"column":9},"end":{"row":0,"column":10},"action":"insert","lines":["d"]},{"start":{"row":0,"column":10},"end":{"row":0,"column":11},"action":"insert","lines":["r"]}],[{"start":{"row":0,"column":11},"end":{"row":0,"column":12},"action":"insert","lines":["K"],"id":8},{"start":{"row":0,"column":12},"end":{"row":0,"column":13},"action":"insert","lines":["i"]},{"start":{"row":0,"column":13},"end":{"row":0,"column":14},"action":"insert","lines":["n"]},{"start":{"row":0,"column":14},"end":{"row":0,"column":15},"action":"insert","lines":["d"]}],[{"start":{"row":0,"column":15},"end":{"row":0,"column":16},"action":"insert","lines":[" "],"id":9},{"start":{"row":0,"column":16},"end":{"row":0,"column":17},"action":"insert","lines":["{"]}],[{"start":{"row":0,"column":17},"end":{"row":2,"column":1},"action":"insert","lines":["","    ","}"],"id":10}],[{"start":{"row":1,"column":4},"end":{"row":1,"column":5},"action":"insert","lines":["V"],"id":11},{"start":{"row":1,"column":5},"end":{"row":1,"column":6},"action":"insert","lines":["4"]},{"start":{"row":1,"column":6},"end":{"row":1,"column":7},"action":"insert","lines":[","]}],[{"start":{"row":1,"column":7},"end":{"row":2,"column":0},"action":"insert","lines":["",""],"id":12},{"start":{"row":2,"column":0},"end":{"row":2,"column":4},"action":"insert","lines":["    "]},{"start":{"row":2,"column":4},"end":{"row":2,"column":5},"action":"insert","lines":["V"]},{"start":{"row":2,"column":5},"end":{"row":2,"column":6},"action":"insert","lines":["6"]}],[{"start":{"row":2,"column":6},"end":{"row":2,"column":7},"action":"insert","lines":[","],"id":13}],[{"start":{"row":6,"column":4},"end":{"row":6,"column":30},"action":"remove","lines":["println!(\"Hello, world!\");"],"id":14},{"start":{"row":6,"column":4},"end":{"row":7,"column":25},"action":"insert","lines":["let four = IpAddrKind::V4;","let six = IpAddrKind::V6;"]}],[{"start":{"row":7,"column":0},"end":{"row":7,"column":4},"action":"insert","lines":["    "],"id":15}],[{"start":{"row":9,"column":0},"end":{"row":9,"column":33},"action":"insert","lines":["fn route(ip_kind: IpAddrKind) { }"],"id":16}],[{"start":{"row":8,"column":1},"end":{"row":9,"column":0},"action":"insert","lines":["",""],"id":17}],[{"start":{"row":7,"column":29},"end":{"row":8,"column":0},"action":"insert","lines":["",""],"id":18},{"start":{"row":8,"column":0},"end":{"row":8,"column":4},"action":"insert","lines":["    "]}],[{"start":{"row":8,"column":4},"end":{"row":9,"column":22},"action":"insert","lines":["route(IpAddrKind::V4);","route(IpAddrKind::V6);"],"id":19}],[{"start":{"row":8,"column":4},"end":{"row":9,"column":0},"action":"insert","lines":["",""],"id":20},{"start":{"row":9,"column":0},"end":{"row":9,"column":4},"action":"insert","lines":["    "]}],[{"start":{"row":10,"column":0},"end":{"row":10,"column":4},"action":"insert","lines":["    "],"id":21}],[{"start":{"row":0,"column":0},"end":{"row":3,"column":1},"action":"remove","lines":["enum IpAddrKind {","    V4,","    V6,","}"],"id":22},{"start":{"row":0,"column":0},"end":{"row":7,"column":47},"action":"insert","lines":["enum IpAddr {","    V4(u8, u8, u8, u8),","    V6(String),","}","","let home = IpAddr::V4(127, 0, 0, 1);","","let loopback = IpAddr::V6(String::from(\"::1\"));"]}],[{"start":{"row":5,"column":36},"end":{"row":6,"column":0},"action":"remove","lines":["",""],"id":23}],[{"start":{"row":16,"column":33},"end":{"row":17,"column":0},"action":"insert","lines":["",""],"id":24},{"start":{"row":17,"column":0},"end":{"row":18,"column":0},"action":"insert","lines":["",""]}],[{"start":{"row":18,"column":0},"end":{"row":24,"column":0},"action":"insert","lines":["enum Message {","    Quit,","    Move { x: i32, y: i32 },","    Write(String),","    ChangeColor(i32, i32, i32),","}",""],"id":25}],[{"start":{"row":24,"column":0},"end":{"row":25,"column":0},"action":"insert","lines":["",""],"id":26}],[{"start":{"row":25,"column":0},"end":{"row":31,"column":57},"action":"insert","lines":["struct QuitMessage; // unit struct","struct MoveMessage {","    x: i32,","    y: i32,","}","struct WriteMessage(String); // tuple struct","struct ChangeColorMessage(i32, i32, i32); // tuple struct"],"id":27}],[{"start":{"row":5,"column":0},"end":{"row":6,"column":47},"action":"remove","lines":["let home = IpAddr::V4(127, 0, 0, 1);","let loopback = IpAddr::V6(String::from(\"::1\"));"],"id":28},{"start":{"row":4,"column":0},"end":{"row":5,"column":0},"action":"remove","lines":["",""]},{"start":{"row":3,"column":1},"end":{"row":4,"column":0},"action":"remove","lines":["",""]}],[{"start":{"row":7,"column":29},"end":{"row":8,"column":0},"action":"insert","lines":["",""],"id":29},{"start":{"row":8,"column":0},"end":{"row":8,"column":4},"action":"insert","lines":["    "]},{"start":{"row":8,"column":4},"end":{"row":9,"column":0},"action":"insert","lines":["",""]},{"start":{"row":9,"column":0},"end":{"row":9,"column":4},"action":"insert","lines":["    "]}],[{"start":{"row":9,"column":4},"end":{"row":10,"column":47},"action":"insert","lines":["let home = IpAddr::V4(127, 0, 0, 1);","let loopback = IpAddr::V6(String::from(\"::1\"));"],"id":30}],[{"start":{"row":10,"column":0},"end":{"row":10,"column":4},"action":"insert","lines":["    "],"id":31}],[{"start":{"row":10,"column":51},"end":{"row":13,"column":26},"action":"remove","lines":["","    ","    route(IpAddrKind::V4);","    route(IpAddrKind::V6);"],"id":32}],[{"start":{"row":12,"column":0},"end":{"row":13,"column":33},"action":"remove","lines":["","fn route(ip_kind: IpAddrKind) { }"],"id":33},{"start":{"row":11,"column":1},"end":{"row":12,"column":0},"action":"remove","lines":["",""]}],[{"start":{"row":5,"column":11},"end":{"row":7,"column":29},"action":"remove","lines":["","    let four = IpAddrKind::V4;","    let six = IpAddrKind::V6;"],"id":34}],[{"start":{"row":6,"column":0},"end":{"row":6,"column":4},"action":"remove","lines":["    "],"id":35},{"start":{"row":5,"column":11},"end":{"row":6,"column":0},"action":"remove","lines":["",""]}],[{"start":{"row":17,"column":0},"end":{"row":23,"column":57},"action":"remove","lines":["struct QuitMessage; // unit struct","struct MoveMessage {","    x: i32,","    y: i32,","}","struct WriteMessage(String); // tuple struct","struct ChangeColorMessage(i32, i32, i32); // tuple struct"],"id":36},{"start":{"row":16,"column":0},"end":{"row":17,"column":0},"action":"remove","lines":["",""]},{"start":{"row":15,"column":1},"end":{"row":16,"column":0},"action":"remove","lines":["",""]}],[{"start":{"row":7,"column":51},"end":{"row":8,"column":0},"action":"insert","lines":["",""],"id":37},{"start":{"row":8,"column":0},"end":{"row":8,"column":4},"action":"insert","lines":["    "]}],[{"start":{"row":8,"column":4},"end":{"row":10,"column":0},"action":"insert","lines":["let m = Message::Write(String::from(\"hello\"));","m.call();",""],"id":38}],[{"start":{"row":9,"column":0},"end":{"row":9,"column":4},"action":"insert","lines":["    "],"id":39}],[{"start":{"row":13,"column":0},"end":{"row":18,"column":1},"action":"remove","lines":["enum Message {","    Quit,","    Move { x: i32, y: i32 },","    Write(String),","    ChangeColor(i32, i32, i32),","}"],"id":40},{"start":{"row":12,"column":0},"end":{"row":13,"column":0},"action":"remove","lines":["",""]},{"start":{"row":11,"column":1},"end":{"row":12,"column":0},"action":"remove","lines":["",""]}],[{"start":{"row":4,"column":0},"end":{"row":5,"column":0},"action":"insert","lines":["",""],"id":41}],[{"start":{"row":5,"column":0},"end":{"row":10,"column":1},"action":"insert","lines":["enum Message {","    Quit,","    Move { x: i32, y: i32 },","    Write(String),","    ChangeColor(i32, i32, i32),","}"],"id":42}],[{"start":{"row":10,"column":1},"end":{"row":11,"column":0},"action":"insert","lines":["",""],"id":43}],[{"start":{"row":14,"column":51},"end":{"row":15,"column":0},"action":"insert","lines":["",""],"id":44},{"start":{"row":15,"column":0},"end":{"row":15,"column":4},"action":"insert","lines":["    "]}],[{"start":{"row":10,"column":1},"end":{"row":11,"column":0},"action":"insert","lines":["",""],"id":45},{"start":{"row":11,"column":0},"end":{"row":12,"column":0},"action":"insert","lines":["",""]}],[{"start":{"row":12,"column":0},"end":{"row":16,"column":1},"action":"insert","lines":["impl Message {","    fn call(&self) {","        // method body would be defined here","    }","}"],"id":46}],[{"start":{"row":14,"column":8},"end":{"row":14,"column":44},"action":"remove","lines":["// method body would be defined here"],"id":47},{"start":{"row":14,"column":8},"end":{"row":14,"column":9},"action":"insert","lines":["p"]},{"start":{"row":14,"column":9},"end":{"row":14,"column":10},"action":"insert","lines":["r"]},{"start":{"row":14,"column":10},"end":{"row":14,"column":11},"action":"insert","lines":["i"]},{"start":{"row":14,"column":11},"end":{"row":14,"column":12},"action":"insert","lines":["n"]},{"start":{"row":14,"column":12},"end":{"row":14,"column":13},"action":"insert","lines":["t"]},{"start":{"row":14,"column":13},"end":{"row":14,"column":14},"action":"insert","lines":["l"]},{"start":{"row":14,"column":14},"end":{"row":14,"column":15},"action":"insert","lines":["n"]},{"start":{"row":14,"column":15},"end":{"row":14,"column":16},"action":"insert","lines":["!"]}],[{"start":{"row":14,"column":16},"end":{"row":14,"column":18},"action":"insert","lines":["()"],"id":48}],[{"start":{"row":14,"column":17},"end":{"row":14,"column":19},"action":"insert","lines":["{}"],"id":49}],[{"start":{"row":14,"column":19},"end":{"row":14,"column":20},"action":"insert","lines":[","],"id":50}],[{"start":{"row":14,"column":20},"end":{"row":14,"column":21},"action":"insert","lines":[" "],"id":51}],[{"start":{"row":14,"column":21},"end":{"row":14,"column":22},"action":"insert","lines":["&"],"id":52},{"start":{"row":14,"column":22},"end":{"row":14,"column":23},"action":"insert","lines":["s"]},{"start":{"row":14,"column":23},"end":{"row":14,"column":24},"action":"insert","lines":["e"]},{"start":{"row":14,"column":24},"end":{"row":14,"column":25},"action":"insert","lines":["l"]},{"start":{"row":14,"column":25},"end":{"row":14,"column":26},"action":"insert","lines":["f"]},{"start":{"row":14,"column":26},"end":{"row":14,"column":27},"action":"insert","lines":["."]},{"start":{"row":14,"column":27},"end":{"row":14,"column":28},"action":"insert","lines":["W"]},{"start":{"row":14,"column":28},"end":{"row":14,"column":29},"action":"insert","lines":["r"]},{"start":{"row":14,"column":29},"end":{"row":14,"column":30},"action":"insert","lines":["i"]},{"start":{"row":14,"column":30},"end":{"row":14,"column":31},"action":"insert","lines":["t"]},{"start":{"row":14,"column":31},"end":{"row":14,"column":32},"action":"insert","lines":["e"]}],[{"start":{"row":14,"column":33},"end":{"row":14,"column":34},"action":"insert","lines":[";"],"id":53}],[{"start":{"row":14,"column":17},"end":{"row":14,"column":18},"action":"insert","lines":["\""],"id":54}],[{"start":{"row":14,"column":20},"end":{"row":14,"column":21},"action":"insert","lines":["\""],"id":55}],[{"start":{"row":14,"column":23},"end":{"row":14,"column":24},"action":"remove","lines":["&"],"id":56}],[{"start":{"row":14,"column":23},"end":{"row":14,"column":24},"action":"insert","lines":["&"],"id":57}],[{"start":{"row":14,"column":28},"end":{"row":14,"column":34},"action":"remove","lines":[".Write"],"id":58}],[{"start":{"row":12,"column":14},"end":{"row":13,"column":0},"action":"insert","lines":["",""],"id":59}],[{"start":{"row":13,"column":0},"end":{"row":13,"column":16},"action":"insert","lines":["#[derive(Debug)]"],"id":60}],[{"start":{"row":13,"column":0},"end":{"row":13,"column":4},"action":"insert","lines":["    "],"id":61}],[{"start":{"row":13,"column":4},"end":{"row":13,"column":20},"action":"remove","lines":["#[derive(Debug)]"],"id":62},{"start":{"row":13,"column":0},"end":{"row":13,"column":4},"action":"remove","lines":["    "]},{"start":{"row":12,"column":14},"end":{"row":13,"column":0},"action":"remove","lines":["",""]}],[{"start":{"row":4,"column":0},"end":{"row":5,"column":0},"action":"insert","lines":["",""],"id":63}],[{"start":{"row":5,"column":0},"end":{"row":5,"column":16},"action":"insert","lines":["#[derive(Debug)]"],"id":64}],[{"start":{"row":15,"column":19},"end":{"row":15,"column":20},"action":"insert","lines":[":"],"id":65},{"start":{"row":15,"column":20},"end":{"row":15,"column":21},"action":"insert","lines":["#"]},{"start":{"row":15,"column":21},"end":{"row":15,"column":22},"action":"insert","lines":["?"]}],[{"start":{"row":26,"column":1},"end":{"row":27,"column":0},"action":"insert","lines":["",""],"id":66},{"start":{"row":27,"column":0},"end":{"row":28,"column":0},"action":"insert","lines":["",""]},{"start":{"row":28,"column":0},"end":{"row":28,"column":1},"action":"insert","lines":["/"]},{"start":{"row":28,"column":1},"end":{"row":28,"column":2},"action":"insert","lines":["/"]}],[{"start":{"row":28,"column":2},"end":{"row":28,"column":3},"action":"insert","lines":[" "],"id":67},{"start":{"row":28,"column":3},"end":{"row":28,"column":4},"action":"insert","lines":["T"]},{"start":{"row":28,"column":4},"end":{"row":28,"column":5},"action":"insert","lines":["h"]},{"start":{"row":28,"column":5},"end":{"row":28,"column":6},"action":"insert","lines":["e"]}],[{"start":{"row":28,"column":6},"end":{"row":28,"column":7},"action":"insert","lines":[" "],"id":68},{"start":{"row":28,"column":7},"end":{"row":28,"column":8},"action":"insert","lines":["O"]},{"start":{"row":28,"column":8},"end":{"row":28,"column":9},"action":"insert","lines":["p"]},{"start":{"row":28,"column":9},"end":{"row":28,"column":10},"action":"insert","lines":["t"]},{"start":{"row":28,"column":10},"end":{"row":28,"column":11},"action":"insert","lines":["i"]},{"start":{"row":28,"column":11},"end":{"row":28,"column":12},"action":"insert","lines":["o"]},{"start":{"row":28,"column":12},"end":{"row":28,"column":13},"action":"insert","lines":["n"]}],[{"start":{"row":28,"column":13},"end":{"row":28,"column":14},"action":"insert","lines":[" "],"id":69},{"start":{"row":28,"column":14},"end":{"row":28,"column":15},"action":"insert","lines":["t"]},{"start":{"row":28,"column":15},"end":{"row":28,"column":16},"action":"insert","lines":["y"]},{"start":{"row":28,"column":16},"end":{"row":28,"column":17},"action":"insert","lines":["p"]},{"start":{"row":28,"column":17},"end":{"row":28,"column":18},"action":"insert","lines":["e"]}],[{"start":{"row":28,"column":18},"end":{"row":29,"column":0},"action":"insert","lines":["",""],"id":70}],[{"start":{"row":29,"column":0},"end":{"row":32,"column":1},"action":"insert","lines":["enum Option<T> {","    Some(T),","    None,","}"],"id":71}],[{"start":{"row":32,"column":1},"end":{"row":33,"column":0},"action":"insert","lines":["",""],"id":72},{"start":{"row":33,"column":0},"end":{"row":34,"column":0},"action":"insert","lines":["",""]}],[{"start":{"row":34,"column":0},"end":{"row":34,"column":1},"action":"insert","lines":["f"],"id":73},{"start":{"row":34,"column":1},"end":{"row":34,"column":2},"action":"insert","lines":["n"]}],[{"start":{"row":34,"column":2},"end":{"row":34,"column":3},"action":"insert","lines":[" "],"id":74},{"start":{"row":34,"column":3},"end":{"row":34,"column":4},"action":"insert","lines":["o"]},{"start":{"row":34,"column":4},"end":{"row":34,"column":5},"action":"insert","lines":["p"]},{"start":{"row":34,"column":5},"end":{"row":34,"column":6},"action":"insert","lines":["t"]},{"start":{"row":34,"column":6},"end":{"row":34,"column":7},"action":"insert","lines":["i"]},{"start":{"row":34,"column":7},"end":{"row":34,"column":8},"action":"insert","lines":["o"]},{"start":{"row":34,"column":8},"end":{"row":34,"column":9},"action":"insert","lines":["n"]}],[{"start":{"row":34,"column":9},"end":{"row":34,"column":10},"action":"insert","lines":["_"],"id":75},{"start":{"row":34,"column":10},"end":{"row":34,"column":11},"action":"insert","lines":["e"]},{"start":{"row":34,"column":11},"end":{"row":34,"column":12},"action":"insert","lines":["x"]},{"start":{"row":34,"column":12},"end":{"row":34,"column":13},"action":"insert","lines":["m"]},{"start":{"row":34,"column":13},"end":{"row":34,"column":14},"action":"insert","lines":["p"]},{"start":{"row":34,"column":14},"end":{"row":34,"column":15},"action":"insert","lines":["l"]},{"start":{"row":34,"column":15},"end":{"row":34,"column":16},"action":"insert","lines":["a"]},{"start":{"row":34,"column":16},"end":{"row":34,"column":17},"action":"insert","lines":["e"]},{"start":{"row":34,"column":17},"end":{"row":34,"column":18},"action":"insert","lines":["s"]}],[{"start":{"row":34,"column":17},"end":{"row":34,"column":18},"action":"remove","lines":["s"],"id":76},{"start":{"row":34,"column":16},"end":{"row":34,"column":17},"action":"remove","lines":["e"]},{"start":{"row":34,"column":15},"end":{"row":34,"column":16},"action":"remove","lines":["a"]}],[{"start":{"row":34,"column":15},"end":{"row":34,"column":16},"action":"insert","lines":["e"],"id":77},{"start":{"row":34,"column":16},"end":{"row":34,"column":17},"action":"insert","lines":["s"]}],[{"start":{"row":34,"column":16},"end":{"row":34,"column":17},"action":"remove","lines":["s"],"id":78},{"start":{"row":34,"column":15},"end":{"row":34,"column":16},"action":"remove","lines":["e"]},{"start":{"row":34,"column":14},"end":{"row":34,"column":15},"action":"remove","lines":["l"]},{"start":{"row":34,"column":13},"end":{"row":34,"column":14},"action":"remove","lines":["p"]},{"start":{"row":34,"column":12},"end":{"row":34,"column":13},"action":"remove","lines":["m"]}],[{"start":{"row":34,"column":12},"end":{"row":34,"column":13},"action":"insert","lines":["a"],"id":79},{"start":{"row":34,"column":13},"end":{"row":34,"column":14},"action":"insert","lines":["m"]},{"start":{"row":34,"column":14},"end":{"row":34,"column":15},"action":"insert","lines":["p"]},{"start":{"row":34,"column":15},"end":{"row":34,"column":16},"action":"insert","lines":["l"]},{"start":{"row":34,"column":16},"end":{"row":34,"column":17},"action":"insert","lines":["e"]},{"start":{"row":34,"column":17},"end":{"row":34,"column":18},"action":"insert","lines":["s"]}],[{"start":{"row":34,"column":18},"end":{"row":34,"column":20},"action":"insert","lines":["()"],"id":80}],[{"start":{"row":34,"column":20},"end":{"row":34,"column":21},"action":"insert","lines":[" "],"id":81},{"start":{"row":34,"column":21},"end":{"row":34,"column":22},"action":"insert","lines":["{"]}],[{"start":{"row":34,"column":22},"end":{"row":36,"column":1},"action":"insert","lines":["","    ","}"],"id":82}],[{"start":{"row":35,"column":4},"end":{"row":38,"column":38},"action":"insert","lines":["let some_number = Some(5);","let some_string = Some(\"a string\");","","let absent_number: Option<i32> = None;"],"id":83}],[{"start":{"row":36,"column":0},"end":{"row":36,"column":4},"action":"insert","lines":["    "],"id":84}],[{"start":{"row":38,"column":0},"end":{"row":38,"column":4},"action":"insert","lines":["    "],"id":85}],[{"start":{"row":37,"column":0},"end":{"row":38,"column":0},"action":"insert","lines":["",""],"id":86}],[{"start":{"row":38,"column":0},"end":{"row":38,"column":4},"action":"insert","lines":["    "],"id":87}],[{"start":{"row":38,"column":4},"end":{"row":38,"column":5},"action":"insert","lines":["/"],"id":88},{"start":{"row":38,"column":5},"end":{"row":38,"column":6},"action":"insert","lines":["/"]}],[{"start":{"row":38,"column":6},"end":{"row":38,"column":7},"action":"insert","lines":[" "],"id":89}],[{"start":{"row":38,"column":7},"end":{"row":38,"column":134},"action":"insert","lines":["If we use None rather than Some, we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type"],"id":90}]]},"ace":{"folds":[],"scrolltop":314,"scrollleft":0,"selection":{"start":{"row":38,"column":134},"end":{"row":38,"column":134},"isBackwards":false},"options":{"guessTabSize":true,"useWrapMode":false,"wrapToView":true},"firstLineState":{"row":17,"state":"start","mode":"ace/mode/rust"}},"timestamp":1562007716997}