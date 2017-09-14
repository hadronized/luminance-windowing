/// Dimension of the window to create.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowDim {
  /// Windowed mode.
  Windowed(u32, u32),
  /// Fullscreen mode (adapt to your screen).
  Fullscreen,
  /// Fullscreen mode with restricted viewport dimension.
  FullscreenRestricted(u32, u32)
}

/// Different window options.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowOpt {
  hide_cursor: bool
}

impl Default for WindowOpt {
  fn default() -> Self {
    WindowOpt {
      hide_cursor: false
    }
  }
}

impl WindowOpt {
  /// Hide or unhide the cursor. Default to `false`.
  #[inline]
  pub fn hide_cursor(self, hide: bool) -> Self {
    WindowOpt { hide_cursor: hide, ..self }
  }

  #[inline]
  pub fn is_cursor_hidden(&self) -> bool {
    self.hide_cursor
  }
}

pub trait Device: Sized {
  /// Type of events.
  type Event;

  /// Iterator over events.
  type EventIter: Iterator<Item = Self::Event>;

  /// Type of device errors.
  type Error;

  /// Create a device and bootstrap a luminance environment that lives as long as the device
  /// lives.
  fn new(dim: WindowDim, title: &str, win_opt: WindowOpt) -> Result<Self, Self::Error>;

  /// Size of the framebuffer attached to the window.
  fn size(&self) -> [u32; 2];

  /// Width of the framebuffer attached to the window.
  fn width(&self) -> u32 {
    self.size()[0]
  }

  /// Height of the framebuffer attached to the window.
  fn height(&self) -> u32 {
    self.size()[1]
  }

  // FIXME: as soon as either ATC (https://github.com/rust-lang/rust/issues/44265) or conservative
  // impl trait in methods (https://github.com/rust-lang/rust/issues/42183) are a real thing,
  // changethe interface
  /// Retrieve an iterator over any pulled events.
  fn events<'a>(&'a mut self) -> Box<Iterator<Item = Self::Event> + 'a>;

  /// Perform a draw. You should recall that function each time you want to draw a single frame to
  /// the screen.
  fn draw<F>(&mut self, f: F) where F: FnOnce();
}
