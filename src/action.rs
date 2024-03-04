use serde::{Deserialize, Serialize};
use strum::Display;

use crate::components::home::Pane;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
  Tick,
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  Refresh,
  Error(String),
  Help,
  Focus(Pane),
  Up,
  Down,
}
