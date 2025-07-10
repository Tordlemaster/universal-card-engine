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

///For passing # and N variables between the routines of a Choice and its conditions
///TODO May cause issues with nested Choices
pub struct TempVars {
    pub pound: Option<String>,
    pub n: Option<String>
}

impl TempVars {
    pub const fn new() -> TempVars {
        TempVars { pound: None, n: None }
    }

    pub fn get_pound(&self) -> &Option<String> {
        &self.pound
    }
    pub fn set_pound(&mut self, s: &String) {
        if self.pound.is_none() {
            self.pound = Some(s.clone());
        }
        else {
            panic!("Should not be setting TempVars when they are already set!")
        }
    }
    pub fn get_n(&self) -> &Option<String> {
        &self.n
    }
    pub fn set_n(&mut self, s: &String) {
        if self.n.is_none() {
            self.n = Some(s.clone());
        }
        else {
            panic!("Should not be setting TempVars when they are already set!")
        }
    }
    pub fn clear(&mut self) {
        self.pound = None;
        self.n = None;
    }
}