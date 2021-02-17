//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{error::CallError, rpc::unpack::TryUnpack, Neovim, Value};
use futures::io::AsyncWrite;
use serde::Serialize;
use std::marker::PhantomData;
pub struct Tabpage<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: u32,
  pub(crate) neovim: Neovim<W>,
}
impl<W: AsyncWrite + Send + Unpin + 'static> Tabpage<W> {
  pub fn new(code_data: u32, neovim: Neovim<W>) -> Tabpage<W> {
    Tabpage { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> u32 {
    self.code_data
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_get_var(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_tabpage_get_var",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_tabpage_set_var",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          name,
          value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_del_var(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_tabpage_del_var",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_tabpage_get_number(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
pub struct Buffer<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: u32,
  pub(crate) neovim: Neovim<W>,
}
impl<W: AsyncWrite + Send + Unpin + 'static> Buffer<W> {
  pub fn new(code_data: u32, neovim: Neovim<W>) -> Buffer<W> {
    Buffer { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> u32 {
    self.code_data
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_line_count(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    send_buffer: bool,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          send_buffer,
          opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_buf_detach(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    start: i64,
    end: i64,
    strict_indexing: bool,
  ) -> Result<std::vec::Vec<String>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64, i64, bool);
    self
      .neovim
      .call(
        "nvim_buf_get_lines",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          start,
          end,
          strict_indexing,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_lines(
    &self,
    start: i64,
    end: i64,
    strict_indexing: bool,
    replacement: &[&str],
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          start,
          end,
          strict_indexing,
          replacement,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_set_text(
    &self,
    start_row: i64,
    start_col: i64,
    end_row: i64,
    end_col: i64,
    replacement: &[&str],
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          start_row,
          start_col,
          end_row,
          end_col,
          replacement,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_get_offset(
    &self,
    index: i64,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64);
    self
      .neovim
      .call(
        "nvim_buf_get_offset",
        Args(std::marker::PhantomData, self.code_data.clone(), index),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_var(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_var",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 2"]
  pub async fn nvim_buf_get_changedtick(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    mode: &str,
  ) -> Result<std::vec::Vec<std::vec::Vec<(Value, Value)>>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_keymap",
        Args(std::marker::PhantomData, self.code_data.clone(), mode),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_buf_set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          mode,
          lhs,
          rhs,
          opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 6"]
  pub async fn nvim_buf_del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_del_keymap",
        Args(std::marker::PhantomData, self.code_data.clone(), mode, lhs),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 4"]
  pub async fn nvim_buf_get_commands(
    &self,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<(Value, Value)>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_get_commands",
        Args(std::marker::PhantomData, self.code_data.clone(), opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_buf_set_var",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          name,
          value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_del_var(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_del_var",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_option(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_option",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_buf_set_option",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          name,
          value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_name(&self) -> Result<String, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_set_name",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_is_loaded(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_buf_delete",
        Args(std::marker::PhantomData, self.code_data.clone(), opts),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_is_valid(&self) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    name: &str,
  ) -> Result<(i64, i64), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_buf_get_mark",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_get_extmark_by_id(
    &self,
    ns_id: i64,
    id: i64,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<i64>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          ns_id,
          id,
          opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_get_extmarks(
    &self,
    ns_id: i64,
    start: Value,
    end: Value,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<std::vec::Vec<Value>, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          ns_id,
          start,
          end,
          opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_set_extmark(
    &self,
    ns_id: i64,
    line: i64,
    col: i64,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          ns_id,
          line,
          col,
          opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 7"]
  pub async fn nvim_buf_del_extmark(
    &self,
    ns_id: i64,
    id: i64,
  ) -> Result<bool, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_del_extmark",
        Args(std::marker::PhantomData, self.code_data.clone(), ns_id, id),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_buf_add_highlight(
    &self,
    src_id: i64,
    hl_group: &str,
    line: i64,
    col_start: i64,
    col_end: i64,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64, &'a str, i64, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_add_highlight",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          src_id,
          hl_group,
          line,
          col_start,
          col_end,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_clear_namespace(
    &self,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_clear_namespace",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          ns_id,
          line_start,
          line_end,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 5"]
  pub async fn nvim_buf_set_virtual_text(
    &self,
    src_id: i64,
    line: i64,
    chunks: std::vec::Vec<Value>,
    opts: std::vec::Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
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
          src_id,
          line,
          chunks,
          opts,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Deprecated since 2"]
  #[doc = "Since 1"]
  pub async fn nvim_buf_get_number(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
    self
      .neovim
      .call(
        "nvim_buf_get_number",
        Args(std::marker::PhantomData, self.code_data.clone()),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Deprecated since 7"]
  #[doc = "Since 1"]
  pub async fn nvim_buf_clear_highlight(
    &self,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64, i64, i64);
    self
      .neovim
      .call(
        "nvim_buf_clear_highlight",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          ns_id,
          line_start,
          line_end,
        ),
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
  pub(crate) code_data: u32,
  pub(crate) neovim: Neovim<W>,
}
impl<W: AsyncWrite + Send + Unpin + 'static> Window<W> {
  pub fn new(code_data: u32, neovim: Neovim<W>) -> Window<W> {
    Window { code_data, neovim }
  }
  #[doc = r" Internal value, that represent type"]
  pub fn get_value(&self) -> u32 {
    self.code_data
  }
  #[doc = "Since 5"]
  pub async fn nvim_win_set_buf(
    &self,
    buffer: Buffer<W>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, u32);
    self
      .neovim
      .call(
        "nvim_win_set_buf",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          buffer.get_value(),
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
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    pos: (i64, i64),
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, (i64, i64));
    self
      .neovim
      .call(
        "nvim_win_set_cursor",
        Args(std::marker::PhantomData, self.code_data.clone(), pos),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_height(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    height: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64);
    self
      .neovim
      .call(
        "nvim_win_set_height",
        Args(std::marker::PhantomData, self.code_data.clone(), height),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_width(&self) -> Result<i64, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    width: i64,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, i64);
    self
      .neovim
      .call(
        "nvim_win_set_width",
        Args(std::marker::PhantomData, self.code_data.clone(), width),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_var(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_win_get_var",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_win_set_var",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          name,
          value,
        ),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_del_var(
    &self,
    name: &str,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_win_del_var",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_get_option(
    &self,
    name: &str,
  ) -> Result<Value, Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str);
    self
      .neovim
      .call(
        "nvim_win_get_option",
        Args(std::marker::PhantomData, self.code_data.clone(), name),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
  #[doc = "Since 1"]
  pub async fn nvim_win_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, &'a str, Value);
    self
      .neovim
      .call(
        "nvim_win_set_option",
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          name,
          value,
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
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    config: std::vec::Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(
      PhantomData<&'a ()>,
      u32,
      std::vec::Vec<(Value, Value)>,
    );
    self
      .neovim
      .call(
        "nvim_win_set_config",
        Args(std::marker::PhantomData, self.code_data.clone(), config),
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
    pub struct Args<'a>(PhantomData<&'a ()>, u32);
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
    force: bool,
  ) -> Result<(), Box<CallError>> {
    #[derive(Debug, Serialize)]
    pub struct Args<'a>(PhantomData<&'a ()>, u32, bool);
    self
      .neovim
      .call(
        "nvim_win_close",
        Args(std::marker::PhantomData, self.code_data.clone(), force),
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
  }
}
