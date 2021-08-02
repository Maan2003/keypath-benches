use std::marker::PhantomData;

pub trait Lens<T, U> {
    fn _with(&self, t: &T, f: &mut dyn FnMut(&U));
    fn _with_mut(&self, t: &mut T, f: &mut dyn FnMut(&mut U));
}

impl<T, U> dyn Lens<T, U> {
    pub fn with<R>(&self, t: &T, f: impl FnOnce(&U) -> R) -> R {
        let mut f = Some(f);
        let mut r = None;
        self._with(t, &mut |u| r = Some(f.take().unwrap()(u)));
        r.unwrap()
    }

    pub fn with_mut<R>(&self, t: &mut T, f: impl FnOnce(&mut U) -> R) -> R {
        let mut f = Some(f);
        let mut r = None;
        self._with_mut(t, &mut |u| r = Some(f.take().unwrap()(u)));
        r.unwrap()
    }
}

pub struct FieldLens<T, U, Get: Fn(&T) -> &U, GetMut: Fn(&mut T) -> &mut U> {
    get: Get,
    get_mut: GetMut,
    _gens: PhantomData<*const (T, U)>,
}

pub macro lens {
    ($root:ident :: $($field:tt).+) => {
        lens! {<$root>::$($field).+}
    },
    (<$root:ty> :: $($field:tt).+) => {
        FieldLens::<$root, _, _, _> {
            get: |root| &root.$($field).+,
            get_mut: |root| &mut root.$($field).+,
            _gens: PhantomData,
        }
    },
}

impl<T, U, Get, GetMut> Lens<T, U> for FieldLens<T, U, Get, GetMut>
where
    U: Clone,
    Get: Fn(&T) -> &U,
    GetMut: Fn(&mut T) -> &mut U,
{
    fn _with(&self, t: &T, f: &mut dyn FnMut(&U)) {
        f((self.get)(t));
    }

    fn _with_mut(&self, t: &mut T, f: &mut dyn FnMut(&mut U)) {
        f((self.get_mut)(t));
    }
}
