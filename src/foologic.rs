mod foobox;
use foobox::FooBox;

pub trait FooLogic {
    // Implement logical 'and' 
    //fn and(&self, other: Box<dyn FooBox>) -> Box<dyn FooBox>;
    fn and(&self) -> Box<dyn FooBox>;
}

