use builtin::*;
use builtin_macros::*;
use vstd::simple_pptr::*;
use vstd::prelude::guards;
use vstd::std_specs::option::*;
use vstd::std_specs::result::*;
use core::marker::PhantomData;

verus! {

pub struct X<A> {
    pub data: PhantomData<A>,
}

pub struct S<A> {
    pub a: A,
    pub b: bool,
    pub dummy: X<A>,
}

impl<B> X<B> {
    #[verifier::imaginary_field]
    pub open spec fn v(self) -> (res : B);
}

fn move_a<A>(a : A) -> (res : S<A>)
   ensures
        res.a == a, res.b == true,
   guard_effects
       res.a = a,
       res.dummy->v = a,
{
    S { a: a, b: true, dummy: X { data : PhantomData, } }
}

fn ok_or_wrapper<A>(o : Option<A>) -> (res : Result<A, ()>) 
    guard_effects
        res->Ok_0 = o->0,
{
    o.ok_or(())
}

fn main() {
    let (p, perm) = PPtr::new(42);
    // Γ ≜ { perm |-> p }
    let mut s = move_a(perm);
    // Γ ≜ { s.a |-> p }
    let Tracked(perm) = s.a;
    // Γ ≜ { perm |-> p }
    p.replace(Tracked(&mut perm), 24);
    // Γ ≜ { perm |-> p }
    let perm_opt = Some(Tracked(perm));
    // Γ ≜ { perm_opt->Some_0 |-> p }
    let perm_res = perm_opt.ok_or(());
    // Γ ≜ { perm_opt->Ok_0 |-> p }
    let x = if let Ok(perm_trk) = &perm_res {
        // Γ ≜ { *perm_trk |-> p (blocks ..), perm_opt->Ok_0 |-> p (blocked by perm_trk) }
        let tracked perm = perm_trk.borrow();
        // Γ ≜ { *perm |-> p (blocks perm_trk), *perm_trk |-> p (blocks ..), perm_opt->Ok_0 |-> p (blocked by perm_trk) }
        p.read(Tracked(perm))
        // Γ ≜ { *perm |-> p (blocks perm_trk), *perm_trk |-> p (blocks ..), perm_opt->Ok_0 |-> p (blocked by perm_trk) }
    } else { 0 };
}

} // verus!
