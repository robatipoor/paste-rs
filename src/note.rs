// use mysql::value::{
//     from_value,
//     Value,
// };
// use time::Timespec;

// impl Note {

//     pub fn from_row(row: &Vec<Value>) -> Note {
//         Note {
//             id: from_value(&row[0]),
//             code: from_value(&row[1]),
//             time_created: from_value(&row[2]),
//             data: from_value(&row[3])
//         }
//     }

//     pub fn find_by_code(code: &str) -> Option<Note> {
//         let conn = Database::new().connect();
//         let mut stmt = match conn.prepare("SELECT * FROM paste \
//                                        WHERE code = ? LIMIT 1") {
//             Ok(stmt) => stmt,
//             Err(e) => {
//                 println!("Error, failed to find Note: {}", e);
//                 return None
//             }
//         };

//         for row in stmt.execute(&[&code.to_string()]).unwrap() {
//             let row = row.unwrap();
//             return Some(Note::from_row(&row))
//         }

//         None
//     }

//     pub fn insert(note: &mut Note) {
//         let conn = Database::new().connect();
//         let mut stmt = conn.prepare("
//             INSERT INTO paste (
//                 code,
//                 time_created,
//                 data
//             )
//             VALUES (?, ?, ?);").unwrap();

//         for row in stmt.execute(&[&note.code, &note.time_created, &note.data]).unwrap() {
//             let row = row.unwrap();
//             let id = from_value(&row[0]);
//             note.id = id;
//             break;
//         }
//     }

//     pub fn _all() -> Vec<Note> {
//         let mut notes: Vec<Note> = Vec::new();
//         let conn = Database::new().connect();
//         let mut stmt = conn.prepare("SELECT id, code, time_created, data FROM paste;").unwrap();

//         for row in stmt.execute([]).unwrap() {
//             let row = row.unwrap();
//             let note = Note::from_row(&row);
//             notes.push(note);
//         }
//         notes
//     }
// }

use crate::db::Database;
use sqlx::mysql::MySqlPool;
use std::time::Instant;

#[derive(Clone, PartialEq, Debug)]
pub struct Note {
    pub id: i32,
    pub code: String,
    pub time_created: Instant,
    pub data: String,
}

impl Note {
    pub fn new(id: i32, code: String, data: String) -> Self {
        Note {
            id,
            code,
            time_created: Instant::now(),
            data,
        }
    }

    pub fn find_by_id(pool :MySqlPool,id: &str) -> Option<Self> {
        None
    }

    pub fn find_by_code(pool :MySqlPool,code: &str) -> Option<Self> {
        None
    }

    pub fn find_all(pool :MySqlPool) -> Option<Vec<Self>> {
        None
    }

    pub fn insert(&self,pool :MySqlPool) -> crate::Result<()> {
        Ok(())
    }
}
