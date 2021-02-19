//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{error::CallError, rpc::unpack::TryUnpack, Neovim, Value};
use futures::io::AsyncWrite;
use serde::Serialize;
use std::marker::PhantomData;
impl<W: AsyncWrite + Send + Unpin + 'static> Neovim<W> {
  #[doc = "Since 1"]
  pub async fn tabpage_get_var(
    &self,
    r#tabpage: Tabpage<W>,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_tabpage_get_var", (r#tabpage.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn tabpage_set_var(
    &self,
    r#tabpage: Tabpage<W>,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_tabpage_set_var",
        (r#tabpage.get_value(), r#name, r#value),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn tabpage_del_var(
    &self,
    r#tabpage: Tabpage<W>,
    r#name: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_tabpage_del_var", (r#tabpage.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn tabpage_get_win(
    &self,
    r#tabpage: Tabpage<W>,
  ) -> Result<Window<W>, Box<CallError>> {
    self
      .call("nvim_tabpage_get_win", (r#tabpage.get_value(),))
      .await?
      .map(|val| Window::new(val.try_unpack().unwrap(), self.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn tabpage_get_number(
    &self,
    r#tabpage: Tabpage<W>,
  ) -> Result<i64, Box<CallError>> {
    self
      .call("nvim_tabpage_get_number", (r#tabpage.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn tabpage_is_valid(
    &self,
    r#tabpage: Tabpage<W>,
  ) -> Result<bool, Box<CallError>> {
    self
      .call("nvim_tabpage_is_valid", (r#tabpage.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn ui_attach(
    &self,
    r#width: i64,
    r#height: i64,
    r#options: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_attach", (r#width, r#height, r#options))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn ui_detach(&self) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_detach", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn ui_try_resize(
    &self,
    r#width: i64,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_try_resize", (r#width, r#height))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn ui_set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_set_option", (r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn ui_try_resize_grid(
    &self,
    r#grid: i64,
    r#width: i64,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_try_resize_grid", (r#grid, r#width, r#height))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn ui_pum_set_height(
    &self,
    r#height: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_pum_set_height", (r#height,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn ui_pum_set_bounds(
    &self,
    r#width: f64,
    r#height: f64,
    r#row: f64,
    r#col: f64,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_ui_pum_set_bounds", (r#width, r#height, r#row, r#col))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn exec(
    &self,
    r#src: &str,
    r#output: bool,
  ) -> Result<String, Box<CallError>> {
    self
      .call("nvim_exec", (r#src, r#output))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn command(&self, r#command: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_command", (r#command,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn get_hl_by_name(
    &self,
    r#name: &str,
    r#rgb: bool,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_hl_by_name", (r#name, r#rgb))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn get_hl_by_id(
    &self,
    r#hl_id: i64,
    r#rgb: bool,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_hl_by_id", (r#hl_id, r#rgb))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn get_hl_id_by_name(
    &self,
    r#name: &str,
  ) -> Result<i64, Box<CallError>> {
    self
      .call("nvim_get_hl_id_by_name", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn set_hl(
    &self,
    r#ns_id: i64,
    r#name: &str,
    r#val: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_hl", (r#ns_id, r#name, r#val))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn set_hl_ns(&self, r#ns_id: i64) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_hl_ns", (r#ns_id,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn feedkeys(
    &self,
    r#keys: &str,
    r#mode: &str,
    r#escape_csi: bool,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_feedkeys", (r#keys, r#mode, r#escape_csi))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn input(&self, r#keys: &str) -> Result<i64, Box<CallError>> {
    self
      .call("nvim_input", (r#keys,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn input_mouse(
    &self,
    r#button: &str,
    r#action: &str,
    r#modifier: &str,
    r#grid: i64,
    r#row: i64,
    r#col: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_input_mouse",
        (r#button, r#action, r#modifier, r#grid, r#row, r#col),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn replace_termcodes(
    &self,
    r#str: &str,
    r#from_part: bool,
    r#do_lt: bool,
    r#special: bool,
  ) -> Result<String, Box<CallError>> {
    self
      .call(
        "nvim_replace_termcodes",
        (r#str, r#from_part, r#do_lt, r#special),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn eval(&self, r#expr: &str) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_eval", (r#expr,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn exec_lua(
    &self,
    r#code: &str,
    r#args: &[Value],
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_exec_lua", (r#code, r#args))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn notify(
    &self,
    r#msg: &str,
    r#log_level: i64,
    r#opts: &[(Value, Value)],
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_notify", (r#msg, r#log_level, r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn call_function(
    &self,
    r#fn: &str,
    r#args: &[Value],
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_call_function", (r#fn, r#args))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn call_dict_function(
    &self,
    r#dict: Value,
    r#fn: &str,
    r#args: &[Value],
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_call_dict_function", (r#dict, r#fn, r#args))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn strwidth(&self, r#text: &str) -> Result<i64, Box<CallError>> {
    self
      .call("nvim_strwidth", (r#text,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn list_runtime_paths(
    &self,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    self
      .call("nvim_list_runtime_paths", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn get_runtime_file(
    &self,
    r#name: &str,
    r#all: bool,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    self
      .call("nvim_get_runtime_file", (r#name, r#all))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_current_dir(
    &self,
    r#dir: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_current_dir", (r#dir,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_current_line(&self) -> Result<String, Box<CallError>> {
    self
      .call("nvim_get_current_line", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_current_line(
    &self,
    r#line: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_current_line", (r#line,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn del_current_line(&self) -> Result<(), Box<CallError>> {
    self
      .call("nvim_del_current_line", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_var(&self, r#name: &str) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_get_var", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_var", (r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn del_var(&self, r#name: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_del_var", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_vvar(&self, r#name: &str) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_get_vvar", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn set_vvar(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_vvar", (r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_option(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_get_option", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn get_all_options_info(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_all_options_info", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn get_option_info(
    &self,
    r#name: &str,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_option_info", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_option", (r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn echo(
    &self,
    r#chunks: &[Value],
    r#history: bool,
    r#opts: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_echo", (r#chunks, r#history, r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn out_write(&self, r#str: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_out_write", (r#str,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn err_write(&self, r#str: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_err_write", (r#str,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn err_writeln(&self, r#str: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_err_writeln", (r#str,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_current_buf(&self) -> Result<Buffer<W>, Box<CallError>> {
    self
      .call("nvim_get_current_buf", {
        let empty: [Value; 0] = [];
        empty
      })
      .await?
      .map(|val| Buffer::new(val.try_unpack().unwrap(), self.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_current_buf(
    &self,
    r#buffer: Buffer<W>,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_current_buf", (r#buffer.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_current_win(&self) -> Result<Window<W>, Box<CallError>> {
    self
      .call("nvim_get_current_win", {
        let empty: [Value; 0] = [];
        empty
      })
      .await?
      .map(|val| Window::new(val.try_unpack().unwrap(), self.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_current_win(
    &self,
    r#window: Window<W>,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_current_win", (r#window.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn create_buf(
    &self,
    r#listed: bool,
    r#scratch: bool,
  ) -> Result<Buffer<W>, Box<CallError>> {
    self
      .call("nvim_create_buf", (r#listed, r#scratch))
      .await?
      .map(|val| Buffer::new(val.try_unpack().unwrap(), self.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn open_win(
    &self,
    r#buffer: Buffer<W>,
    r#enter: bool,
    r#config: &[(Value, Value)],
  ) -> Result<Window<W>, Box<CallError>> {
    self
      .call("nvim_open_win", (r#buffer.get_value(), r#enter, r#config))
      .await?
      .map(|val| Window::new(val.try_unpack().unwrap(), self.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_current_tabpage(
    &self,
  ) -> Result<Tabpage<W>, Box<CallError>> {
    self
      .call("nvim_get_current_tabpage", {
        let empty: [Value; 0] = [];
        empty
      })
      .await?
      .map(|val| Tabpage::new(val.try_unpack().unwrap(), self.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_current_tabpage(
    &self,
    r#tabpage: Tabpage<W>,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_current_tabpage", (r#tabpage.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn create_namespace(
    &self,
    r#name: &str,
  ) -> Result<i64, Box<CallError>> {
    self
      .call("nvim_create_namespace", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn get_namespaces(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_namespaces", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn paste(
    &self,
    r#data: &str,
    r#crlf: bool,
    r#phase: i64,
  ) -> Result<bool, Box<CallError>> {
    self
      .call("nvim_paste", (r#data, r#crlf, r#phase))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn put(
    &self,
    r#lines: &[&str],
    r#type: &str,
    r#after: bool,
    r#follow: bool,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_put", (r#lines, r#type, r#after, r#follow))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn subscribe(&self, r#event: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_subscribe", (r#event,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn unsubscribe(&self, r#event: &str) -> Result<(), Box<CallError>> {
    self
      .call("nvim_unsubscribe", (r#event,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_color_by_name(
    &self,
    r#name: &str,
  ) -> Result<i64, Box<CallError>> {
    self
      .call("nvim_get_color_by_name", (r#name,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_color_map(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_color_map", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn get_context(
    &self,
    r#opts: &[(Value, Value)],
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_context", (r#opts,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn load_context(
    &self,
    r#dict: &[(Value, Value)],
  ) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_load_context", (r#dict,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 2"]
  pub async fn get_mode(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_mode", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn get_keymap(
    &self,
    r#mode: &str,
  ) -> Result<std::vec::Vec<std::vec::Vec<(Value, Value)>>, Box<CallError>> {
    self
      .call("nvim_get_keymap", (r#mode,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn set_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
    r#rhs: &str,
    r#opts: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_keymap", (r#mode, r#lhs, r#rhs, r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn del_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_del_keymap", (r#mode, r#lhs))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn get_commands(
    &self,
    r#opts: &[(Value, Value)],
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_commands", (r#opts,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_api_info(
    &self,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .call("nvim_get_api_info", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn set_client_info(
    &self,
    r#name: &str,
    r#version: &[(Value, Value)],
    r#type: &str,
    r#methods: &[(Value, Value)],
    r#attributes: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_set_client_info",
        (r#name, r#version, r#type, r#methods, r#attributes),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn get_chan_info(
    &self,
    r#chan: i64,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_get_chan_info", (r#chan,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn list_chans(
    &self,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .call("nvim_list_chans", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn call_atomic(
    &self,
    r#calls: &[Value],
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .call("nvim_call_atomic", (r#calls,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn parse_expression(
    &self,
    r#expr: &str,
    r#flags: &str,
    r#highlight: bool,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .call("nvim_parse_expression", (r#expr, r#flags, r#highlight))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn list_uis(&self) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .call("nvim_list_uis", {
        let empty: [Value; 0] = [];
        empty
      })
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn get_proc_children(
    &self,
    r#pid: i64,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .call("nvim_get_proc_children", (r#pid,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn get_proc(&self, r#pid: i64) -> Result<Value, Box<CallError>> {
    self
      .call("nvim_get_proc", (r#pid,))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn select_popupmenu_item(
    &self,
    r#item: i64,
    r#insert: bool,
    r#finish: bool,
    r#opts: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_select_popupmenu_item",
        (r#item, r#insert, r#finish, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn set_decoration_provider(
    &self,
    r#ns_id: i64,
    r#opts: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .call("nvim_set_decoration_provider", (r#ns_id, r#opts))
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
  #[doc = "Since 1"]
  pub async fn get_buf(&self) -> Result<Buffer<W>, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_buf", (self.get_value(),))
      .await?
      .map(|val| Buffer::new(val.try_unpack().unwrap(), self.neovim.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn set_buf(
    &self,
    r#buffer: Buffer<W>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_buf", (self.get_value(), r#buffer.get_value()))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_cursor(&self) -> Result<(i64, i64), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_cursor", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_cursor(
    &self,
    r#pos: (i64, i64),
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_cursor", (self.get_value(), r#pos))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_height(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_height", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_height(&self, r#height: i64) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_height", (self.get_value(), r#height))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_width(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_width", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_width(&self, r#width: i64) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_width", (self.get_value(), r#width))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_var(&self, r#name: &str) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_var", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_var", (self.get_value(), r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn del_var(&self, r#name: &str) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_del_var", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_option(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_option", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_option", (self.get_value(), r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_position(&self) -> Result<(i64, i64), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_position", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_tabpage(&self) -> Result<Tabpage<W>, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_tabpage", (self.get_value(),))
      .await?
      .map(|val| Tabpage::new(val.try_unpack().unwrap(), self.neovim.clone()))
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_number(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_number", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn is_valid(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_is_valid", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn set_config(
    &self,
    r#config: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_set_config", (self.get_value(), r#config))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn get_config(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_config", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn close(&self, r#force: bool) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_close", (self.get_value(), r#force))
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
  pub async fn line_count(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_line_count", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn attach(
    &self,
    r#send_buffer: bool,
    r#opts: &[(Value, Value)],
  ) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_attach", (self.get_value(), r#send_buffer, r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn detach(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_detach", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_lines(
    &self,
    r#start: i64,
    r#end: i64,
    r#strict_indexing: bool,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_lines",
        (self.get_value(), r#start, r#end, r#strict_indexing),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_lines(
    &self,
    r#start: i64,
    r#end: i64,
    r#strict_indexing: bool,
    r#replacement: &[&str],
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_lines",
        (
          self.get_value(),
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
  pub async fn set_text(
    &self,
    r#start_row: i64,
    r#start_col: i64,
    r#end_row: i64,
    r#end_col: i64,
    r#replacement: &[&str],
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_text",
        (
          self.get_value(),
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
  pub async fn get_offset(&self, r#index: i64) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_offset", (self.get_value(), r#index))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_var(&self, r#name: &str) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_var", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 2"]
  pub async fn get_changedtick(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_changedtick", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 3"]
  pub async fn get_keymap(
    &self,
    r#mode: &str,
  ) -> Result<std::vec::Vec<std::vec::Vec<(Value, Value)>>, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_keymap", (self.get_value(), r#mode))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn set_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
    r#rhs: &str,
    r#opts: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_keymap",
        (self.get_value(), r#mode, r#lhs, r#rhs, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn del_keymap(
    &self,
    r#mode: &str,
    r#lhs: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_del_keymap", (self.get_value(), r#mode, r#lhs))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn get_commands(
    &self,
    r#opts: &[(Value, Value)],
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_commands", (self.get_value(), r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_var(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_set_var", (self.get_value(), r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn del_var(&self, r#name: &str) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_del_var", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_option(
    &self,
    r#name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_option", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_option(
    &self,
    r#name: &str,
    r#value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_set_option", (self.get_value(), r#name, r#value))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_name(&self) -> Result<String, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_name", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn set_name(&self, r#name: &str) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_set_name", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn is_loaded(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_is_loaded", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn delete(
    &self,
    r#opts: &[(Value, Value)],
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_delete", (self.get_value(), r#opts))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn is_valid(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_is_valid", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn get_mark(
    &self,
    r#name: &str,
  ) -> Result<(i64, i64), Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_mark", (self.get_value(), r#name))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn get_extmark_by_id(
    &self,
    r#ns_id: i64,
    r#id: i64,
    r#opts: &[(Value, Value)],
  ) -> Result<std::vec::Vec<i64>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_extmark_by_id",
        (self.get_value(), r#ns_id, r#id, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn get_extmarks(
    &self,
    r#ns_id: i64,
    r#start: Value,
    r#end: Value,
    r#opts: &[(Value, Value)],
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_extmarks",
        (self.get_value(), r#ns_id, r#start, r#end, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn set_extmark(
    &self,
    r#ns_id: i64,
    r#line: i64,
    r#col: i64,
    r#opts: &[(Value, Value)],
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_extmark",
        (self.get_value(), r#ns_id, r#line, r#col, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn del_extmark(
    &self,
    r#ns_id: i64,
    r#id: i64,
  ) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_del_extmark", (self.get_value(), r#ns_id, r#id))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn add_highlight(
    &self,
    r#src_id: i64,
    r#hl_group: &str,
    r#line: i64,
    r#col_start: i64,
    r#col_end: i64,
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_add_highlight",
        (
          self.get_value(),
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
  pub async fn clear_namespace(
    &self,
    r#ns_id: i64,
    r#line_start: i64,
    r#line_end: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_clear_namespace",
        (self.get_value(), r#ns_id, r#line_start, r#line_end),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn set_virtual_text(
    &self,
    r#src_id: i64,
    r#line: i64,
    r#chunks: &[Value],
    r#opts: &[(Value, Value)],
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_virtual_text",
        (self.get_value(), r#src_id, r#line, r#chunks, r#opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn call(&self) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_call", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Deprecated since 2"]
  #[doc = "Since 1"]
  pub async fn get_number(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_number", (self.get_value(),))
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Deprecated since 7"]
  #[doc = "Since 1"]
  pub async fn clear_highlight(
    &self,
    r#ns_id: i64,
    r#line_start: i64,
    r#line_end: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_clear_highlight",
        (self.get_value(), r#ns_id, r#line_start, r#line_end),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
