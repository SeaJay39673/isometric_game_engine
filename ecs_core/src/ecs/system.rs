use crate::{Component, Entity, World};

pub trait SystemParam<'w> {
    type Item;

    fn fetch(world: &'w mut World, entity: &Entity) -> Option<Self::Item>;
}

impl<'w, T: Component> SystemParam<'w> for T {
    type Item = &'w mut T;

    fn fetch(world: &'w mut World, entity: &Entity) -> Option<Self::Item> {
        world.get_component_mut::<T>(entity)
    }
}

impl<'w, T: Component> SystemParam<'w> for Option<T> {
    type Item = Option<&'w mut T>;

    fn fetch(world: &'w mut World, entity: &Entity) -> Option<Self::Item> {
        Some(world.get_component_mut::<T>(entity))
    }
}

macro_rules! impl_system_param_tuple {
    ($($name:ident),+) => {
        impl<'w, $($name),+> SystemParam<'w> for ($($name,)+)
        where
            $($name: SystemParam<'w>,)+
        {
            type Item = ($($name::Item,)+);

            fn fetch(world: &'w mut World, entity: &Entity) -> Option<Self::Item> {
                let world_ptr: *mut World = world;
                unsafe {
                    $(let $name = $name::fetch(&mut *world_ptr, entity)?;)+
                    Some(($($name,)+))
                }
            }
        }
    };
}

macro_rules! impl_system_params {
    (($($t:tt)+)) => {
        impl_system_params!(@acc (), $($t)+);
    };

    (@acc ($($acc:tt)*), $t:ident, $($rest:tt)+) => {
        impl_system_param_tuple!($($acc)* $t);
        impl_system_params!(@acc ($($acc)* $t,), $($rest)+);
    };

    (@acc ($($acc:tt)*), $t:ident) => {
        impl_system_param_tuple!($($acc)* $t);
    };
}

impl_system_params!((
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
));
