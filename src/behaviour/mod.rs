use crate::humans::Human;

pub trait Action {
    fn do_action<R, F>(&self, human: &mut Human, world : &mut R, f : F)
        where 
        F : FnOnce();
}



