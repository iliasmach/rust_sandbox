use std::any::Any;
use std::collections::HashMap;
use std::borrow::Borrow;

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

trait BaseEntity: AsAny+ Send + Sync {
    type ID;
    fn new(id: Self::ID) -> Self where Self: Sized;
}

#[derive(Debug)]
struct User {
    id: i64,
}

#[derive(Debug)]
struct Car {
    id: String,
}

impl BaseEntity for User {
    type ID = i64;



    fn new(id: Self::ID) -> Self {
        User { id }
    }
}

impl AsAny for User {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AsAny for Car {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl BaseEntity for Car {
    type ID = String;

    fn new(id: String) -> Self {
        Car { id }
    }
}


trait BaseRepository : AsAny
{
    type Model;
    type ID;

    fn find(&self, id: Self::ID) -> Self::Model;
}

#[derive(Debug)]
struct UserRepository {}

#[derive(Debug)]
struct CarRepository {}

impl BaseRepository for UserRepository {
    type Model = Box<dyn BaseEntity<ID=i64>>;
    type ID = i64;



    fn find(&self, id: i64) -> Self::Model {
        Box::new(User { id })
    }
}

impl AsAny for UserRepository {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AsAny for CarRepository {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BaseRepository for CarRepository {
    type Model = Box<dyn BaseEntity<ID=String>>;
    type ID = String;



    fn find(&self, id: Self::ID) -> Self::Model {
        Box::new(Car { id })
    }
}

type IDRepo<T> = dyn BaseRepository<ID=T, Model=Box<dyn BaseEntity<ID=T>>>;

struct App {
    pub user_repo: Box<IDRepo<i64>>,
    pub car_repo: Box<IDRepo<String>>,
}


fn main() {
    let mut app = App {
        user_repo: Box::new(UserRepository {}),
        car_repo: Box::new(CarRepository {}),
    };

    let user_repo = app.user_repo.as_any().downcast_ref::<UserRepository>().unwrap();
    let car_repo = app.car_repo.as_any().downcast_ref::<CarRepository>().unwrap();

    let a = user_repo.find(4);
    let b: &User = a.as_any().downcast_ref().unwrap();

    let car_a = car_repo.find("Toyota".to_string());
    let car_b: &Car = car_a.as_any().downcast_ref().unwrap();
    println!("{:?}", b);
    println!("{:?}", car_b);
}