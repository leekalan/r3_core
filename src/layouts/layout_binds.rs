use std::marker::PhantomData;

use reqs::feedable::{feedable, Fed, Req, Unfed};

use crate::prelude::*;

pub type EmptyBinds<L> = <L as HasLayoutBinds<0>>::EmptyBinds;

feedable!(LayoutRequiresBind => LayoutFeedableBind
    in 'a for 'r where 'r: 'a
    => fn(render_pass: &'a mut wgpu::RenderPass<'r>)
    => {
        fn covariant 'a: 'b, 'b in 'a -> 'b where 'r: 'a + 'b;
    }
);

pub trait HasLayoutBinds<const OFFSET: u32> {
    type EmptyBinds: 'static;
    fn empty_binds() -> Self::EmptyBinds;
}

pub struct LayoutBind<B: Bind + 'static, const GROUP: u32> {
    __bind: PhantomData<B>,
}
impl<B: Bind + 'static, const GROUP: u32> LayoutBind<B, GROUP> {
    pub const fn new() -> Self {
        Self {
            __bind: PhantomData,
        }
    }
}

impl<B: Bind + 'static, const GROUP: u32> Default for LayoutBind<B, GROUP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const GROUP: u32> HasLayoutBinds<GROUP> for Void {
    type EmptyBinds = Void;
    fn empty_binds() -> Self::EmptyBinds {
        Void
    }
}
impl<B: Bind + 'static, const N: u32> HasLayoutBinds<N> for LayoutBind<B, N> {
    type EmptyBinds = Unfed<LayoutBind<B, N>>;
    fn empty_binds() -> Self::EmptyBinds {
        Unfed(LayoutBind::new())
    }
}
impl<B: Bind + 'static, const N: u32, R: HasLayoutBinds<{ N + 1 }>> HasLayoutBinds<N>
    for (LayoutBind<B, N>, R)
{
    type EmptyBinds = (Unfed<LayoutBind<B, N>>, R::EmptyBinds);
    fn empty_binds() -> Self::EmptyBinds {
        (Unfed(LayoutBind::new()), R::empty_binds())
    }
}

impl<'r, B: Bind + 'static, const G: u32> LayoutRequiresBind<'r> for LayoutBind<B, G> {
    type Req<'a>
        = &'a B
    where
        'r: 'a;
    type Out = Self;

    fn give<'a>(self, render_pass: &'a mut wgpu::RenderPass<'r>, req: Self::Req<'a>) -> Self::Out
    where
        'r: 'a,
    {
        render_pass.set_bind_group(G, req.bind_group(), &[]);
        self
    }

    fn covariant<'a: 'b, 'b>(req: Self::Req<'a>) -> Self::Req<'b>
    where
        'r: 'a + 'b,
    {
        req
    }
}
