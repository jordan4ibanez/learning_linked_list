use std::fmt;

struct Value<T> {
    pub value: T,
    pub next: Box<Option<Value<T>>>
}
impl <T: Copy + std::fmt::Debug> Value <T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: Box::new(None),
        }
    }
}

impl <T: Copy + std::fmt::Debug> fmt::Debug for Value<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value").field("value", &self.value).field("next", &self.next).finish()
    }
}


struct LinkedList <T>{
    size: usize,
    head: Option<Value<T>>
}

impl <T: Copy + std::fmt::Debug + std::fmt::Display> LinkedList <T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, value: T) {

        let mut found = false;

        let mut next = &mut self.head;

        while !found {
            match next {
                Some(next_reference) => {
                    next = &mut *next_reference.next;
                },
                None => {
                    *next = Some(Value::new(value));
                    found = true;

                    self.size += 1;
                }
            }
        }
    }

    pub fn raw_print(&self) {

        let mut found = false;

        let mut next = &self.head;
        print!("{{");
        let mut count = 0;
        while !found {

            match next {
                Some(next_reference) => {
                    count += 1;
                    print!("{}", next_reference.value);
                    if count < self.size {
                        print!(",");
                    }
                    next = &*next_reference.next;
                },
                None => {
                    println!("}}");
                    found = true;
                },
            }
        }
    }
}

fn main() {

    let mut my_list: LinkedList<u32> = LinkedList::new();

    for i in 0..6 {
        my_list.insert(i);
    }    

    my_list.raw_print();

    println!("list size: {}", my_list.len());

}
