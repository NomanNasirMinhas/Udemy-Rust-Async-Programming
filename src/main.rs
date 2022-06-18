//! This A Simple Async Program that demonstrates asynchronous calls to dummy API. It also explains usage of Enums and Structs in Rust. 
use futures::executor::block_on;
use tokio::*;
use reqwest;

enum AsyncCalls{
    /// These are the types of Async calls which can be used with API
    GETCall,
    POSTCall
}

struct AsyncDemo {
    /// This Struct acts as a class taking the base URL of an API in its constructor. 
    url: String,
}

impl AsyncDemo{
    pub async fn async_call(&mut self, endpoint: String, method: AsyncCalls, post_data: Option<String>){
        /// This is the only public function of AsyncDemo struct. This function takes the endpoint+value of API, method to be used and an optional data to be used with POST call
        match method{
            GETCall=>{
                self.get_request(endpoint).await;
            },
            POSTCall=>{
                self.post_request(endpoint, post_data.unwrap());
            }
        }
    }

    async fn get_request(&mut self, endpoint: String){
        /// This function is used only for GET requests
        let req_url = format!("{}/{}", self.url.clone(), endpoint);
        let body = reqwest::get(req_url).await.unwrap().text().await.unwrap();
        println!("GET Response = {}", body);
    }

    async fn post_request(&mut self, endpoint: String, data: String){
        /// This function is used only for POST request
        let req_url = format!("{}/{}", self.url.clone(), endpoint);
        let client = reqwest::Client::new();
        let res = client.post(req_url).body(
            data
        ).send().await.unwrap();
        println!("POST Response = {:?}", res);
    }

}

///We declare our main function to be async using tokio crate.
#[tokio::main]
async fn main() {
    let mut async_call = AsyncDemo{
        url: "https://jsonplaceholder.typicode.com".to_string()
    };

    async_call.get_request("posts/1".to_string()).await;
    async_call.post_request("posts".to_string(), "String".to_string()).await;
}
