//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{error::CallError, rpc::unpack::TryUnpack, Neovim, Value};
use futures::io::AsyncWrite;
use serde::Serialize;
use std::marker::PhantomData;
impl<W: AsyncWrite + Send + Unpin + 'static> Neovim<W> {
  #[doc = "Since 1"]
  pub async fn nvim_ui_detach(&self) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_ui_detach", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_ui_try_resize(
    &self,
    r#width: i64,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64);
    self
      .call(
        "nvim_ui_try_resize",
        Args(std::marker::PhantomData, r#width, r#height),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_ui_set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, Value);
    self
      .call(
        "nvim_ui_set_option",
        Args(std::marker::PhantomData, r#name, r#value),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_ui_try_resize_grid(
    &self,
    r#grid: i64,
    r#width: i64,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64, i64);
    self
      .call(
        "nvim_ui_try_resize_grid",
        Args(std::marker::PhantomData, r#grid, r#width, r#height),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_ui_pum_set_height(
    &self,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call(
        "nvim_ui_pum_set_height",
        Args(std::marker::PhantomData, r#height),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_ui_pum_set_bounds(
    &self,
    r#width: f64,
    r#height: f64,
    r#row: f64,
    r#col: f64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, f64, f64, f64, f64);
    self
      .call(
        "nvim_ui_pum_set_bounds",
        Args(std::marker::PhantomData, r#width, r#height, r#row, r#col),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_exec(
    &self,
    r#src: &str,
    r#output: bool,
  ) -> Result<String, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, bool);
    self
      .call("nvim_exec", Args(std::marker::PhantomData, r#src, r#output))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_command(
    &self,
    r#command: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_command", Args(std::marker::PhantomData, r#command))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn nvim_get_hl_by_name(
    &self,
    r#name: &str,
    r#rgb: bool,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, bool);
    self
      .call(
        "nvim_get_hl_by_name",
        Args(std::marker::PhantomData, r#name, r#rgb),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn nvim_get_hl_by_id(
    &self,
    r#hl_id: i64,
    r#rgb: bool,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, bool);
    self
      .call(
        "nvim_get_hl_by_id",
        Args(std::marker::PhantomData, r#hl_id, r#rgb),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_get_hl_id_by_name(
    &self,
    r#name: &str,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call(
        "nvim_get_hl_id_by_name",
        Args(std::marker::PhantomData, r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_set_hl(
    &self,
    r#ns_id: i64,
    r#name: &str,
    r#val: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      &'a str,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_set_hl",
        Args(std::marker::PhantomData, r#ns_id, r#name, r#val),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_set_hl_ns(
    &self,
    r#ns_id: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call("nvim_set_hl_ns", Args(std::marker::PhantomData, r#ns_id))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_feedkeys(
    &self,
    r#keys: &str,
    r#mode: &str,
    r#escape_csi: bool,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, &'a str, bool);
    self
      .call(
        "nvim_feedkeys",
        Args(std::marker::PhantomData, r#keys, r#mode, r#escape_csi),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_input(&self, r#keys: &str) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_input", Args(std::marker::PhantomData, r#keys))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_input_mouse(
    &self,
    r#button: &str,
    r#action: &str,
    r#modifier: &str,
    r#grid: i64,
    r#row: i64,
    r#col: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      &'a str,
      &'a str,
      &'a str,
      i64,
      i64,
      i64,
    );
    self
      .call(
        "nvim_input_mouse",
        Args(
          std::marker::PhantomData,
          r#button,
          r#action,
          r#modifier,
          r#grid,
          r#row,
          r#col,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_replace_termcodes(
    &self,
    r#str: &str,
    r#from_part: bool,
    r#do_lt: bool,
    r#special: bool,
  ) -> Result<String, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, bool, bool, bool);
    self
      .call(
        "nvim_replace_termcodes",
        Args(
          std::marker::PhantomData,
          r#str,
          r#from_part,
          r#do_lt,
          r#special,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_eval(&self, r#expr: &str) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_eval", Args(std::marker::PhantomData, r#expr))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_exec_lua(
    &self,
    r#code: &str,
    r#args: std::vec::Vec<Value>,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, std::vec::Vec<Value>);
    self
      .call(
        "nvim_exec_lua",
        Args(std::marker::PhantomData, r#code, r#args),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_notify(
    &self,
    r#msg: &str,
    r#log_level: i64,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      &'a str,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_notify",
        Args(std::marker::PhantomData, r#msg, r#log_level, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_call_function(
    &self,
    r#fn: &str,
    r#args: std::vec::Vec<Value>,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, std::vec::Vec<Value>);
    self
      .call(
        "nvim_call_function",
        Args(std::marker::PhantomData, r#fn, r#args),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_call_dict_function(
    &self,
    r#dict: Value,
    r#fn: &str,
    r#args: std::vec::Vec<Value>,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      Value,
      &'a str,
      std::vec::Vec<Value>,
    );
    self
      .call(
        "nvim_call_dict_function",
        Args(std::marker::PhantomData, r#dict, r#fn, r#args),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_strwidth(
    &self,
    r#text: &str,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_strwidth", Args(std::marker::PhantomData, r#text))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_list_runtime_paths(
    &self,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_list_runtime_paths", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_get_runtime_file(
    &self,
    r#name: &str,
    r#all: bool,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, bool);
    self
      .call(
        "nvim_get_runtime_file",
        Args(std::marker::PhantomData, r#name, r#all),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_current_dir(
    &self,
    r#dir: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call(
        "nvim_set_current_dir",
        Args(std::marker::PhantomData, r#dir),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_current_line(&self) -> Result<String, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_get_current_line", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_current_line(
    &self,
    r#line: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call(
        "nvim_set_current_line",
        Args(std::marker::PhantomData, r#line),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_del_current_line(&self) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_del_current_line", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_var(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_get_var", Args(std::marker::PhantomData, r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, Value);
    self
      .call(
        "nvim_set_var",
        Args(std::marker::PhantomData, r#name, r#value),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_del_var(&self, r#name: &str) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_del_var", Args(std::marker::PhantomData, r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_vvar(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_get_vvar", Args(std::marker::PhantomData, r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_set_vvar(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, Value);
    self
      .call(
        "nvim_set_vvar",
        Args(std::marker::PhantomData, r#name, r#value),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_option(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_get_option", Args(std::marker::PhantomData, r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_get_all_options_info(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_get_all_options_info", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_get_option_info(
    &self,
    r#name: &str,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call(
        "nvim_get_option_info",
        Args(std::marker::PhantomData, r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, Value);
    self
      .call(
        "nvim_set_option",
        Args(std::marker::PhantomData, r#name, r#value),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_echo(
    &self,
    r#chunks: std::vec::Vec<Value>,
    r#history: bool,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      std::vec::Vec<Value>,
      bool,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_echo",
        Args(std::marker::PhantomData, r#chunks, r#history, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_out_write(
    &self,
    r#str: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_out_write", Args(std::marker::PhantomData, r#str))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_err_write(
    &self,
    r#str: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_err_write", Args(std::marker::PhantomData, r#str))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_err_writeln(
    &self,
    r#str: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_err_writeln", Args(std::marker::PhantomData, r#str))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_current_buf(
    &self,
    r#buffer: Buffer<W>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call(
        "nvim_set_current_buf",
        Args(std::marker::PhantomData, r#buffer.get_value()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_current_win(
    &self,
    r#window: Window<W>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call(
        "nvim_set_current_win",
        Args(std::marker::PhantomData, r#window.get_value()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_set_current_tabpage(
    &self,
    r#tabpage: Tabpage<W>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call(
        "nvim_set_current_tabpage",
        Args(std::marker::PhantomData, r#tabpage.get_value()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_create_namespace(
    &self,
    r#name: &str,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call(
        "nvim_create_namespace",
        Args(std::marker::PhantomData, r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_get_namespaces(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_get_namespaces", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_paste(
    &self,
    r#data: &str,
    r#crlf: bool,
    r#phase: i64,
  ) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, bool, i64);
    self
      .call(
        "nvim_paste",
        Args(std::marker::PhantomData, r#data, r#crlf, r#phase),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_put(
    &self,
    r#lines: &[&str],
    r#type: &str,
    r#after: bool,
    r#follow: bool,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      &'a [&'a str],
      &'a str,
      bool,
      bool,
    );
    self
      .call(
        "nvim_put",
        Args(std::marker::PhantomData, r#lines, r#type, r#after, r#follow),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_subscribe(
    &self,
    r#event: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_subscribe", Args(std::marker::PhantomData, r#event))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_unsubscribe(
    &self,
    r#event: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_unsubscribe", Args(std::marker::PhantomData, r#event))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_color_by_name(
    &self,
    r#name: &str,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call(
        "nvim_get_color_by_name",
        Args(std::marker::PhantomData, r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_color_map(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_get_color_map", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_get_context(
    &self,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, std::vec::Vec<(Value, Value)>);
    self
      .call("nvim_get_context", Args(std::marker::PhantomData, r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_load_context(
    &self,
    r#dict: std::vec::Vec<(Value, Value)>,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, std::vec::Vec<(Value, Value)>);
    self
      .call("nvim_load_context", Args(std::marker::PhantomData, r#dict))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 2"]
  pub async fn nvim_get_mode(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_get_mode", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn nvim_get_keymap(
    &self,
    r#mode: &str,
  ) -> Result<std::vec::Vec<std::vec::Vec<(Value, Value)>>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str);
    self
      .call("nvim_get_keymap", Args(std::marker::PhantomData, r#mode))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_set_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
    r#rhs: &str,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      &'a str,
      &'a str,
      &'a str,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_set_keymap",
        Args(std::marker::PhantomData, r#mode, r#lhs, r#rhs, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_del_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, &'a str);
    self
      .call(
        "nvim_del_keymap",
        Args(std::marker::PhantomData, r#mode, r#lhs),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_get_commands(
    &self,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, std::vec::Vec<(Value, Value)>);
    self
      .call("nvim_get_commands", Args(std::marker::PhantomData, r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_get_api_info(
    &self,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_get_api_info", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_set_client_info(
    &self,
    r#name: &str,
    r#version: std::vec::Vec<(Value, Value)>,
    r#type: &str,
    r#methods: std::vec::Vec<(Value, Value)>,
    r#attributes: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      &'a str,
      std::vec::Vec<(Value, Value)>,
      &'a str,
      std::vec::Vec<(Value, Value)>,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_set_client_info",
        Args(
          std::marker::PhantomData,
          r#name,
          r#version,
          r#type,
          r#methods,
          r#attributes,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_get_chan_info(
    &self,
    r#chan: i64,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call("nvim_get_chan_info", Args(std::marker::PhantomData, r#chan))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_list_chans(
    &self,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_list_chans", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_call_atomic(
    &self,
    r#calls: std::vec::Vec<Value>,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, std::vec::Vec<Value>);
    self
      .call("nvim_call_atomic", Args(std::marker::PhantomData, r#calls))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_parse_expression(
    &self,
    r#expr: &str,
    r#flags: &str,
    r#highlight: bool,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, &'a str, &'a str, bool);
    self
      .call(
        "nvim_parse_expression",
        Args(std::marker::PhantomData, r#expr, r#flags, r#highlight),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_list_uis(
    &self,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>);
    self
      .call("nvim_list_uis", Args(std::marker::PhantomData))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_get_proc_children(
    &self,
    r#pid: i64,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call(
        "nvim_get_proc_children",
        Args(std::marker::PhantomData, r#pid),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_get_proc(
    &self,
    r#pid: i64,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .call("nvim_get_proc", Args(std::marker::PhantomData, r#pid))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_select_popupmenu_item(
    &self,
    r#item: i64,
    r#insert: bool,
    r#finish: bool,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      bool,
      bool,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_select_popupmenu_item",
        Args(std::marker::PhantomData, r#item, r#insert, r#finish, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_set_decoration_provider(
    &self,
    r#ns_id: i64,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .call(
        "nvim_set_decoration_provider",
        Args(std::marker::PhantomData, r#ns_id, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
pub struct Window<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: i64,
  pub(crate) neovim: Neovim<W>,
}
impl<W: AsyncWrite + Send + Unpin + 'static> Window<W> {
  pub fn new(code_data: i64, neovim: Neovim<W>) -> Window<W> {
    Window { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> i64 {
    self.code_data
  }
  #[doc = "Since 5"]
  pub async fn nvim_win_set_buf(
    &self,
    r#buffer: Buffer<W>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64);
    self
      .neovim
      .call(
        "nvim_win_set_buf",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#buffer.get_value(),
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_cursor(
    &self,
  ) -> Result<(i64, i64), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_get_cursor",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_cursor(
    &self,
    r#pos: (i64, i64),
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, (i64, i64));
    self
      .neovim
      .call(
        "nvim_win_set_cursor",
        Args(std::marker::PhantomData, self.code_data.clone(), r#pos),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_height(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_get_height",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_height(
    &self,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64);
    self
      .neovim
      .call(
        "nvim_win_set_height",
        Args(std::marker::PhantomData, self.code_data.clone(), r#height),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_width(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_get_width",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_width(
    &self,
    r#width: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64);
    self
      .neovim
      .call(
        "nvim_win_set_width",
        Args(std::marker::PhantomData, self.code_data.clone(), r#width),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_var(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_win_get_var",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_win_set_var",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#name,
          r#value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_del_var(
    &self,
    r#name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_win_del_var",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_option(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_win_get_option",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_win_set_option",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#name,
          r#value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_position(
    &self,
  ) -> Result<(i64, i64), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_get_position",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_number(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_get_number",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_is_valid(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_is_valid",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_win_set_config(
    &self,
    r#config: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_win_set_config",
        Args(std::marker::PhantomData, self.code_data.clone(), r#config),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_win_get_config(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_win_get_config",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_win_close(
    &self,
    r#force: bool,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, bool);
    self
      .neovim
      .call(
        "nvim_win_close",
        Args(std::marker::PhantomData, self.code_data.clone(), r#force),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
pub struct Buffer<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: i64,
  pub(crate) neovim: Neovim<W>,
}
impl<W: AsyncWrite + Send + Unpin + 'static> Buffer<W> {
  pub fn new(code_data: i64, neovim: Neovim<W>) -> Buffer<W> {
    Buffer { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> i64 {
    self.code_data
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_line_count(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_buf_line_count",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_buf_attach(
    &self,
    r#send_buffer: bool,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      bool,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_attach",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#send_buffer,
          r#opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_buf_detach(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_buf_detach",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_lines(
    &self,
    r#start: i64,
    r#end: i64,
    r#strict_indexing: bool,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64, i64, bool);
    self
      .neovim
      .call(
        "nvim_buf_get_lines",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#start,
          r#end,
          r#strict_indexing,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_lines(
    &self,
    r#start: i64,
    r#end: i64,
    r#strict_indexing: bool,
    r#replacement: &[&str],
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      i64,
      i64,
      bool,
      &'a [&'a str],
    );
    self
      .neovim
      .call(
        "nvim_buf_set_lines",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#start,
          r#end,
          r#strict_indexing,
          r#replacement,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_set_text(
    &self,
    r#start_row: i64,
    r#start_col: i64,
    r#end_row: i64,
    r#end_col: i64,
    r#replacement: &[&str],
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      i64,
      i64,
      i64,
      i64,
      &'a [&'a str],
    );
    self
      .neovim
      .call(
        "nvim_buf_set_text",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#start_row,
          r#start_col,
          r#end_row,
          r#end_col,
          r#replacement,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_get_offset(
    &self,
    r#index: i64,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_get_offset",
        Args(std::marker::PhantomData, self.code_data.clone(), r#index),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_var(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_var",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 2"]
  pub async fn nvim_buf_get_changedtick(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_buf_get_changedtick",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn nvim_buf_get_keymap(
    &self,
    r#mode: &str,
  ) -> Result<std::vec::Vec<std::vec::Vec<(Value, Value)>>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_keymap",
        Args(std::marker::PhantomData, self.code_data.clone(), r#mode),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_buf_set_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
    r#rhs: &str,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      &'a str,
      &'a str,
      &'a str,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_set_keymap",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#mode,
          r#lhs,
          r#rhs,
          r#opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_buf_del_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_del_keymap",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#mode,
          r#lhs,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_buf_get_commands(
    &self,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_get_commands",
        Args(std::marker::PhantomData, self.code_data.clone(), r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_buf_set_var",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#name,
          r#value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_del_var(
    &self,
    r#name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_del_var",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_option(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_option",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_buf_set_option",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#name,
          r#value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_name(&self) -> Result<String, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_buf_get_name",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_name(
    &self,
    r#name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_set_name",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_is_loaded(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_buf_is_loaded",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_delete(
    &self,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_delete",
        Args(std::marker::PhantomData, self.code_data.clone(), r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_is_valid(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_buf_is_valid",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_mark(
    &self,
    r#name: &str,
  ) -> Result<(i64, i64), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_mark",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_get_extmark_by_id(
    &self,
    r#ns_id: i64,
    r#id: i64,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<i64>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      i64,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_get_extmark_by_id",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#ns_id,
          r#id,
          r#opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_get_extmarks(
    &self,
    r#ns_id: i64,
    r#start: Value,
    r#end: Value,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      i64,
      Value,
      Value,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_get_extmarks",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#ns_id,
          r#start,
          r#end,
          r#opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_set_extmark(
    &self,
    r#ns_id: i64,
    r#line: i64,
    r#col: i64,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      i64,
      i64,
      i64,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_set_extmark",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#ns_id,
          r#line,
          r#col,
          r#opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_del_extmark(
    &self,
    r#ns_id: i64,
    r#id: i64,
  ) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_del_extmark",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#ns_id,
          r#id,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_add_highlight(
    &self,
    r#src_id: i64,
    r#hl_group: &str,
    r#line: i64,
    r#col_start: i64,
    r#col_end: i64,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64, &'a str, i64, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_add_highlight",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#src_id,
          r#hl_group,
          r#line,
          r#col_start,
          r#col_end,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_clear_namespace(
    &self,
    r#ns_id: i64,
    r#line_start: i64,
    r#line_end: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, i64, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_clear_namespace",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#ns_id,
          r#line_start,
          r#line_end,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_set_virtual_text(
    &self,
    r#src_id: i64,
    r#line: i64,
    r#chunks: std::vec::Vec<Value>,
    r#opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      i64,
      i64,
      i64,
      std::vec::Vec<Value>,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_set_virtual_text",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#src_id,
          r#line,
          r#chunks,
          r#opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
pub struct Tabpage<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: i64,
  pub(crate) neovim: Neovim<W>,
}
impl<W: AsyncWrite + Send + Unpin + 'static> Tabpage<W> {
  pub fn new(code_data: i64, neovim: Neovim<W>) -> Tabpage<W> {
    Tabpage { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> i64 {
    self.code_data
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_get_var(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_tabpage_get_var",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_tabpage_set_var",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          r#name,
          r#value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_del_var(
    &self,
    r#name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64, &'a str);
    self
      .neovim
      .call(
        "nvim_tabpage_del_var",
        Args(std::marker::PhantomData, self.code_data.clone(), r#name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_get_number(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_tabpage_get_number",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_is_valid(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, i64);
    self
      .neovim
      .call(
        "nvim_tabpage_is_valid",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
