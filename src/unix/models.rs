pub struct EntryPoint {
    pub name: String,
    pub typee: String,
    pub exec: String,
    pub icon: String,
    pub categories: String,
}

impl EntryPoint {
    pub fn get_content(&mut self) -> String {
        let mut content = String::from("[Desktop Entry]\n");
        content.push_str(&format!("Name={}\n", self.name));
        content.push_str(&format!("Type={}\n", self.typee));
        content.push_str(&format!("Exec={}\n", self.exec));
        content.push_str(&format!("Icon={}\n", self.icon));
        content.push_str(&format!("Categories={};\n", self.categories));
        return content;
    }
}
