use std::error::Error;

mod members;

fn main() -> Result<(), Box<Error>>{
    let members_json = std::fs::File::open("members.json")?;
    let members : members::Members = serde_json::from_reader(members_json)?;

    println!("{:?}", &members);

    Ok(())
}