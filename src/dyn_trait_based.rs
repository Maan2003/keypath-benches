use std::{borrow::Cow, marker::PhantomData, rc::Rc};

#[derive(Clone)]
pub enum KeyPath<T: 'static, U: Clone + 'static> {
    Rc(Rc<dyn KeyPathImpl<Root = T, Data = U>>),
    Static(&'static dyn KeyPathImpl<Root = T, Data = U>),
}

impl<T: 'static, U: Clone + 'static> KeyPath<T, U> {
    pub fn get<'a>(&self, root: &'a T) -> Cow<'a, U> {
        match self {
            KeyPath::Rc(r) => r.get(root),
            KeyPath::Static(s) => s.get(root),
        }
    }

    pub fn put(&self, root: &mut T, data: U) {
        match self {
            KeyPath::Rc(r) => r.put(root, data),
            KeyPath::Static(s) => s.put(root, data),
        }
    }
}

pub trait KeyPathImpl {
    type Root;
    type Data: Clone;
    fn get<'a>(&self, root: &'a Self::Root) -> Cow<'a, Self::Data>;
    fn put(&self, root: &mut Self::Root, data: Self::Data);
}

pub struct FieldKeyPath<T, U, Get: Fn(&T) -> &U, GetMut: Fn(&mut T) -> &mut U> {
    get: Get,
    get_mut: GetMut,
    _gens: PhantomData<*const (T, U)>,
}

pub macro keypath {
    ($root:ident :: $($field:tt).+) => {
        keypath! {<$root>::$($field).+}
    },
    (<$root:ty> :: $($field:tt).+) => {
        KeyPath::Static(&FieldKeyPath::<$root, _, _, _> {
            get: |root| &root.$($field).+,
            get_mut: |root| &mut root.$($field).+,
            _gens: PhantomData,
        })
    },
}

impl<T, U, Get, GetMut> KeyPathImpl for FieldKeyPath<T, U, Get, GetMut>
where
    U: Clone,
    Get: Fn(&T) -> &U,
    GetMut: Fn(&mut T) -> &mut U,
{
    type Root = T;
    type Data = U;

    fn get<'a>(&self, root: &'a Self::Root) -> Cow<'a, Self::Data> {
        Cow::Borrowed((self.get)(root))
    }

    fn put(&self, root: &mut Self::Root, data: Self::Data) {
        *(self.get_mut)(root) = data;
    }
}
