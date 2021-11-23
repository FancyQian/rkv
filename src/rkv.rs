use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum KeyValueAction {
    KvNull,
    KvSet,
    KvGet,
    KvDel,
}

#[derive(Debug)]
pub struct KeyValueCmd {
    pub action: KeyValueAction,
    pub key: String,
    pub value: String,
}

impl KeyValueCmd {
    pub fn new(cmd: &str) -> Result<KeyValueCmd, &'static str > {
        let regex = Regex::new(r"\s.*\s").unwrap();
        if regex.is_match(cmd) == false {
            Err("Invalid command!")
        } else {
            let cmd: Vec<&str> = Regex::new(r"\s").unwrap().split(cmd).collect();
            let act;

            match cmd[0] {
                "SET" => act = KeyValueAction::KvSet,
                "GET" => act = KeyValueAction::KvGet,
                "DEL" => act = KeyValueAction::KvDel,
                _ => act = KeyValueAction::KvNull
            }

            let k = String::from(cmd[1]);
            let v = String::from(cmd[2]);

            let ret = KeyValueCmd {
                action: act,
                key: k,
                value: v
            };

            Ok(ret)
        }
    }

    pub fn run(self, db: & mut HashMap<String, String>) -> Vec<u8> {
        let mut ret_vec = Vec::new();

        let mut str2vec = |x: String| {
            for value in x.as_bytes().iter() {
                ret_vec.push(*value)
            }
         };

        match self.action {
            KeyValueAction::KvSet => {
                let buf = format!("{}: {}\n",self.key, self.value);
                db.insert(self.key, self.value);
                str2vec(buf);
            },
            KeyValueAction::KvGet => {
                let buf = db.get(&self.key);
                match buf {
                    Some(buf) => str2vec(buf.clone()),
                    None => str2vec("Cannot found!\n".to_string())
                }
            },
            KeyValueAction::KvDel => {
                let buf = db.remove(&self.key);
                match buf {
                    Some(_buf) => str2vec("Removed!\n".to_string()),
                    None => str2vec("Cannot found!!!\n".to_string())
                }
            },
            _ => str2vec("Unsupport!!!\n".to_string())
        }
        ret_vec
    }
}