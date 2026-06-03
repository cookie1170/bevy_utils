//! Allows creating a component group where only one of the components in the group is allowed to
//! exist on an entity

pub use bevy_utils_macros::group;

use bevy::ecs::component::ComponentId;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use std::marker::PhantomData;

/// A component hook used by the `#[group]` macro for `on_insert`
pub fn component_group_on_insert<TComp: Component, TGroup: Send + Sync + 'static>(
    mut world: DeferredWorld,
    ctx: HookContext,
) {
    let current = world
        .entity(ctx.entity)
        .get::<ComponentGroup<TGroup>>()
        .map(|g| g.current);

    if let Some(current_id) = current {
        world.commands().entity(ctx.entity).remove_by_id(current_id);
    }

    let id = world.component_id::<TComp>().unwrap();

    world
        .commands()
        .entity(ctx.entity)
        .insert(ComponentGroup::<TGroup> {
            current: id,
            _phantom: Default::default(),
        });
}

/// A component hook used by the `#[group]` macro for `on_remove`
pub fn component_group_on_remove<TComp: Component, TGroup: Send + Sync + 'static>(
    mut world: DeferredWorld,
    ctx: HookContext,
) {
    world
        .commands()
        .entity(ctx.entity)
        .remove::<ComponentGroup<TGroup>>();
}

#[derive(Component, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[component(immutable)]
struct ComponentGroup<T> {
    current: ComponentId,
    _phantom: PhantomData<T>,
}
