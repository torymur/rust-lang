use std::collections::HashMap;

#[derive(Debug)]
struct Org {
    // {<department>: [<name1>, <name2>]}
    org: HashMap<String, Vec<String>>,
}

impl Org {
    fn add_person(&mut self, text: &str) {
        let (department, name) = Self::parse_text(&text);
        println!("Department is {:?}, name is {:?}", department, name);
        let names = self.org.entry(department).or_insert(vec![]);
        names.push(name);
    }
    fn parse_text(text: &str) -> (String, String) {
        let res: Vec<&str> = text.split("to").collect();
        let left_part = res[0].trim();
        let department = res[1].trim();
        
        let res: Vec<&str> = left_part.split("Add").collect();
        let name = res[1].trim();
        (department.to_string(), name.to_string())
    }
    // list all people in the department sorted alphabetically
    fn list_by_department(&self, department: &str) {
        if !self.org.contains_key(department) {
            println!(
                "Org has many departments like {:?}, but {:?} is not one of them",
                self.org.keys(), department
            );
            return
        }

        let mut copy_names = Vec::new();
        if let Some(names) = self.org.get(department) {
            copy_names = names.to_vec()
        };
        copy_names.sort();
        println!("Department found: {:?}, people {:?}", department, copy_names);
    }
    // list all people in the org sorted alphabetically
    fn list_all(&self) {
        for (k, v) in self.org.iter() {
            let mut copy_v = v.to_vec();
            copy_v.sort();
            println!("Department: {:?}, people {:?}", k, copy_v);
        }
    }
}


fn main() {
    let mut o = Org { org: HashMap::new() };
    let commands = vec![
        "Add Sally to Engineering",
        "Add Amir to Sales",
        "Add Kate to SA",
        "Add Robert to Sales",
        "Add Olga to Engineering",
        "Add Nicola to SA",
        "Add Bob to SA",
    ];
    for command in commands {
        o.add_person(&command);
    }
    o.list_all();
    o.list_by_department("SA");
    // department doesn't exist
    o.list_by_department("HR");
}
