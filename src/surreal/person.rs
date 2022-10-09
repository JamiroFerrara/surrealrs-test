use serde::{Deserialize, Serialize};
use crate::surreal::post::{surreal_post, surreal_get};

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
        let query = format!("CREATE person:{0} SET name='{0}', age={1}", self.name, self.age);
        surreal_post(query).await?;
        Ok(())
    }

    pub async fn get_all() -> std::result::Result<(), reqwest::Error> {
        let query = "select * from person".to_string();
        let res = surreal_get(query).await?;

        let res = serde_json::from_value::<Vec<Person>>(res).unwrap();
        println!("{:#?}", res);
        
        Ok(())
    }

    fn get() -> std::result::Result<(), reqwest::Error> {
        todo!()
    }

    fn delete() -> std::result::Result<(), reqwest::Error> {
        todo!()
    }

    fn delete_all() -> std::result::Result<(), reqwest::Error> {
        todo!()
    }
}
