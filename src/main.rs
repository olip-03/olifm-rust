use shared::getJson;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let result = getJson("https://jsonplaceholder.typicode.com/posts/1")
            .await
            .unwrap();
        println!("10 + 20 = {}", result);
    })
}
