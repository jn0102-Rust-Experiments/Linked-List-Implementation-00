use std::{cell::RefCell, ptr, rc::Rc};

#[derive(Debug)]
pub enum ListOperationErr {
    IndexOutOfBounds,
    OperationOnEmptyList,
    UnexpectedError,
    ElementNotFound,
}

pub const UNEXPECTED_ERR: ListOperationErr = ListOperationErr::UnexpectedError;

#[derive(Debug, Clone)]
struct ListNode<T> {
    content: Rc<RefCell<T>>,
    linked_node: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    fn new(content: Rc<RefCell<T>>) -> Rc<RefCell<ListNode<T>>> {
        Rc::new(RefCell::new(ListNode {
            content,
            linked_node: None,
        }))
    }

    fn link_to(&mut self, node: Rc<RefCell<ListNode<T>>>) {
        match self.linked_node {
            Some(ref mut n) => n.clone_from(&node),
            None => {
                self.linked_node = Some(node.clone());
            }
        }
    }
}

/// ### Summary
/// Represents a list of items of type `T`
pub trait List<T>: IntoIterator + Clone {
    /// add an item to the end of the list
    /// #### Params
    /// - `item` - a reference to the item to add
    fn add(&mut self, item: Rc<RefCell<T>>);

    /// add an item to the end of the list
    /// #### Params
    /// - `item` - the item to add
    fn add_raw(&mut self, item: T);

    /// insert an item at a specific index in the list
    /// #### Params
    /// - `item` - a reference to the item to insert
    fn insert_at(&mut self, item: Rc<RefCell<T>>, index: i64) -> Result<(), ListOperationErr>;

    /// insert an item at a specific index in the list
    /// #### Params
    /// - `item` - the item to insert
    fn insert_raw_at(&mut self, item: T, index: i64) -> Result<(), ListOperationErr>;

    /// get a reference to the item at the specified index
    /// #### Params
    /// - `index` - the index to lookup
    fn get(&self, index: i64) -> Result<Rc<RefCell<T>>, ListOperationErr>;

    /// removes the specified `item` from the list
    /// #### Params
    /// - `item` - a reference to the item to be removed
    fn remove(&mut self, item: Rc<RefCell<T>>) -> Result<(), ListOperationErr>;

    /// removes the item at the specified `index`
    /// #### Params
    /// - `index` - the index of the item to remove
    fn remove_at(&mut self, index: i64) -> Result<Rc<RefCell<T>>, ListOperationErr>;

    /// checks whether `item` is in the list
    /// #### Params
    /// - `item` - the item to lookup
    fn contains(&self, item: Rc<RefCell<T>>) -> bool;

    /// #### Returns
    /// `true` if the list is empty
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
    tail: Option<Rc<RefCell<ListNode<T>>>>,
    size: i64,
}

impl<T> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut clone = LinkedList::new();
        let mut cur = self.head.clone();
        loop {
            match cur {
                Some(c) => {
                    clone.add(c.clone().borrow().content.clone());
                    cur = c.borrow().linked_node.clone();
                }
                None => break,
            }
        }
        clone
    }
}

impl<T> LinkedList<T> {
    /// Constructs an empty `LinkedList<T>`
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Check index bounds
    pub fn index_check(&self, index: i64) -> Result<(), ListOperationErr> {
        if index < 0 || self.size <= index {
            Err(ListOperationErr::IndexOutOfBounds)
        } else {
            Ok(())
        }
    }

    /// Removes the first element of the list
    pub fn shift(&mut self) -> Result<Rc<RefCell<T>>, ListOperationErr> {
        // if head
        match self
            .head
            .clone()
            .ok_or(ListOperationErr::OperationOnEmptyList)?
            .borrow()
            .linked_node
            .clone()
        {
            Some(n) => {
                self.size -= 1;
                let tmp = Some(
                    self.head
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow()
                        .content
                        .clone(),
                );
                self.head.replace(n.clone());
                tmp.ok_or(UNEXPECTED_ERR)
            }
            None => {
                // if list size = 1
                // reset
                self.size -= 1;
                self.head.take();
                Ok(self
                    .tail
                    .take()
                    .ok_or(UNEXPECTED_ERR)?
                    .borrow()
                    .content
                    .clone())
            }
        }
    }

    /// Removes the last element of the list
    pub fn pop(&mut self) -> Result<Rc<RefCell<T>>, ListOperationErr> {
        // if tail
        // set node before tail node as tail
        if self.size == 1 {
            // if list size = 1
            // reset
            self.size -= 1;
            self.head.take();
            Ok(self
                .tail
                .take()
                .ok_or(UNEXPECTED_ERR)?
                .borrow()
                .content
                .clone())
        } else {
            self.tail.replace(self.get_node_at(self.size - 2)?);

            let n = self.tail.clone().ok_or(UNEXPECTED_ERR)?;

            let tmp = n
                .borrow_mut()
                .linked_node
                .take()
                .ok_or(UNEXPECTED_ERR)?
                .borrow()
                .content
                .clone();
            self.size -= 1;

            Ok(tmp)
        }
    }

    /// Get list node at `index`
    fn get_node_at(&self, index: i64) -> Result<Rc<RefCell<ListNode<T>>>, ListOperationErr> {
        self.index_check(index)?;

        let mut cur = self.head.clone();
        for _ in 0..index {
            cur.replace(
                cur.clone()
                    .ok_or(UNEXPECTED_ERR)?
                    .borrow()
                    .linked_node
                    .clone()
                    .ok_or(UNEXPECTED_ERR)?,
            );
        }
        cur.ok_or(UNEXPECTED_ERR)
    }
}

