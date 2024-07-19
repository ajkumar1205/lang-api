use libsql::Rows;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct User {
    id: usize,
    token: String,
    email: String,
    requests: usize,
    remaining: usize,
}

impl User {
    pub async fn from(rows: &mut Rows) -> HashMap<String, Self> {
        let mut map = HashMap::new();

        while let Ok(Some(r)) = rows.next().await {
            let mut user = Self {
                id: 0,
                token: String::new(),
                email: String::new(),
                requests: 10,
                remaining: 10000,
            };

            if let Some(id) = r.get_value(0).unwrap().as_integer() {
                user.id = id.clone() as usize;
            }

            if let Some(email) = r.get_value(2).unwrap().as_text() {
                user.email = email.clone();
            }

            if let Some(token) = r.get_value(1).unwrap().as_text().to_owned() {
                user.token = token.clone();
            }

            if let Some(req) = r.get_value(4).unwrap().as_integer() {
                user.requests = req.clone() as usize;
            }

            if let Some(rem) = r.get_value(5).unwrap().as_integer() {
                user.remaining = rem.clone() as usize;
            }

            map.insert(user.token.clone(), user.clone());
        }
        map
    }
}
