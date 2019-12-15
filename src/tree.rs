use std::cmp::Ordering;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Ord>{
    age: T,
    name: String,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
pub struct Tree<T: Ord>{
    root: Link<T>,
}

impl<T: Ord> Node<T>{
    fn new(age: T, name: String) ->Self{
        Node{
            age: age,
            name: name,
            left: None,
            right: None,
        }
    }

    fn contains(&self, age: T, name: String) -> bool {
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

impl<T: Ord> Tree<T>{
    pub fn new() -> Self {
        Tree { root: None}
    }

    pub fn insert(&mut self, age: T, name: String){
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

    pub fn contains(&self, age:T, name:String) -> bool {
        match self.root {
            None => false,
            Some(ref root) => root.contains(age, name),
        }
    }

    pub fn erase(&self, age: T, name: String) -> Option<Node<T>> {
        unimplemented!()
    }


    pub fn print(&self) {
        //
    }

    pub fn reset(&mut self){
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

impl<T: Ord> Drop for Tree<T> {

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
        tree.insert(1, "a".to_string());
        tree.insert(2, "b".to_string());
        tree.insert(3, "c".to_string());

        assert_eq!(tree.contains(1, "a".to_string()), true);
        assert_eq!(tree.contains(2, "b".to_string()), true);
        assert_eq!(tree.contains(3, "f".to_string()), false);
        assert_eq!(tree.contains(4, "d".to_string()), false);
    }
}
