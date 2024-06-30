

use cfg_if::cfg_if;
use funutd::{math::Float, Rnd};
use core::sync::atomic::{AtomicU32};
pub trait Atomic: Float {
    type Storage: Send + Sync;

    fn storage(t: Self) -> Self::Storage;
    fn store(stored: &Self::Storage, t: Self);
    fn get_stored(stored: &Self::Storage) -> Self;
}

impl Atomic for f32 {
    type Storage = AtomicU32;

    fn storage(t: Self) -> Self::Storage {
        AtomicU32::from(t.to_bits())
    }

    #[inline]
    fn store(stored: &Self::Storage, t: Self) {
        stored.store(t.to_bits(), core::sync::atomic::Ordering::Relaxed);
    }

    #[inline]
    fn get_stored(stored: &Self::Storage) -> Self {
        let u = stored.load(core::sync::atomic::Ordering::Relaxed);
        f32::from_bits(u)
    }
}

cfg_if! {
    if #[cfg(all(target_pointer_width = "32", not(target_pointer_width = "64")))] {

        use num_complex::{ Complex32};
        pub type TargetU = u32;
        pub type TargetAtomicU = AtomicU32;
        pub type TargetF = f32;
        pub type TargetI = i32;
        pub type TargetComplex = Complex32;
        pub const rnd_from_target_u: fn(u32) -> Rnd = Rnd::from_u32; 
        pub const rnd_target_f: fn(&mut Rnd) -> TargetF = Rnd::f32;
        pub const rnd_target_i: fn(&mut Rnd) -> TargetI = Rnd::i32;
        pub const rnd_target_u: fn(&mut Rnd) -> TargetU = Rnd::u32;
    } else if #[cfg(target_pointer_width = "64")] {
        use num_complex::{Complex};
        use core::sync::atomic::{ AtomicU64};
        pub type TargetU = u64;
        pub type TargetAtomicU = AtomicU64;
        pub type TargetF = f64;
        pub type TargetI = i64;
        pub type TargetComplex = Complex<f64>;
        pub const rnd_from_target_u: fn(u64) -> Rnd = Rnd::from_u64;
        pub const rnd_target_f: fn(&mut Rnd) -> TargetF = Rnd::f64;
        pub const rnd_target_i: fn(&mut Rnd) -> TargetI = Rnd::i64;
        pub const rnd_target_u: fn(&mut Rnd) -> TargetU = Rnd::u64;
        impl Atomic for TargetF {
            type Storage = TargetAtomicU;
        
            fn storage(t: Self) -> Self::Storage {
                TargetAtomicU::from(t.to_bits())
            }
        
            #[inline]
            fn store(stored: &Self::Storage, t: Self) {
                stored.store(t.to_bits(), core::sync::atomic::Ordering::Relaxed);
            }
        
            #[inline]
            fn get_stored(stored: &Self::Storage) -> Self {
                let u = stored.load(core::sync::atomic::Ordering::Relaxed);
                TargetF::from_bits(u)
            }
        }
    }
}



