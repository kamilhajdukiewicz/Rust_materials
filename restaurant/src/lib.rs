mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();    
    }

    fn cook_order() {
        //it's not possible even with use
        //hosting::add_to_waitlist();
    }
}

fn deliver_order() {

}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //with use crate::front_of_house::hosting is possible
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}