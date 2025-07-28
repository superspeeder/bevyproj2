use bevy::ecs::component::Component;

///
/// Entities are split into functional layers:
///
/// `Editor`: Used for editor interfaces (not loaded from scenes)
/// `Debug`: Used for debug overlay stuff (not loaded from scenes)
/// `UI`: Used for game UI
/// `Game`: Used for game graphics
/// 
/// This is largely used for organization, logic separation (like slowing down updates for UI over game stuff), and editor things.
///
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Component)]
pub enum EntityLayer {
    Editor,
    Debug,
    UI,
    Game,
}


