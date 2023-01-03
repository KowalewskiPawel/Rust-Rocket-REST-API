use crate::appconfig;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sqlite::State;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Dao {
    name: String,
    description: String,
    address: String,
    creator: String,
}
impl Default for Dao {
    fn default() -> Dao {
        Dao {
            name: String::new(),
            description: String::new(),
            address: String::new(),
            creator: String::new(),
        }
    }
}

#[get("/v1/test/sayhi/<name>")]
pub fn sayhi(name: String) -> String {
    format!("Hi {}, I see you are an inexperienced noob", name)
}

pub fn create(dao: &Dao) -> String {
    let conn = sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists

    let result: String = "SUCCESS".to_string();
    let _statement = match conn.execute(format!(
        "INSERT INTO daos values ('{}', '{}', '{}', '{}')",
        &*dao.name, &*dao.description, &*dao.address, &*dao.creator
    )) {
        Ok(statement) => statement,
        Err(e) => return format!("Problem running query: {:?}", e),
    };

    result
}

fn fill_user_with_daojson(dao: &Json<Dao>) -> Dao {
    let mut dao_instance = Dao::default();
    dao_instance.name = dao.name.to_owned();
    dao_instance.description = dao.description.to_owned();
    dao_instance.address = dao.address.to_owned();
    dao_instance.creator = dao.creator.to_owned();
    dao_instance
}

#[get("/v1/test/query/<name>")]
pub fn query(name: String) -> String {
    appconfig::check_dbfile(appconfig::DATABASE_FILE);

    let conn = sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists

    let mut result: String = "".to_string();

    let statement = match conn.prepare("SELECT * FROM test where name = ?1") {
        Ok(statement) => statement,
        Err(e) => return format!("Problem running query: {:?}", e),
    };

    let mut t = match statement.bind(1, &*name) {
        Ok(statement) => statement,
        Err(e) => return format!("Problem binding params: {:?}", e),
    };

    while let State::Row = t.next().unwrap() {
        let name_record = t.read::<String>(0).unwrap();
        let description_record = t.read::<String>(1).unwrap();
        result += format!(
            "Name: {}, Description: {}",
            &name_record, &description_record
        )
        .as_str();
    }

    if result == "" {
        result += "No records found";
    }

    result
}

#[get("/v1/test/query_all")]
pub fn query_all() -> Json<Vec<Dao>> {
    appconfig::check_dbfile(appconfig::DATABASE_FILE);

    let conn = sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists

    let mut result: Vec<Dao> = Vec::new();

    let query = "SELECT * FROM daos";

    let mut dao_new: Dao = Dao::default();

    conn.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            let current_value = value.unwrap();
            match name {
                "name" => dao_new.name = String::from(current_value),
                "desc" => dao_new.description = String::from(current_value),
                "addr" => dao_new.address = String::from(current_value),
                "creator" => {
                    dao_new.creator = String::from(current_value);
                    let dao_copy = Dao { name: dao_new.name.clone(), description: dao_new.description.clone(), address: dao_new.address.clone(), creator: dao_new.creator.clone() };
                    result.push(dao_copy);
                },
                &_ => ()
            }
        }
        true
    })
    .unwrap();

    Json(result)
}

#[post("/v1/test/create", format = "json", data = "<dao>")]
pub fn web_create(dao: Json<Dao>) -> String {
    create(&fill_user_with_daojson(&dao))
}
