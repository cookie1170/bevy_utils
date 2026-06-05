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

#[cfg(test)]
mod computed {
    use bevy::prelude::*;
    use bevy::state::app::StatesPlugin;
    use bevy_utils::prelude::*;

    #[derive(States, Debug, Hash, Default, PartialEq, Eq, Clone, Copy)]
    enum States {
        #[default]
        One,
        Two,
        Three,
    }

    #[computed(States)]
    enum Computed {
        #[pat(States::One | States::Three)]
        NotTwo,
        #[pat(States::Two)]
        GreaterThanOne,
    }

    #[test]
    fn it_works() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<States>().add_computed_state::<Computed>();
        assert!(app.world().get_resource::<State<Computed>>().is_none());
        app.world_mut().run_schedule(StateTransition);

        app.insert_resource(NextState::<States>::Pending(States::Two));
        app.world_mut().run_schedule(StateTransition);
        assert_eq!(
            *app.world().resource::<State<Computed>>(),
            Computed::GreaterThanOne
        );

        app.insert_resource(NextState::<States>::Pending(States::Three));
        app.world_mut().run_schedule(StateTransition);
        assert_eq!(*app.world().resource::<State<Computed>>(), Computed::NotTwo);
    }
}
