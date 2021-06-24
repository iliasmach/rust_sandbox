use std::any::Any;
use std::borrow::Borrow;
use std::ops::Deref;
use std::collections::HashMap;

trait BaseEntity {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
struct User {
    id: i64,
}

impl BaseEntity for User {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
struct Profile {
    name: String,
    email: String,
}

impl BaseEntity for Profile {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<'a> From<&'a Box<dyn BaseEntity>> for &'a User {
    fn from(e: &'a Box<dyn BaseEntity>) -> Self {
        let a = e.as_any();
        a.downcast_ref::<User>().unwrap()
    }
}

impl<'a> From<&'a mut Box<dyn BaseEntity>> for &'a mut User {
    fn from(e: &'a mut Box<dyn BaseEntity>) -> Self {
        let mut a = e.as_any_mut();
        a.downcast_mut::<User>().unwrap()
    }
}


fn main() {
    let mut map: HashMap<String, Box<dyn BaseEntity>> = HashMap::new();

    map.insert("user".to_string(), Box::new(User { id: 5 }));
    map.insert("profile".to_string(), Box::new(Profile { name: "".to_string(), email: "".to_string() }));

    let user: &mut User = map.get_mut("user").unwrap().into();
    println!("{:?}", user);
    let profile: &Profile = map.get_mut("profile").unwrap().as_any().downcast_ref::<Profile>().unwrap();
    println!("{:?}", profile);
}
