use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split("&") { // split at &
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i]; // get string before =
                val = &sub_str[i + 1..]; // get string after =
           }

           // assign to our hashmap
           // .entry to find a key
           // .and_modify to update existing key
            // uses closures |existing| to provide existing value and we match
            // on two condition 2. either append to existing vector
            // 1. or modify existing memory to a new instace of a multiple value vector
           // .or_insert to insert new value
           data.entry(key)
               .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(existing_vec) => existing_vec.push(val)
               })
               .or_insert(Value::Single(val));


        }


        QueryString { data }
    }
}
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}