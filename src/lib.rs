//! # luminance windowing
//!
//! This is the base, abstract crate for windowing common types and functions in luminance. The
//! `luminance` crate provides you with abstracting over OpenGL, but it doesn’t give you a way to
//! create an OpenGL context. This is due to the fact that creating and managing OpenGL contexts is
//! tightly related to the type of application you target. Typical PC applications might need
//! something like **GLFW** or **glutin** whilst others will directly bind to **X11** or **Windows
//! API**. Several crates – `luminance-*` – exist to solve that problem. They all provide a
//! different implementation for a simple need: opening an OpenGL context, opening a window and
//! manage events. In theory, you could even have a `luminance-gtk` or `luminance-qt` to embed
//! `luminance` in surfaces in those libraries.
//!
//! # What’s included
//!
//! This crate exposes several important types that all backends must use. Among them, you’ll find:
//!
//! - `WindowDim`: abstraction over the dimension of a window and its mode (windowed, fullscreen,
//!   fullscreen restricted).
//! - `WindowOpt`: an opaque type giving access to hints to customize the window integration, such
//!   as whether the cursor should be hidden or not.
//!
//! The `Device` trait must be implemented by a backend so that an application is completely
//! agnostic of the backend. This trait defines several basic methods that will help you to:
//!
//! - Retrieve the dimension of the window / framebuffer.
//! - Iterate over the system events captured by your application.
//! - Draw and swap the buffer chain.

/// Dimension metrics.
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
  /// Defaults:
  ///
  /// - `hide_cursor(true)`
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

/// Windowing device.
///
/// This type holds anything related to windowing – window system, events, mostly. The interface
/// is straight forward, so feel free to have a look around.
pub trait Device: Sized {
  /// Type of events.
  type Event;

  /// Type of device errors.
  type Error;

  /// Create a device and bootstrap a luminance environment that lives as long as the device
  /// lives.
  fn new(dim: WindowDim, title: &str, win_opt: WindowOpt) -> Result<Self, Self::Error>;

  /// Size of the framebuffer attached to the window.
  fn size(&self) -> [u32; 2];

  /// Width of the framebuffer attached to the window.
  ///
  /// # Defaults
  ///
  /// Defaults to `.size()[0]`.
  fn width(&self) -> u32 {
    self.size()[0]
  }

  /// Height of the framebuffer attached to the window.
  ///
  /// # Defaults
  ///
  /// Defaults to `.size()[1]`.
  fn height(&self) -> u32 {
    self.size()[1]
  }

  // FIXME: as soon as either ATC (https://github.com/rust-lang/rust/issues/44265) or conservative
  // impl trait in methods (https://github.com/rust-lang/rust/issues/42183) are a real thing,
  // change the interface
  /// Retrieve an iterator over any pulled events.
  fn events<'a>(&'a mut self) -> Box<Iterator<Item = Self::Event> + 'a>;

  /// Perform a draw. You should recall that function each time you want to draw a single frame to
  /// the screen.
  fn draw<F>(&mut self, f: F) where F: FnOnce();
}
