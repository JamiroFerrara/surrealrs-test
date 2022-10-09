use crate::surreal::Person;

mod surreal;

fn main () -> Result<(), std::io::Error> {
    tokio_main();
    Ok(())
}

#[tokio::main]
async fn tokio_main() -> Result<(), Box<dyn std::error::Error>> {
    let jamiro = Person::new("Jamiro".to_string(), 25).insert().await?;
    let alessia = Person::new("Alessia".to_string(), 26).insert().await?;
    let sarah = Person::new("Sarah".to_string(), 26).insert().await?;

    jamiro.kiss(&alessia).await?;
    jamiro.kiss(&sarah).await?;
    jamiro.get_kissed().await?;

    Ok(())
}
