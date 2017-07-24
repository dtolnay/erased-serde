use std::mem;

pub struct Any {
    ptr: *mut (),
    drop: fn(*mut ()),
    fingerprint: Fingerprint,
}

impl Any {
    pub fn new<T>(t: T) -> Self {
        let ptr = Box::into_raw(Box::new(t));
        Any {
            ptr: ptr as *mut (),
            drop: |ptr| drop(unsafe { Box::from_raw(ptr as *mut T) }),
            fingerprint: Fingerprint::of::<T>(),
        }
    }

    pub fn view<T>(&mut self) -> &mut T {
        if self.fingerprint != Fingerprint::of::<T>() {
            panic!("invalid cast");
        }
        let ptr = self.ptr as *mut T;
        unsafe { &mut *ptr }
    }

    pub fn take<T>(self) -> T {
        if self.fingerprint != Fingerprint::of::<T>() {
            panic!("invalid cast");
        }
        let ptr = self.ptr as *mut T;
        let box_t = unsafe { Box::from_raw(ptr) };
        mem::forget(self);
        *box_t
    }
}

impl Drop for Any {
    fn drop(&mut self) {
        (self.drop)(self.ptr);
    }
}

#[derive(Eq, PartialEq)]
struct Fingerprint {
    size: usize,
    align: usize,
    id: usize,
}

impl Fingerprint {
    fn of<T>() -> Fingerprint {
        Fingerprint {
            size: mem::size_of::<T>(),
            align: mem::align_of::<T>(),
            id: Fingerprint::of::<T> as usize,
        }
    }
}