pub struct LinkedListIterator<T> {
    current: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = Rc<RefCell<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(ref c) => {
                let result = Some(c.clone().borrow_mut().content.clone());

                match c.clone().borrow().linked_node.clone() {
                    Some(nxt) => {
                        // set `current.linked_node` as current
                        self.current.replace(nxt);
                    }
                    None => {
                        // set `current` to `None`
                        self.current.take();
                    }
                }

                result
            }
            None => None,
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = Rc<RefCell<T>>;

    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator {
            current: self.head.clone(),
        }
    }
}

impl<T> List<T> for LinkedList<T> {
    fn add(&mut self, item: Rc<RefCell<T>>) {
        // init node for new item
        let node = ListNode::new(item);

        match self.tail {
            Some(ref mut tail) => {
                // on non-empty list
                tail.borrow_mut().link_to(node.clone());
                tail.clone_from(&node);
            }
            None => {
                // On empty, use the same node for head and tail
                self.tail = Some(node);
                self.head = self.tail.clone();
            }
        }

        // increment size
        self.size += 1;
    }

    fn add_raw(&mut self, item: T) {
        self.add(Rc::new(RefCell::new(item)));
    }

    fn insert_at(&mut self, item: Rc<RefCell<T>>, index: i64) -> Result<(), ListOperationErr> {
        self.index_check(index)?;

        if index == 0 {
            // if head
            self.head.replace(Rc::new(RefCell::new(ListNode {
                content: item,
                linked_node: self.head.clone(),
            })));
        } else if index == self.size - 1 {
            // if tail
            self.add(item);
        } else {
            let prev = self.get_node_at(index - 1)?;
            let n0 = prev.borrow().linked_node.clone().ok_or(UNEXPECTED_ERR)?;
            prev.borrow_mut().link_to(Rc::new(RefCell::new(ListNode {
                content: item,
                linked_node: Some(n0),
            })));
        }

        Ok(())
    }

    fn insert_raw_at(&mut self, item: T, index: i64) -> Result<(), ListOperationErr> {
        self.insert_at(Rc::new(RefCell::new(item)), index)
    }

    fn get(&self, index: i64) -> Result<Rc<RefCell<T>>, ListOperationErr> {
        self.index_check(index)?;

        let mut iter = self.clone().into_iter();

        for _ in 0..index {
            iter.next();
        }

        iter.next().clone().ok_or(UNEXPECTED_ERR)
    }

    fn contains(&self, item: Rc<RefCell<T>>) -> bool {
        let clone = self.clone();
        let mut result = false;

        for i in clone {
            if ptr::eq(item.as_ref(), i.as_ref()) {
                result = true;
            }
        }

        result
    }

    fn remove(&mut self, item: Rc<RefCell<T>>) -> Result<(), ListOperationErr> {
        let mut cur = self.head.clone();

        // check if empty
        if self.is_empty() {
            Err(UNEXPECTED_ERR)
        }
        // if head
        else if ptr::eq(
            cur.clone().ok_or(UNEXPECTED_ERR)?.borrow().content.as_ref(),
            item.as_ref(),
        ) {
            match cur.ok_or(UNEXPECTED_ERR)?.borrow().linked_node.clone() {
                Some(linked) => {
                    self.head.replace(linked);
                }
                None => {
                    self.head.take();
                }
            }

            self.size -= 1;
            Ok(())
        } else {
            let prev_node;

            // look for node before the node matching `item`
            loop {
                if ptr::eq(
                    cur.clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow()
                        .linked_node
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow()
                        .content
                        .as_ref(),
                    item.as_ref(),
                ) {
                    prev_node = Some(cur);
                    break;
                } else {
                    cur.replace(
                        cur.clone()
                            .ok_or(UNEXPECTED_ERR)?
                            .borrow()
                            .linked_node
                            .clone()
                            .ok_or(UNEXPECTED_ERR)?,
                    );
                }
            }

            if let Some(prev_node) = prev_node {
                // if tail
                if ptr::eq(
                    prev_node
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow()
                        .linked_node
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .as_ref(),
                    self.tail.clone().ok_or(UNEXPECTED_ERR)?.as_ref(),
                ) {
                    self.tail.replace(prev_node.clone().ok_or(UNEXPECTED_ERR)?);
                } else {
                    let target_node = prev_node
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow()
                        .linked_node
                        .clone();
                    prev_node
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow_mut()
                        .linked_node
                        .replace(
                            target_node
                                .ok_or(UNEXPECTED_ERR)?
                                .borrow()
                                .linked_node
                                .clone()
                                .ok_or(UNEXPECTED_ERR)?,
                        );
                }

                self.size -= 1;
                Ok(())
            } else {
                Err(ListOperationErr::ElementNotFound)
            }
        }
    }

    fn remove_at(&mut self, index: i64) -> Result<Rc<RefCell<T>>, ListOperationErr> {
        self.index_check(index)?;

        if index == 0 {
            // if head
            self.shift()
        } else if index == self.size - 1 {
            // if tail
            self.pop()
        } else {
            // otherwise...
            // get node before specified `index`
            let n = self.get_node_at(index - 1)?;
            // get node after specified `index`
            let n_after = self.get_node_at(index)?.borrow().linked_node.clone();

            self.size -= 1;
            let result = {
                n.borrow()
                    .linked_node
                    .clone()
                    .ok_or(UNEXPECTED_ERR)?
                    .borrow()
                    .content
                    .clone()
            };

            if let Some(nxt) = n_after {
                // link previous node to after node
                n.borrow_mut().linked_node.replace(nxt);
            }

            Ok(result)
        }
    }

    fn is_empty(&self) -> bool {
        self.size < 1
    }
}
