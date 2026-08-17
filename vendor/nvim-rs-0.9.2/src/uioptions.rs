//! Options for UI implementations
//!
//! This should be used with the manually implemented
//! [`ui_attach`](crate::neovim::Neovim::ui_attach)
use rmpv::Value;

pub enum UiOption {
  Rgb(bool),
  Override(bool),
  ExtCmdline(bool),
  ExtHlstate(bool),
  ExtLinegrid(bool),
  ExtMessages(bool),
  ExtMultigrid(bool),
  ExtPopupmenu(bool),
  ExtTabline(bool),
  ExtTermcolors(bool),
  TermName(String),
  TermColors(u64),
  TermBackground(String),
  StdinFd(u64),
  StdinTty(bool),
  StdoutTty(bool),
  ExtWildmenu(bool),
}

impl UiOption {
  fn to_value(&self) -> (Value, Value) {
    let name_value = self.to_name_value();
    (name_value.0.into(), name_value.1)
  }

  fn to_name_value(&self) -> (&'static str, Value) {
    match self {
      Self::Rgb(val) => ("rgb", (*val).into()),
      Self::Override(val) => ("override", (*val).into()),
      Self::ExtCmdline(val) => ("ext_cmdline", (*val).into()),
      Self::ExtHlstate(val) => ("ext_hlstate", (*val).into()),
      Self::ExtLinegrid(val) => ("ext_linegrid", (*val).into()),
      Self::ExtMessages(val) => ("ext_messages", (*val).into()),
      Self::ExtMultigrid(val) => ("ext_multigrid", (*val).into()),
      Self::ExtPopupmenu(val) => ("ext_popupmenu", (*val).into()),
      Self::ExtTabline(val) => ("ext_tabline", (*val).into()),
      Self::ExtTermcolors(val) => ("ext_termcolors", (*val).into()),
      Self::TermName(val) => ("term_name", val.as_str().into()),
      Self::TermColors(val) => ("term_colors", (*val).into()),
      Self::TermBackground(val) => ("term_background", val.as_str().into()),
      Self::StdinFd(val) => ("stdin_fd", (*val).into()),
      Self::StdinTty(val) => ("stdin_tty", (*val).into()),
      Self::StdoutTty(val) => ("stdout_tty", (*val).into()),
      Self::ExtWildmenu(val) => ("ext_wildmenu", (*val).into()),
    }
  }
}

#[derive(Default)]
pub struct UiAttachOptions {
  options: Vec<(&'static str, UiOption)>,
}

macro_rules! ui_opt_setters {
  ($( $opt:ident as $set:ident($type:ty) );+ ;) => {
    impl UiAttachOptions {
      $(
        pub fn $set(&mut self, val: $type) -> &mut Self {
          self.set_option(UiOption::$opt(val.into()));
          self
        }
      )+
    }
  }
}

ui_opt_setters! (

  Rgb as set_rgb(bool);
  Override as set_override(bool);
  ExtCmdline as set_cmdline_external(bool);
  ExtHlstate as set_hlstate_external(bool);
  ExtLinegrid as set_linegrid_external(bool);
  ExtMessages as set_messages_externa(bool);
  ExtMultigrid as set_multigrid_external(bool);
  ExtPopupmenu as set_popupmenu_external(bool);
  ExtTabline as set_tabline_external(bool);
  ExtTermcolors as set_termcolors_external(bool);
  TermName as set_term_name(&str);
  TermColors as set_term_colors(u64);
  TermBackground as set_term_background(&str);
  StdinFd as set_stdin_fd(u64);
  StdinTty as set_stdin_tty(bool);
  StdoutTty as set_stdout_tty(bool);
  ExtWildmenu as set_wildmenu_external(bool);
);

impl UiAttachOptions {
  #[must_use]
  pub fn new() -> UiAttachOptions {
    UiAttachOptions {
      options: Vec::new(),
    }
  }

  fn set_option(&mut self, option: UiOption) {
    let name = option.to_name_value();
    let position = self.options.iter().position(|o| o.0 == name.0);

    if let Some(position) = position {
      self.options[position].1 = option;
    } else {
      self.options.push((name.0, option));
    }
  }

  #[must_use]
  pub fn to_value_map(&self) -> Value {
    let map = self.options.iter().map(|o| o.1.to_value()).collect();
    Value::Map(map)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ui_options() {
    let value_map = UiAttachOptions::new()
      .set_rgb(true)
      .set_rgb(false)
      .set_popupmenu_external(true)
      .to_value_map();

    assert_eq!(
      Value::Map(vec![
        ("rgb".into(), false.into()),
        ("ext_popupmenu".into(), true.into()),
      ]),
      value_map
    );
  }
}
