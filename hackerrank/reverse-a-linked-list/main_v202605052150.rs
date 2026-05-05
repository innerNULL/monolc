use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::ptr;

struct SinglyLinkedListNode {
    data: i32,
    next: *mut SinglyLinkedListNode,
}

struct SinglyLinkedList {
    head: *mut SinglyLinkedListNode,
    tail: *mut SinglyLinkedListNode,
}

impl SinglyLinkedListNode {
    pub fn new(data: i32) -> *mut Self {
        Box::into_raw(Box::new(SinglyLinkedListNode {
            data,
            next: ptr::null_mut(),
        }))
    }
}

impl Drop for SinglyLinkedListNode {
    fn drop(&mut self) {
        self.next = ptr::null_mut();
    }
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        SinglyLinkedList { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn insert_node(&mut self, data: i32) {
        unsafe {
            let node = SinglyLinkedListNode::new(data);

            if self.head.is_null() {
                self.head = node;
            } else {
                (*self.tail).next = node;
            }

            self.tail = node;
        }
    }
}

impl Drop for SinglyLinkedList {
    fn drop(&mut self) {
        while !self.head.is_null() {
            unsafe {
                if !self.head.is_null() {
                    let head = Box::from_raw(self.head);
                    self.head = head.next;
                }
            }
        }

        self.tail = ptr::null_mut();
    }
}

fn print_singly_linked_list(head: *const SinglyLinkedListNode, sep: &str, fptr: &mut File) {
    let mut node = head;

    while !node.is_null() {
        unsafe {
            write!(fptr, "{}", (*node).data).ok();

            node = (*node).next;
        }

        if !node.is_null() {
            write!(fptr, "{}", sep).ok();
        }
    }
}

/*
 * Complete the 'reverse' function below.
 *
 * The function is expected to return an INTEGER_SINGLY_LINKED_LIST.
 * The function accepts INTEGER_SINGLY_LINKED_LIST llist as parameter.
 */

/*
 * For your reference:
 *
 * SinglyLinkedListNode {
 *     data: i32,
 *     next: *mut SinglyLinkedListNode,
 * };
 *
 */

fn reverse(
    llist: *const SinglyLinkedListNode
) -> *const SinglyLinkedListNode {
    unsafe {
        let mut dummy_head: *mut SinglyLinkedListNode = {
            SinglyLinkedListNode::new(0)
        };
        let mut node: *mut SinglyLinkedListNode = {
            llist as *mut SinglyLinkedListNode
        };
        while !node.is_null() {
            let next_node: *mut SinglyLinkedListNode = (*node).next;
            (*node).next = (*dummy_head).next;
            (*dummy_head).next = node;
            node = next_node;
        }
        return (*dummy_head).next;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let tests = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..tests {
        let llist_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        let mut llist = SinglyLinkedList::new();

        for _ in 0..llist_count {
            let llist_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

            llist.insert_node(llist_item);
        }

        let llist1 = reverse(llist.head);

        print_singly_linked_list(llist1, " ", &mut fptr);
        writeln!(&mut fptr).ok();
    }
}

