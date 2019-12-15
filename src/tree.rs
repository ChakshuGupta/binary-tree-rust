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

    fn insert(&mut self, node: Node){
        match node.age.cmp(&self.age){
            Ordering::Less => {
                match self.left {
                    None => self.left = Some(Box::new(node)),
                    Some(ref mut left) => left.insert(node),
                }
            },
            Ordering::Greater => {
                match self.right{
                    None => self.right = Some(Box::new(node)),
                    Some(ref mut right) => right.insert(node),
                }
            },
            _ => {},
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
                if let Some(ref right) = self.right {
                    right.contains(age, name)
                } else{
                    false
                }
            }

        }
    }

    fn print(&mut self) {
        print!("[ {:?}, ", self);

        match self.left{
            None => print!("Null "),
            Some(ref mut left) => left.print()
        }
        print!(",");

        match self.right{
            None => print!("Null "),
            Some(ref mut right) => {
                right.print();
            }
        }
        print!("]");
    }

}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} : {} }}", self.age, self.name)
    }
}


impl Tree{
    pub fn new() -> Self {
        Tree { root: None}
    }

    pub fn insert(&mut self, age: i32, name: String){
        match self.root {
            None => self.root = Some(Box::new(Node::new(age, name))),
            Some(ref mut root) => root.insert(Node::new(age, name)),
        }
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


    pub fn print(&mut self) {
        match self.root{
            None => println!("Null"),
            Some(ref mut root) => root.print()
        }
        println!("");
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
        assert_eq!(tree.contains(3, "c".to_string()), true);
        assert_eq!(tree.contains(1, "d".to_string()), false);
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

    #[test]
    fn print(){
        let mut tree = Tree::new();

        assert_eq!(tree.is_empty(), true);
        tree.insert(1, "a".to_string());
        tree.insert(2, "b".to_string());
        tree.insert(3, "c".to_string());

        tree.print();

    }
}
