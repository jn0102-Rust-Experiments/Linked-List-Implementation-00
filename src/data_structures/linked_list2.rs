use super::linked_list::{List, ListOperationErr, UNEXPECTED_ERR};
use std::{cell::RefCell, ptr, rc::Rc};

#[derive(Debug, Clone)]
struct ListNode2<T> {
    content: Rc<RefCell<T>>,
    linked_nodes: (
        Option<Rc<RefCell<ListNode2<T>>>>,
        Option<Rc<RefCell<ListNode2<T>>>>,
    ),
}

impl<T: std::fmt::Debug> ListNode2<T> {
    /// Creates a new node with no linked nodes
    /// ### Returns
    /// a reference to the newly created node
    fn new(content: Rc<RefCell<T>>) -> Rc<RefCell<ListNode2<T>>> {
        Rc::new(RefCell::new(ListNode2 {
            content,
            linked_nodes: (None, None),
        }))
    }

    /// Breaks the link between this node and the node linked through `self.linked_nodes.0`
    /// ### Returns
    /// a reference to the linked node (if any)
    fn break_link0(&mut self) -> Option<Rc<RefCell<ListNode2<T>>>> {
        let n0 = self.linked_nodes.0.take();
        n0.clone()?.borrow_mut().linked_nodes.1.take();
        n0
    }

    /// Breaks the link between this node and the node linked through `self.linked_nodes.1`
    /// ### Returns
    /// a reference to the linked node (if any)
    fn break_link1(&mut self) -> Option<Rc<RefCell<ListNode2<T>>>> {
        let n1 = self.linked_nodes.1.take();
        n1.clone()?.borrow_mut().linked_nodes.0.take();
        n1
    }
}

pub struct LinkedList2<T: std::fmt::Debug> {
    head: Option<Rc<RefCell<ListNode2<T>>>>,
    tail: Option<Rc<RefCell<ListNode2<T>>>>,
    size: usize,
}

