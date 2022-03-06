use std::fmt;

#[derive(Clone)]
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
    pub fn set_next(&mut self, new_next: Box<Option<Value<T>>>){
        self.next = new_next;
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

    pub fn remove(&mut self, index: usize) {

        if index >= self.size {
            return;
        }

        let mut found = false;
        let mut next = &mut self.head;
        let mut count = 0;
        while !found {
            match next {
                Some(next_reference) => {
                    if count == index - 1 {
                        let blah = next_reference.next.clone().unwrap().next;
                        next_reference.set_next(blah);
                        found = true;
                        self.size -= 1;
                    }
                    count += 1;
                    next = &mut *next_reference.next;
                },
                None => (),
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

    pub fn get(&self, index: usize) -> Option<T> {

        if index >= self.size {
            return None;
        }
        
        let mut next = &self.head;
        let mut count = 0;
        loop {
            match next {
                Some(next_reference) => {
                    if count == index {
                        return Some(next_reference.value);
                    }
                    count += 1;
                    next = &*next_reference.next;
                },
                None => (),
            }
        }
    }
}

fn main() {

    let mut my_list: LinkedList<u32> = LinkedList::new();

    for i in 0..6 {
        my_list.insert(i * i);
    }
    
    my_list.remove(5);

    my_list.raw_print();

    println!("The value at 2 is: {}", my_list.get(2).unwrap());

    println!("list size: {}", my_list.len());

}
