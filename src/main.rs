extern crate bandiera_client;

fn main() {
    let base_url = String::from("http://localhost:5000");

    let mut client = bandiera_client::create_client(base_url);

    let response = client.get_all();

    println!("{}", response);

    if client.get_feature("group-one", "another-feature") {
        println!("hello");
    } else {
        println!("bye bye");
    }
}