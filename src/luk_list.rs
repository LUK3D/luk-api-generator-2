pub struct LukList{
    pub items:Vec<String>
}


impl LukList{
    pub fn add(&mut self, val:String){
        self.items.push(val);

        for elem in &self.items {
            println!("Adicionando {0}", elem);
        }
    }
}