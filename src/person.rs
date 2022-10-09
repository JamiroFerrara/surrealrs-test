use reqwest::*;
use serde::{Deserialize, Serialize};

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
            .header("Accept", "application/json")
            .header("NS", "test")
            .header("DB", "test")
            .body(query)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_all() -> std::result::Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let query = "select * from person";
        let res = client.post("http://localhost:8000/sql")
            .basic_auth("root", Some("root"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("NS", "test")
            .header("DB", "test")
            .body(query)
            .send()
            .await?
            .json::<serde_json::Value>().await?[0]
            .get("result").unwrap().clone();

        let res = serde_json::from_value::<Vec<Person>>(res).unwrap();
        println!("{:#?}", res);
        
        Ok(())
    }
}