impl<T: std::fmt::Debug> LinkedList2<T> {
    /// Constructs an empty `LinkedList2<T>`
    pub fn new() -> Self {
        LinkedList2 {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Check index bounds
    pub fn index_check(&self, index: usize) -> Result<(), ListOperationErr> {
        if self.size <= index {
            Err(ListOperationErr::IndexOutOfBounds)
        } else {
            Ok(())
        }
    }

    /// Removes the first element of the list
    pub fn shift(&mut self) -> Result<Rc<RefCell<T>>, ListOperationErr> {
        // if head
        let after_head = self
            .head
            .clone()
            .ok_or(ListOperationErr::OperationOnEmptyList)?
            .borrow()
            .linked_nodes
            .1
            .clone();
        match after_head {
            Some(n) => {
                // set node after head node as head
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
                n.borrow_mut().break_link0();
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
        let tail_prev = self
            .tail
            .clone()
            .ok_or(ListOperationErr::OperationOnEmptyList)?
            .borrow()
            .linked_nodes
            .0
            .clone();

        match tail_prev {
            Some(n) => {
                // set node before tail node as tail
                self.size -= 1;
                let tmp = Some(
                    self.tail
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?
                        .borrow()
                        .content
                        .clone(),
                );
                self.tail.replace(n.clone());

                n.borrow_mut().break_link1();
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

    /// Get list node at `index`
    fn get_node_at(&self, index: usize) -> Result<Rc<RefCell<ListNode2<T>>>, ListOperationErr> {
        self.index_check(index)?;

        let mut cur = self.head.clone();
        for _ in 0..index {
            cur.replace(
                cur.clone()
                    .ok_or(UNEXPECTED_ERR)?
                    .borrow()
                    .linked_nodes
                    .1
                    .clone()
                    .ok_or(UNEXPECTED_ERR)?,
            );
        }
        cur.ok_or(UNEXPECTED_ERR)
    }

    /// Links `node0` with `node1` through `node0`'s link 1 and `node1`'s link 0
    fn link_nodes(
        node0: Rc<RefCell<ListNode2<T>>>,
        node1: Rc<RefCell<ListNode2<T>>>,
    ) -> (
        Option<Rc<RefCell<ListNode2<T>>>>,
        Option<Rc<RefCell<ListNode2<T>>>>,
    ) {
        let node0_old_link = node0.borrow_mut().break_link1();
        let node1_old_link = node1.borrow_mut().break_link0();

        node0.borrow_mut().linked_nodes.1.replace(node1.clone());
        node1.borrow_mut().linked_nodes.0.replace(node0.clone());

        (node0_old_link, node1_old_link)
    }
}

#[derive(Debug)]
pub struct LinkedList2Iterator<T> {
    current: Option<Rc<RefCell<ListNode2<T>>>>,
}

impl<T: std::fmt::Debug> Clone for LinkedList2Iterator<T> {
    fn clone(&self) -> Self {
        Self {
            current: self.current.clone(),
        }
    }
}

impl<T: std::fmt::Debug> Iterator for LinkedList2Iterator<T> {
    type Item = Rc<RefCell<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.current.clone()?;
        let result = Some(c.clone().borrow_mut().content.clone());

        match c.borrow().linked_nodes.1.clone() {
            Some(nxt) => {
                // set `current.linked_node` as current
                self.current.replace(nxt);
            }
            None => {
                // set `current` to `None`
                self.current.take();
            }
        };

        result
    }
}

impl<T: std::fmt::Debug> IntoIterator for LinkedList2<T> {
    type Item = Rc<RefCell<T>>;

    type IntoIter = LinkedList2Iterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedList2Iterator {
            current: self.head.clone(),
        }
    }
}

impl<T: std::fmt::Debug> Clone for LinkedList2<T> {
    fn clone(&self) -> Self {
        let mut clone = LinkedList2::new();
        let mut cur = self.head.clone();
        loop {
            match cur {
                Some(c) => {
                    clone.add(c.clone().borrow().content.clone());
                    cur = c.borrow().linked_nodes.1.clone();
                }
                None => break,
            }
        }
        clone
    }
}

impl<T: std::fmt::Debug> List<T> for LinkedList2<T> {
    fn add(&mut self, item: Rc<RefCell<T>>) {
        // init node for new item
        let node = ListNode2::new(item.clone());

        match self.tail {
            Some(ref mut tail) => {
                // on non-empty list
                Self::link_nodes(tail.clone(), node.clone());
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

    fn insert_at(&mut self, item: Rc<RefCell<T>>, index: usize) -> Result<(), ListOperationErr> {
        self.index_check(index)?;

        if index == 0 {
            // if head
            self.head.replace(Rc::new(RefCell::new(ListNode2 {
                content: item,
                linked_nodes: (None, self.head.clone()),
            })));
            // increment size
            self.size += 1;
        } else if index == self.size - 1 {
            // if tail
            self.add(item);
        } else {
            let orig = self.get_node_at(index)?;
            let prev = orig.borrow_mut().break_link0();
            Self::link_nodes(
                prev.ok_or(UNEXPECTED_ERR)?,
                Rc::new(RefCell::new(ListNode2 {
                    content: item,
                    linked_nodes: (None, Some(orig)),
                })),
            );
            // increment size
            self.size += 1;
        }

        Ok(())
    }

    fn insert_raw_at(&mut self, item: T, index: usize) -> Result<(), ListOperationErr> {
        self.insert_at(Rc::new(RefCell::new(item)), index)
    }

    fn get(&self, index: usize) -> Result<Rc<RefCell<T>>, ListOperationErr> {
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
            let _ = self.shift();

            self.size -= 1;
            Ok(())
        } else {
            let mut target_node = Err(ListOperationErr::ElementNotFound);
            // `cur.content` != `item`
            cur = cur.ok_or(UNEXPECTED_ERR)?.borrow().linked_nodes.1.clone();

            // look for node matching `item`
            loop {
                let _cur = cur.clone().ok_or(UNEXPECTED_ERR)?;
                if ptr::eq(_cur.clone().borrow().content.as_ref(), item.as_ref()) {
                    target_node = Ok(_cur.clone());
                    break;
                }

                match _cur.clone().borrow().linked_nodes.1.clone() {
                    Some(nxt) => {
                        cur.replace(nxt);
                    }
                    None => break,
                }
            }

            let target_node = target_node?;

            if ptr::eq(
                self.tail.clone().ok_or(UNEXPECTED_ERR)?.as_ref(),
                target_node.clone().as_ref(),
            ) {
                // if tail
                let _tail = self.tail.clone().ok_or(UNEXPECTED_ERR)?;
                self.tail.replace(
                    _tail
                        .borrow()
                        .linked_nodes
                        .0
                        .clone()
                        .ok_or(UNEXPECTED_ERR)?,
                );
                _tail.borrow_mut().break_link1();
            } else {
                let (n0, n1) = target_node.borrow().linked_nodes.clone();
                Self::link_nodes(n0.ok_or(UNEXPECTED_ERR)?, n1.ok_or(UNEXPECTED_ERR)?);
            }

            self.size -= 1;
            Ok(())
        }
    }

    fn remove_at(&mut self, index: usize) -> Result<Rc<RefCell<T>>, ListOperationErr> {
        self.index_check(index)?;

        if index == 0 {
            // if head
            self.shift()
        } else if index == self.size - 1 {
            // if tail
            self.pop()
        } else {
            // otherwise...
            // get node
            let n = self.get_node_at(index)?;
            let result = n.borrow().content.clone();
            let (n0, n1) = n.borrow().linked_nodes.clone();
            Self::link_nodes(n0.ok_or(UNEXPECTED_ERR)?, n1.ok_or(UNEXPECTED_ERR)?);

            self.size -= 1;

            Ok(result)
        }
    }

    fn is_empty(&self) -> bool {
        self.size < 1
    }

    fn size(&self) -> usize {
        self.size
    }
}
