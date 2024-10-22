use std::{cell::RefCell, rc::Rc};

use bytes::buf::Limit;

#[derive(Debug)]
enum List{

    Cons(i32, RefCell<Rc<List>>),
    Nil,
}



// pub fn testRc() {

//     let a = List::Cons(5,Box::new(List::Cons(3,Box::new(List::Nil))));
//     let b = List::Cons(3,Box::new(a));
//     // let c = List::Cons(4,Box::new(a));
// }


impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {

        match self {
            List::Cons(_,item ) => Some(item),
            List::Nil => None,
        }
    }
}



pub trait Messenger {
    fn send(&self,msg : &str);
}

pub struct LimitTracker<'a,T:Messenger> {

    messenger:&'a T,
    value: usize,
    max: usize,
    
}

impl<'a,T> LimitTracker<'a,T>
where 
    T:Messenger,{

        pub fn new(messenger:&'a T,max:usize) -> LimitTracker<'a,T>{
            LimitTracker{
                messenger,
                value:0,
                max,
            }
        }

        pub fn set_value(&mut self,value:usize) {
         self.value = value;
         let percentage = self.value as f64 / self.max as f64;

         if percentage >= 1.0 {

            self.messenger.send("Error you are over quota");
         }   else {
            self.messenger.send("Your are good");
         }
    }


}


#[cfg(test)]
mod tests{
    use std::cell::RefCell;

    use super::{LimitTracker, Messenger};

    
    struct MockMessenger{
        sent_messeges: RefCell<Vec<String>>, // here we have wrapped Vec<String> inside RefCell, bcz trait Messenger takes an immutable ref to self and we want someway to know if msgs were added
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messeges: RefCell::new(vec![]),//vec![],
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self,msg : &str) {
            self.sent_messeges.borrow_mut().push(String::from(msg));//withour Refcell we wouldnt be able to push msg here
        }
    }

    #[test]
    fn it_sends_an_warning_msg(){

        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(110);
        assert_eq!(mock_messenger.sent_messeges.borrow().len(),1);

    }

}



