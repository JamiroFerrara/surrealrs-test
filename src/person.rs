use reqwest::*;
use serde::{Deserialize, Serialize};

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub result: Vec<Person>,
    pub status: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Person {
        let id = format!("person:{}", name);
        Person { id, name, age }
    }

    pub async fn insert(&self) -> std::result::Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let query = format!("CREATE person:{0} SET name='{0}', age={1}", self.name, self.age);
        let _ = client.post("http://localhost:8000/sql")
            .basic_auth("root", Some("root"))
            .header("Content-Type", "application/json")
            .header("NS", "test")
            .header("DB", "test")
            .body(query)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_all() -> std::result::Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let query = "SELECT * FROM person";
        let res: Response = client.post("http://localhost:8000/sql")
            .basic_auth("root", Some("root"))
            .header("Content-Type", "application/json")
            .header("NS", "test")
            .header("DB", "test")
            .body(query)
            .send()
            .await?;

        let root: Root = res.json().await?;
        println!("Response: {:#?}", root);

        Ok(())
    }
}
