use std::mem;

pub struct Any {
    ptr: *mut (),
    drop: fn(*mut ()),
    fingerprint: Fingerprint,
}

// These functions are all unsafe. They are not exposed to the user. Declaring
// them as `unsafe fn` would not make the rest of erased-serde any safer or more
// readable.
impl Any {
    // This is unsafe -- caller must not hold on to the Any beyond the lifetime
    // of T.
    //
    // Example of bad code:
    //
    //    let s = "bad".to_owned();
    //    let a = Any::new(&s);
    //    drop(s);
    //
    // Now `a.view()` and `a.take()` return references to a dead String.
    pub(crate) fn new<T>(t: T) -> Self {
        let ptr = Box::into_raw(Box::new(t));
        Any {
            ptr: ptr as *mut (),
            drop: |ptr| drop(unsafe { Box::from_raw(ptr as *mut T) }),
            fingerprint: Fingerprint::of::<T>(),
        }
    }

    // This is unsafe -- caller is responsible that T is the correct type.
    pub(crate) fn view<T>(&mut self) -> &mut T {
        if self.fingerprint != Fingerprint::of::<T>() {
            panic!("invalid cast");
        }
        let ptr = self.ptr as *mut T;
        unsafe { &mut *ptr }
    }

    // This is unsafe -- caller is responsible that T is the correct type.
    pub(crate) fn take<T>(self) -> T {
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
            // This is not foolproof -- theoretically Rust or LLVM could
            // deduplicate some or all of these methods. But in practice it's
            // great and I am comfortable relying on this in debug mode to catch
            // bugs early.
            id: Fingerprint::of::<T> as usize,
        }
    }
}
