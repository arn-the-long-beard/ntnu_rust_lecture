struct Company {
    pub name: String,
    /// In Billion
    pub value: u32,
}

fn display_data(company: &Company) {
    println!("Name : {} ", company.name);
    println!("Market Cap : {} ", company.name);
    println!("Rating {}", rate_business(&company.value))
}

fn rate_business(value: &u32) -> &'static str {
    match value {
        0 => "Bankrupt",
        1..=100 => "Weak",
        101..=350 => "Ok",
        351..=700 => "Nice",
        701..=1000 => "Maybe a bubble",
        _ => "Like seven sisters",
    }
}

fn rebuild_business(_: Company) -> Company {
    Company {
        name: "Facebook 2.0".to_string(),
        value: 850,
    }
}

fn update_name(company: &mut Company, new_name: &str) {
    company.name = new_name.to_string();
}

fn main() {
    let mut facebook = Company {
        name: "FaceMash".to_string(),
        value: 0,
    };
    // 1 - immutability
    facebook.name = "Facebook".to_string();
    facebook.value = 900;

    println!("{} new name is cool ", facebook.name);

    // 2 - Reference with & to read data
    // Try to assign value with `&` and see what happens:D
    display_data(&facebook);

    println!("{} is super old, we need rebranding. ", facebook.name); // <- Macro stuff does auto reference  https://stackoverflow.com/questions/30450399/does-println-borrow-or-own-the-variable

    // <- if we do not use mut here, compiler will say NO
    facebook.name = "Meta".to_string();
    facebook.value -= 231;
    println!("{} is an awesome name.", facebook.name);

    // <-- we move `facebook` inside the scope of the `rebuild_business` function so we cannot access it anymore .
    let mut new_facebook = rebuild_business(facebook);

    // println!("{} is still alive ?", facebook.name); <-- get moved error value
    display_data(&new_facebook);

    let update = &mut new_facebook; // Can only make a single mutable reference.
                                    // let update2 = &mut new_facebook;  throw error here
                                    // display_data(&new_facebook); Cannot read while writing :D
    update.name = "Facebook 3.0".to_string();
    println!("{} is an awesome name.", new_facebook.name);

    let update2 = &mut new_facebook;
    update2.name = "Facebook 4.0".to_string();

    update_name(&mut new_facebook, "Facebook 3000");

    println!("{} is an awesome name.", new_facebook.name);
}
