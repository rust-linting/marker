use core::fmt::Debug;
use core::marker::PhantomData;

type AliasTy = Box<u32>;

pub union UnionItem {
    _f: f32,
    _i: i32,
}

pub struct AllowSync<T> {
    _data: PhantomData<T>,
}
unsafe impl<T> Sync for AllowSync<T> {}

trait InterestingSuperTrait {
    type C: Default;
}

trait InterestingTrait<T>: InterestingSuperTrait {
    type A: Default;
    fn use_alias(&self) {
        //#[clippy::dump]
        let _ty: Self::C = Self::C::default();
        let _ty: <Self as InterestingTrait<T>>::A = Self::A::default();
    }
}

struct Duck;

impl InterestingSuperTrait for Duck {
    type C = u32;
}

impl InterestingTrait<u32> for Duck {
    type A = u32;
    fn use_alias(&self) {
        #[clippy::dump]
        let _ty: Self::C = Self::C::default();
        let _ty: <Self as InterestingTrait<u32>>::A = Self::A::default();
    }
}

pub fn param_type<T: Debug>(t: T) {
    let _ty_generic: T = t;
}

fn u32_to_f32(_: u32) -> f32 {
    0.0
}

fn main() {
    let mut x = 0;
    let _ty: u32 = 10;
    let _ty_primitive: Option<(u8, u16, u32, u64, u128, usize)> = None;
    let _ty_primitive: Option<(i8, i16, i32, i64, i128, isize)> = None;
    let _ty_primitive: Option<(char, bool, f32, f64)> = None;
    let _ty_sequence: [u32; 1] = [10];
    let slice: &[u32] = &[10];
    let _ty_sequence: &[u32] = slice;
    let _ty_ptr: Option<(&'static str, *const i32, *mut i32)> = None;
    let _ty_fn_item: fn(u32) -> f32 = u32_to_f32;
    let _ty_closure = || x = 9;

    // The path `u32_to_f32` actually has the function item type, it has
    // to be stored in a value, to become a function pointer
    let fn_ptr: fn(u32) -> f32 = u32_to_f32;
    let _ty_fn_ptr: fn(u32) -> f32 = fn_ptr;

    // Interestingly, rustc substitutes the type directly and the semantic type
    // doesn't show the type alias.
    let _ty_simple_alias: AliasTy = AliasTy::new(12);

    let _ty_adt: String = String::new();
    let _ty_dyn_simple: Option<Box<dyn Debug>> = None;
    let _ty_dyn_complex: Option<Box<dyn Iterator<Item = i32> + 'static>> = None;
}
