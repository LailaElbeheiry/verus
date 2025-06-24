use super::prelude::*;

verus! {

#[rustc_diagnostic_item = "verus::vstd::guards::guards"]
pub open spec fn guards<P, R: GuardedBy<P>>(p: P, r: R) -> (b: bool) {
    r.guarded_by(p)
}

pub trait GuardedBy<B> where Self: Sized, B: Sized {
    spec fn guarded_by(self, b: B) -> bool;
}

impl GuardedBy<u32> for u32 {
    open spec fn guarded_by(self, b: u32) -> bool {
        true  // TODO: implement this

    }
}

// use super::simple_pptr::*;
// impl<V> GuardedBy<PointsTo<V>> for PPtr<V> {
//     spec fn guarded_by(&self, p: &PointsTo<V>) -> bool {
//         self.points_to(p)
//     }
// }
} // verus!
