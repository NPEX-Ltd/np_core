mod np_core {
    pub trait Constructable {
        fn new() -> Self;
    }

    pub struct Ref<T> where T: 'static {
        value: Option<T>,
    }
}
