use std::cmp::Ordering;
use std::fmt;
use std::mem;

type Link = Option<Box<Node>>;

// Structure for Tree node
pub struct Node{
    age: i32,
    name: String,
    left: Link,
    right: Link,
}

//Structure holding tree root
#[derive(Debug)]
pub struct Tree{
    root: Link,
}


//Node functions
impl Node{

    // Create new node
    fn new(age: i32, name: String) ->Self{
        Node{
            age: age,
            name: name,
            left: None,
            right: None,
        }
    }

    //Insert an element to the tree
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

    //Check if the tree contains a value
    fn contains(&self, age: i32, name: String) -> bool {
        match age.cmp(&self.age){
            Ordering::Equal => {
                match self.name == name{
                    true => true,
                    false => false
                }
            },
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

    //Erase a value from the tree
/*    fn erase(&mut self, age: i32, name: String) -> bool{
        match age.cmp(&self.age){
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.erase(age, name)
                } else{
                    false
                }
            },
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.erase(age, name)
                } else{
                    false
                }
            },
            Ordering::Equal =>{
                match self.name == name{
                    true => {
                        if self.left.is_none() && self.right.is_none(){
                            mem::replace(self, None);
                        }
                        else if self.left.is_none(){
                            let replace = self.right.take();
                            mem::replace(self, replace);
                        } else if self.right.is_none(){
                            let replace = self.left.take();
                            mem::replace(self, replace)
                        }else{
                            // Do Nothing
                        }
                        false
                    },
                    false => false,
                }
            }
        }

    }*/

    //Print the tree
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

//Setting the format for printing the node struct value
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} : {} }}", self.age, self.name)
    }
}


//Tree struct functions
impl Tree{
    //Create new tree
    pub fn new() -> Self {
        Tree { root: None}
    }

    //Insert a value to the tree
    pub fn insert(&mut self, age: i32, name: String){
        match self.root {
            None => self.root = Some(Box::new(Node::new(age, name))),
            Some(ref mut root) => root.insert(Node::new(age, name)),
        }
    }

    //Check if the tree contains a value
    pub fn contains(&self, age: i32, name:String) -> bool {
        match self.root {
            None => false,
            Some(ref root) => root.contains(age, name),
        }
    }

    //Erase a value from the tree
    pub fn erase(&mut self, age: i32, name: String) -> bool {
        /*match self.root {
            None => false,
            Some(ref mut root) => root.erase(age, name),
        }*/
        false
    }


    //Print the tree
    pub fn print(&mut self) {
        match self.root{
            None => println!("Null"),
            Some(ref mut root) => root.print()
        }
        println!("");
    }


    // Reset the tree.
    pub fn reset(&mut self){
        self.root.take();
    }

    // Check if the tree is empty
    pub fn is_empty(&self) ->bool{
        match self.root{
            None => true,
            Some(_) => false,
        }
    }
}


// Destructor
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

// Unit tests for the functions
//
// Run the unit test with the following to see all the print statements-
// cargo test -- --nocapture
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
