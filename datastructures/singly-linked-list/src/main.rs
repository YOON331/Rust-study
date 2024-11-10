use std::mem;

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    // 빈 단일 연결 리스트 생성
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    // head로 x 값 추가하기 
    fn push_head(&mut self, x: T) {
        let new_node = Node { 
            element: x,
            next: mem::replace(&mut self.head, None),
        };

        self.head = Some(Box::new(new_node));
    }

    // head에서 x 값 삭제 - 삭제한 값 반환
    fn pop_head(&mut self) -> Option<T> {
        match  mem::replace(&mut self.head, None) {
            Some(val) => {
                self.head = val.next;
                Some(val.element)
            },
            None => {None}, // head가 가리키는 값이 none이면 None 반환
        }
    }
}

fn main() {
    // i32 타입을 갖는 singly linked list 만들어보기 (node)
    let mut head = SinglyLinkedList::<i32>::new();
    println!("{:?}", head);


    head.push_head(4);
    println!("{:?}", head);
    
    head.push_head(7);
    println!("{:?}", head);

    head.push_head(10);
    println!("{:?}", head);
    let mut val = SinglyLinkedList::pop_head(&mut head);
    println!("pop head, val = {:?}", val);
    println!("current list = {:?}", head);

    val = SinglyLinkedList::pop_head(&mut head);
    println!("pop head, val = {:?}", val);
    println!("current list = {:?}", head);

    val = SinglyLinkedList::pop_head(&mut head);
    println!("pop head, val = {:?}", val);
    println!("current list = {:?}", head);

    val = SinglyLinkedList::pop_head(&mut head);
    println!("pop head, val = {:?}", val);
    val = SinglyLinkedList::pop_head(&mut head);
    println!("pop head, val = {:?}", val);
}