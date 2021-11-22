use regex::Regex;

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
}