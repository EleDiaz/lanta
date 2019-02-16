use log::{error, info, log};

use crate::config::*;
use crate::errors::*;
use crate::keys::KeyCombo;
use crate::keys::KeyHandlers;
use crate::utils::Rectangle;
use crate::window_set::WindowSet;
use crate::x::{Connection, Event, StrutPartial, WindowId, WindowType};
use error_chain::ChainedError;
use std::rc::Rc;

pub struct Lanta {
    connection: Rc<Connection>,
    keys: KeyHandlers,
    window_set: WindowSet,
}

impl Lanta {
    pub fn new<K>(keys: K, config: Config) -> Result<Self>
    where
        K: Into<KeyHandlers>,
    {
        let keys = keys.into();
        let connection = Rc::new(Connection::connect()?);
        connection.install_as_wm(&keys)?;
        let screens = vec![]; // TODO
        let window_set = WindowSet::new(config.workspaces_config, screens);

        let mut wm = Lanta {
            connection: connection.clone(),
            keys,
            window_set,
        };

        // Learn about existing top-level windows.
        let existing_windows = connection.top_level_windows()?;
        for window in existing_windows {
            wm.manage_window(window);
        }

        let names = wm.window_set.get_workspace_names();
        wm.connection
            .update_ewmh_desktops(names, wm.window_set.focused_workspace());

        Ok(wm)
    }

    // fn viewport(&self) -> Rectangle {
    //     let (width, height) = self
    //         .connection
    //         .get_window_geometry(self.connection.root_window_id());
    //     self.screen.viewport(width, height)
    // }

    pub fn run(mut self) {
        info!("Started WM, entering event loop.");
        let event_loop_connection = self.connection.clone();
        let event_loop = event_loop_connection.get_event_loop();
        for event in event_loop {
            match event {
                Event::MapRequest(window_id) => self.on_map_request(window_id),
                Event::UnmapNotify(window_id) => self.on_unmap_notify(&window_id),
                Event::DestroyNotify(window_id) => self.on_destroy_notify(&window_id),
                Event::KeyPress(key) => self.on_key_press(key),
                Event::EnterNotify(window_id) => self.on_enter_notify(&window_id),
            }
        }
        info!("Event loop exiting");
    }

    pub fn manage_window(&mut self, window_id: WindowId) {
        if !self.window_set.contains(&window_id) {
            // TODO: Follow the management rules
            let window_types = self.connection.get_window_types(&window_id);
            let dock = window_types.contains(&WindowType::Dock);

            self.connection
                .enable_window_key_events(&window_id, &self.keys);

            if dock {
                // TODO: get_dock screen and associate to that screen
                // let strut_partial = conn.get_strut_partial(&window_id);
                // let screen_id = self.connection.get_screen(window_id);
                // self.window_set.get_screen(screen_id).add_dock(window_id, strut_partial);
                // self.window_set.update_rectangles(); and paint
                self.connection.map_window(&window_id);
            } else {
                self.connection.enable_window_tracking(&window_id);
                self.window_set.add_window(window_id);
            }
        }
    }

    /// TODO
    fn unmanage_window(&mut self, window_id: &WindowId) {
        self.window_set.remove_window(window_id)
    }

    fn on_map_request(&mut self, window_id: WindowId) {
        self.manage_window(window_id)
    }

    fn on_unmap_notify(&mut self, window_id: &WindowId) {
        // We only receive an unmap notify event when the window is actually
        // unmapped by its application. When our layouts unmap windows, they
        // (should) do it by disabling event tracking first.
        self.unmanage_window(window_id);
    }

    fn on_destroy_notify(&mut self, window_id: &WindowId) {
        self.unmanage_window(window_id);
    }

    fn on_key_press(&mut self, key: KeyCombo) {
        self.keys.get(&key).map(move |handler| {
            if let Err(error) = (handler)(self) {
                error!(
                    "Error running command for key command {:?}: {}",
                    key,
                    error.display_chain().to_string()
                );
            }
        });
    }

    fn on_enter_notify(&mut self, window_id: &WindowId) {
        //self.group_mut().focus(window_id);
    }
}