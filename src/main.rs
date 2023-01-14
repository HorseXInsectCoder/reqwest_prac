use std::collections::HashMap;
use serde::{Serialize, Deserialize};

const GET_ADDR: &str = "https://jsonplaceholder.typicode.com/posts/1";
const GET_MULTI_POSTS_ADDR: &str = "https://jsonplaceholder.typicode.com/posts";

#[derive(Serialize, Deserialize, Debug)]
struct Posts {                      // 结构体的字段要与返回的字段对应
    #[serde(rename = "userId")]
    user_id: Option<usize>,

    id: Option<usize>,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // GET方法
    // 把返回的json数据反序列化成结构体
    // let posts_get_one: Posts = client
    //     .get(GET_ADDR)
    //     .send()
    //     .await?
    //     .json()         // 用text的话返回String
    //     .await?;
    // println!("{:?}", posts_get_one);

    // 返回多个post数据
    // let posts_get_array: Vec<Posts> = client
    //     .get(GET_MULTI_POSTS_ADDR)
    //     .send()
    //     .await?
    //     .json()
    //     .await?;
    // println!("{:#?}", posts_get_array);
    // println!("{:?}", posts_get_array.len());
    //
    // // POST的三种请求体
    // 1. 结构体方式
    let create_posts = Posts {
        user_id: Some(10),
        id: None,
        title: "some title".to_string(),
        body: "body".to_string()
    };

    // 2. map方式
    let mut posts_map: HashMap<&str, &str> = HashMap::new();
    // 这里服务器返回一个id为101，所以id不用传
    posts_map.insert("userId", "1");
    posts_map.insert("title", "title map");
    posts_map.insert("body", "body map");

    let posts_post: Posts = client
        .post(GET_MULTI_POSTS_ADDR)
        .json(&create_posts)
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", posts_post);

    // text()返回的是String
    let posts_post_map = client
        .post(GET_MULTI_POSTS_ADDR)
        .json(&posts_map)
        .send()
        .await?
        .text()
        .await?;
    println!("map {}", posts_post_map);

    // 3. serde_json宏方式
    let posts_post_serde: Posts = client
        .post(GET_MULTI_POSTS_ADDR)
        .json(&serde_json::json!({
            "userId": 111,
            "title": "serde title",
            "body": "serde body"
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("serde_json {:?}", posts_post_serde);

    Ok(())
}
