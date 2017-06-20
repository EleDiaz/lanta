use x::{Connection, WindowId};


/// A trait implemented by any objects that allow control over a window on the
/// screen.
pub trait Window {
    fn connection(&self) -> &Connection;
    fn id(&self) -> &WindowId;

    fn without_tracking<'a, F>(&'a self, func: F)
    where
        F: Fn(&'a Self),
    {
        self.connection().disable_window_tracking(self.id());
        (func)(self);
        self.connection().enable_window_tracking(self.id());
    }

    /// Maps the window.
    fn map(&self) {
        debug!("Mapping window: {}", self.id());
        self.connection().map_window(self.id());
    }

    /// Unmaps the window.
    fn unmap(&self) {
        debug!("Unmapping window: {}", self.id());
        self.connection().unmap_window(self.id());
    }

    /// Sets the window's position and size.
    fn configure(&self, x: u32, y: u32, width: u32, height: u32) {
        self.connection()
            .configure_window(self.id(), x, y, width, height);
    }

    /// Closes the window.
    fn close(&self) {
        info!("Closing window: {}", self.id());
        self.connection().close_window(self.id());
    }
}
