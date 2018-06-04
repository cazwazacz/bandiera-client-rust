extern crate bandiera_client;

fn main() {
    let base_url = String::from("http://localhost:5000");

    let mut client = bandiera_client::create_client(base_url);

    let all = client.get_all();

    println!("{}", all);

    let features_for_group = client.get_features_for_group("group-one");

    println!("{}", features_for_group);

    if client.get_feature("group-one", "another-feature") {
        println!("hello");
    } else {
        println!("bye bye");
    }
}