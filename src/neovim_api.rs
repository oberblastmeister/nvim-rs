//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::Window;
use crate::{error::CallError, rpc::unpack::TryUnpack, Neovim, Value};
use futures::io::AsyncWrite;
impl<W: AsyncWrite + Send + Unpin + 'static> Window<W> {
  pub fn new(code_data: Value, neovim: Neovim<W>) -> Window<W> {
    Window { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }
  pub async fn nvim_win_get_buf(&self) -> Result<Window<W>, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_buf", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_buf(
    &self,
    buffer: Window<W>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_buf",
        [self.code_data.clone(), Value::from(buffer)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_cursor(
    &self,
  ) -> Result<(i64, i64), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_cursor", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_cursor(
    &self,
    pos: (i64, i64),
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_cursor",
        [self.code_data.clone(), Value::from(pos)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_height(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_height", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_height(
    &self,
    height: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_height",
        [self.code_data.clone(), Value::from(height)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_width(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_width", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_width(
    &self,
    width: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_width",
        [self.code_data.clone(), Value::from(width)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_var(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_get_var",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_var",
        [
          self.code_data.clone(),
          Value::from(name),
          Value::from(value),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_del_var(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_del_var",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_option(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_get_option",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_option",
        [
          self.code_data.clone(),
          Value::from(name),
          Value::from(value),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_position(
    &self,
  ) -> Result<(i64, i64), Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_position", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_tabpage(
    &self,
  ) -> Result<Tabpage<W>, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_tabpage", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_number(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_number", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_is_valid(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_is_valid", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_set_config(
    &self,
    config: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_set_config",
        [self.code_data.clone(), Value::from(config)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_get_config(
    &self,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .neovim
      .call("nvim_win_get_config", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_win_close(
    &self,
    force: bool,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_win_close",
        [self.code_data.clone(), Value::from(force)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
use crate::Tabpage;
impl<W: AsyncWrite + Send + Unpin + 'static> Tabpage<W> {
  pub fn new(code_data: Value, neovim: Neovim<W>) -> Tabpage<W> {
    Tabpage { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }
  pub async fn nvim_tabpage_list_wins(
    &self,
  ) -> Result<std::vec::Vec<Window<W>>, Box<CallError>> {
    self
      .neovim
      .call("nvim_tabpage_list_wins", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_tabpage_get_var(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_tabpage_get_var",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_tabpage_set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_tabpage_set_var",
        [
          self.code_data.clone(),
          Value::from(name),
          Value::from(value),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_tabpage_del_var(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_tabpage_del_var",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_tabpage_get_win(
    &self,
  ) -> Result<Window<W>, Box<CallError>> {
    self
      .neovim
      .call("nvim_tabpage_get_win", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_tabpage_get_number(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_tabpage_get_number", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_tabpage_is_valid(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_tabpage_is_valid", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
use crate::Buffer;
impl<W: AsyncWrite + Send + Unpin + 'static> Buffer<W> {
  pub fn new(code_data: Value, neovim: Neovim<W>) -> Buffer<W> {
    Buffer { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }
  pub async fn nvim_buf_line_count(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_line_count", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_attach(
    &self,
    send_buffer: bool,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_attach",
        [
          self.code_data.clone(),
          Value::from(send_buffer),
          Value::from(opts),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_detach(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_detach", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_lines(
    &self,
    start: i64,
    end: i64,
    strict_indexing: bool,
  ) -> Result<std::vec::Vec<&str>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_lines",
        [
          self.code_data.clone(),
          Value::from(start),
          Value::from(end),
          Value::from(strict_indexing),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_lines(
    &self,
    start: i64,
    end: i64,
    strict_indexing: bool,
    replacement: std::vec::Vec<&str>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_lines",
        [
          self.code_data.clone(),
          Value::from(start),
          Value::from(end),
          Value::from(strict_indexing),
          Value::from(replacement),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_text(
    &self,
    start_row: i64,
    start_col: i64,
    end_row: i64,
    end_col: i64,
    replacement: std::vec::Vec<&str>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_text",
        [
          self.code_data.clone(),
          Value::from(start_row),
          Value::from(start_col),
          Value::from(end_row),
          Value::from(end_col),
          Value::from(replacement),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_offset(
    &self,
    index: i64,
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_offset",
        [self.code_data.clone(), Value::from(index)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_var(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_var",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_changedtick(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_changedtick", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_keymap(
    &self,
    mode: &str,
  ) -> Result<std::vec::Vec<std::vec::Vec<(Value, Value)>>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_keymap",
        [self.code_data.clone(), Value::from(mode)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_keymap",
        [
          self.code_data.clone(),
          Value::from(mode),
          Value::from(lhs),
          Value::from(rhs),
          Value::from(opts),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_del_keymap",
        [self.code_data.clone(), Value::from(mode), Value::from(lhs)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_commands(
    &self,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_commands",
        [self.code_data.clone(), Value::from(opts)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_var",
        [
          self.code_data.clone(),
          Value::from(name),
          Value::from(value),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_del_var(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_del_var",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_option(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_option",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_option",
        [
          self.code_data.clone(),
          Value::from(name),
          Value::from(value),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_name(&self) -> Result<&str, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_name", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_name(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_name",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_is_loaded(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_is_loaded", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_delete(
    &self,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_delete",
        [self.code_data.clone(), Value::from(opts)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_is_valid(&self) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_is_valid", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_mark(
    &self,
    name: &str,
  ) -> Result<(i64, i64), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_mark",
        [self.code_data.clone(), Value::from(name)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_extmark_by_id(
    &self,
    ns_id: i64,
    id: i64,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<i64>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_extmark_by_id",
        [
          self.code_data.clone(),
          Value::from(ns_id),
          Value::from(id),
          Value::from(opts),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_extmarks(
    &self,
    ns_id: i64,
    start: Value,
    end: Value,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_get_extmarks",
        [
          self.code_data.clone(),
          Value::from(ns_id),
          Value::from(start),
          Value::from(end),
          Value::from(opts),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_extmark(
    &self,
    ns_id: i64,
    line: i64,
    col: i64,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_extmark",
        [
          self.code_data.clone(),
          Value::from(ns_id),
          Value::from(line),
          Value::from(col),
          Value::from(opts),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_del_extmark(
    &self,
    ns_id: i64,
    id: i64,
  ) -> Result<bool, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_del_extmark",
        [self.code_data.clone(), Value::from(ns_id), Value::from(id)],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_add_highlight(
    &self,
    src_id: i64,
    hl_group: &str,
    line: i64,
    col_start: i64,
    col_end: i64,
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_add_highlight",
        [
          self.code_data.clone(),
          Value::from(src_id),
          Value::from(hl_group),
          Value::from(line),
          Value::from(col_start),
          Value::from(col_end),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_clear_namespace(
    &self,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_clear_namespace",
        [
          self.code_data.clone(),
          Value::from(ns_id),
          Value::from(line_start),
          Value::from(line_end),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_set_virtual_text(
    &self,
    src_id: i64,
    line: i64,
    chunks: std::vec::Vec<Value>,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_set_virtual_text",
        [
          self.code_data.clone(),
          Value::from(src_id),
          Value::from(line),
          Value::from(chunks),
          Value::from(opts),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_call(&self) -> Result<Value, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_call", [self.code_data.clone(), Value::from(fun)])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_get_number(&self) -> Result<i64, Box<CallError>> {
    self
      .neovim
      .call("nvim_buf_get_number", [self.code_data.clone()])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  pub async fn nvim_buf_clear_highlight(
    &self,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), Box<CallError>> {
    self
      .neovim
      .call(
        "nvim_buf_clear_highlight",
        [
          self.code_data.clone(),
          Value::from(ns_id),
          Value::from(line_start),
          Value::from(line_end),
        ],
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
