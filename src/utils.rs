#[macro_export]
macro_rules! marker_component {
    ($name: ident) => {
        #[derive(::bevy::prelude::Component, Default)]
        pub struct $name;
    };
}