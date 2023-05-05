use std::collections::HashMap;

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    pub fn add_user_to_department(&mut self, user: &str, department: &str) {
        let department = department.to_string();
        self.departments
            .entry(department)
            .and_modify(|people| {
                let user = user.to_string();
                if !people.contains(&user) {
                    people.push(user);
                }
            })
            .or_insert(vec![user.to_string()]);
    }

    pub fn get_people_department(&self, department: &str) -> &Vec<String> {
        &self.departments[department]
    }

    pub fn get_all_people_by_department(&self) -> &HashMap<String, Vec<String>> {
        &self.departments
    }
}
