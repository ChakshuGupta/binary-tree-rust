use std::cmp::Ordering;
use std::fmt;

type Link = Option<Box<Node>>;

pub struct Node{
    age: i32,
    name: String,
    left: Link,
    right: Link,
}

#[derive(Debug)]
pub struct Tree{
    root: Link,
}

impl Node{
    fn new(age: i32, name: String) ->Self{
        Node{
            age: age,
            name: name,
            left: None,
            right: None,
        }
    }

    fn contains(&self, age: i32, name: String) -> bool {
        match age.cmp(&self.age){
            Ordering::Equal => {
                match self.name == name{
                    true => true,
                    false => false
                }
            }
            Ordering::Less => {
                if let Some(ref left) = self.left {
                    left.contains(age, name)
                } else{
                    false
                }
            },
            Ordering::Greater =>{
                if let Some(ref left) = self.left {
                    left.contains(age, name)
                } else{
                    false
                }
            }

        }
    }

}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ Name: {}, age: {} }}", self.name, self.age)
    }
}


impl Tree{
    pub fn new() -> Self {
        Tree { root: None}
    }

    pub fn insert(&mut self, age: i32, name: String){
        let mut temp_root = &mut self.root;

        while let Some(temp_node) = temp_root {
            match temp_node.age.cmp(&age) {
                Ordering::Equal => {
                    return
                }
                Ordering::Greater => temp_root = &mut temp_node.right,
                Ordering::Less => temp_root = &mut temp_node.left,
            }
        }

        *temp_root = Some(Box::new(Node::new(age, name)));

    }

    pub fn contains(&self, age: i32, name:String) -> bool {
        match self.root {
            None => false,
            Some(ref root) => root.contains(age, name),
        }
    }

    pub fn erase(&self, age: i32, name: String) -> Option<Node> {
        unimplemented!()
    }


    pub fn print(&self) {
        //
    }

    pub fn reset(&mut self){
        self.root.take();
    }

    pub fn is_empty(&self) ->bool{
        match self.root{
            None => true,
            Some(_) => false,
        }
    }
}

impl Drop for Tree{

    fn drop(&mut self) {
        let mut current_node = self.root.take();

        match current_node {
            None => return,
            Some(ref mut node) => {
                node.right.take();
                node.left.take();
            },
        }
    }
}


#[cfg(test)]
mod test{
    use super::Tree;

    #[test]
    fn insert(){
        let mut tree = Tree::new();

        assert_eq!(tree.is_empty(), true);

        tree.insert(1, "a".to_string());
        tree.insert(2, "b".to_string());
        tree.insert(3, "c".to_string());

        println!("The tree is - {:#?}", tree);

        assert_eq!(tree.contains(1, "a".to_string()), true);
        assert_eq!(tree.contains(2, "b".to_string()), true);
        assert_eq!(tree.contains(3, "f".to_string()), false);
        assert_eq!(tree.contains(4, "d".to_string()), false);
    }

    #[test]
    fn reset(){
        let mut tree = Tree::new();

        assert_eq!(tree.is_empty(), true);

        tree.insert(1, "a".to_string());
        tree.insert(2, "b".to_string());
        tree.insert(3, "c".to_string());

        tree.reset();
        match tree.root{
            None => println!("Reset Complete!"),
            _ => panic!("Reset not complete!"),
        }

        assert_eq!(tree.is_empty(), true);
    }
}
