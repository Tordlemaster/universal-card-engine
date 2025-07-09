use std::collections::HashMap;

#[derive(Clone)]
pub struct VarBindSet {
    int_contents: HashMap<String, usize>,
    str_contents: HashMap<String, String>
}

impl VarBindSet {
    pub fn new () -> VarBindSet {
        VarBindSet {
            int_contents: HashMap::new(),
            str_contents: HashMap::new()
        }
    }

    ///Insert a new int variable into the VarBindSet
    pub fn insert_int_var (&mut self, name: &String, value: usize) {
        self.int_contents.insert(name.clone(), value);
    }

    ///Insert a new string variable into the VarBindSet
    pub fn insert_str_var (&mut self, name: &String, value: String) {
        self.str_contents.insert(name.clone(), value);
    }

    ///Remove an existing int variable
    pub fn remove_int_var (&mut self, name: &String) {
        self.int_contents.remove(name);
    }

    ///Remove an existing str variable
    pub fn remove_str_var (&mut self, name: &String) {
        self.str_contents.remove(name);
    }

    ///Look up the value of an existing int variable
    pub fn get_int_val (&self, name: &String) -> Option<&usize> {
        //if name=="#" {
            //"How many?"
        //}
        //else {
            self.int_contents.get(name)
        //}
    }

    //Look up the value of an existing variable
    pub fn get_str_val (&self, name: &String) -> Option<&String> {
        //println!("Looking for variable {}", name);
        self.str_contents.get(name)
    }
}