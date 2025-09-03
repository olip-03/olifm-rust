use reqwest;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn pub_func() {
    println!("Inside public function");
}

fn pvt_func() {
    println!("Calling pvt function");
}

pub fn indirect_fn_access() {
    print!("Accessing indirect functions ");
    pvt_func();
}

pub async fn getJson(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await.unwrap().text().await;
    return result;
}
