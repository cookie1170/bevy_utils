#[cfg(test)]
mod group {
    use bevy::prelude::*;
    use bevy_utils::prelude::*;

    struct Group;

    #[group(Group)]
    #[derive(Component)]
    pub struct MyComponent;

    #[group(Group)]
    #[derive(Component)]
    struct AnotherComponent;

    #[test]
    fn it_works() {
        let mut world = World::new();
        let mut entity = world.spawn(MyComponent);
        assert!(entity.get_components::<&MyComponent>().is_ok());
        entity.insert(AnotherComponent);
        assert!(entity.get_components::<&AnotherComponent>().is_ok());
        assert!(entity.get_components::<&MyComponent>().is_err());
        entity.insert(MyComponent);
        assert!(entity.get_components::<&AnotherComponent>().is_err());
        assert!(entity.get_components::<&MyComponent>().is_ok());
    }
}
