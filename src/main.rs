
const GET_ADDR: &str = "https://jsonplaceholder.typicode.com/posts/1";
const POST_ADDR: &str = "https://jsonplaceholder.typicode.com/posts";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(GET_ADDR)?;
    let header = resp.headers();
    println!("headers: {:?}", header);

    Ok(())
}
