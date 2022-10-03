use crate::person::Person;

mod person;

fn main () -> Result<(), std::io::Error> {
    tokio_main();
    Ok(())
}

#[tokio::main]
async fn tokio_main() {
    Person::new("Jamiro".to_string(), 25).insert().await.unwrap();
    Person::new("Alessia".to_string(), 26).insert().await.unwrap();
    Person::new("Mauro".to_string(), 45).insert().await.unwrap();
    Person::get_all().await.unwrap();
}
