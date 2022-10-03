use crate::person::Person;

mod person;

fn main () -> Result<(), std::io::Error> {
    tokio_main();
    Ok(())
}

#[tokio::main]
async fn tokio_main() {
    let person = Person {
        name: "John".to_string(),
        age: 20,
    };
    println!("Adding person.. {:?}", person);
    match person.insert().await {
        Ok(_) => println!("Person added!"),
        Err(e) => println!("Error adding person: {:?}", e),
    }

    Person::get_all().await.unwrap();
}
