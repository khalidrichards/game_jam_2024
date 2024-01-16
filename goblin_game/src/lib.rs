
use godot::prelude::*;

mod player;

struct LootGoblin;

#[gdextension]
unsafe impl ExtensionLibrary for LootGoblin {}
