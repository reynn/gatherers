use crate::Result;
use std::{any::Any, collections::HashMap};

#[derive(Default, Debug)]
pub struct Cookie {
    crumbs: HashMap<String, String>,
}

impl Cookie {
    pub fn parse(input: &'_ str) -> Result<Cookie> {
        let mut hmap = Cookie::default();
        let input = String::from(input);
        for crumb in input.split(';') {
            let crumb_pieces: Vec<_> = crumb.split('=').collect();
            let key = crumb_pieces[0].trim().to_string();
            if crumb_pieces.len() > 1 {
                hmap.crumbs.insert(key, crumb_pieces[1].trim().into());
            } else {
                hmap.crumbs.insert(key, "true".into());
            }
        }

        Ok(hmap)
    }

    pub fn get(&self, key: &'_ str) -> String {
        if let Some(value) = self.crumbs.get(key) {
            value.clone()
        } else {
            "".into()
        }
        // self.crumbs.get(key).or_else(|| Some("")).unwrap()
    }
}

impl From<Cookie> for String {
    fn from(c: Cookie) -> Self {
        c.crumbs
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("; ")
    }
}
