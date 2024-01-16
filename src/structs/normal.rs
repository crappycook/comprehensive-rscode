#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

#[allow(unused)]
fn change_person(p: &mut Person) {
    p.age = 1;
    p.name = "Robot".to_string();
}

#[allow(unused)]
fn show_person() {
    let p = &mut Person {
        name: "JoJo".to_string(),
        age: 18,
    };
    change_person(p);
    println!("{:?}", p)
}
