use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Score(pub HashMap<u64, u32>);
