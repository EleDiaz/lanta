use std::cmp;
use std::collections::hash_map::HashMap;

use log::{debug, error, log};

use crate::{
    bsplayout::BSPLayout,
    config::WorkspaceConfig,
    utils::{Rectangle, ScreenInfo, Reserved},
    x::WindowId,
};

/// XMonad inspired structure
pub struct WindowSet {
    /// Focused screen, between visible screens
    currentScreen: usize,
    /// Avialable screens
    visible: Vec<Screen>,
    /// All Workspaces
    workspaces: Vec<Workspace>,
    /// Windows internal status
    windows_status: HashMap<WindowId, Status>,
}

/// TODO Maybe i need add more types, Panels and others or remove
/// workspace where the window is locate?
pub enum Status {
    /// Tiled with the postion into the tree
    Tiled(usize),
    /// Is floating
    Floating,
    /// The windows is unmapped on screen
    Minimized,
    /// Take whole screen, don't hide docks (Not fullscreen)
    Maximized,
    /// App take all screen hide docks, and set fullscreen mode on app
    FullScreen,
    /// Special
    Dock,
}

impl WindowSet {
    pub fn new(workspaces_config: Vec<WorkspaceConfig>, screens: Vec<ScreenInfo>) -> Self {
        let workspaces: Vec<Workspace> =
            workspaces_config.into_iter().map(Workspace::new).collect();

        if screens.len() == 0 {
            // TODO: exit
            error!("No screens, you didn't need a wm");
        }

        if workspaces.len() < screens.len() {
            error!(
                "You are lacking of workspaces({}) to serve screens({}). There are going to be screens not used",
                workspaces.len(),
                screens.len()
            );
        }

        let visible = screens
            .into_iter()
            .zip(1..workspaces.len())
            .map(|(info, workspace)| Screen::new(workspace, info))
            .collect();

        Self {
            currentScreen: 0,
            visible,
            workspaces,
            windows_status: HashMap::new(),
        }
    }

    pub fn remove_window(&mut self, window_id: &WindowId) {
        debug!("Unmanaging window: {}", window_id);

        match self.windows_status.get(window_id) {
            Some(status) => match status {
                Status::Tiled(_pos) => {}
                Status::Floating => {}
                Status::Minimized => {}
                Status::Maximized => {}
                Status::FullScreen => {}
                Status::Dock => {} // viewport changed?
            },
            None => debug!("Trying to unmanage, an unmanage window"),
        }
    }

    /// Returns whether the window is a member of any group.
    pub fn contains(&self, window_id: &WindowId) -> bool {
        self.windows_status.get(window_id).is_some()
    }

    /// it should focus?
    pub fn add_window(&mut self, window_id: WindowId) {
        if self.contains(&window_id) {
            error!(
                "Asked to add a window that's already managed: {}",
                window_id
            );
            return;
        }
    }

    pub fn focus(&mut self, window_id: &WindowId) {
        // TODO
    }

    pub fn switch_workspace<'a, S>(&'a mut self, name: S)
    where
        S: Into<&'a str>,
    {
        // self.connection.update_ewmh_desktops(&self.groups);
    }

    /// Get workspace
    pub fn get_workspace_names(&self) -> Vec<&str> {
        self.workspaces
            .iter()
            .map(|workspace| &workspace.name as &str)
            .collect()
    }

    /// Get focused workspace position. You can get the its name with
    /// `self.get_workspace_names()[ix]`
    pub fn focused_workspace(&self) -> usize {
        self.visible[self.currentScreen].workspace
    }

    /// Move the focused window from the active group to another named group.
    ///
    /// If the other named group does not exist, then the window is
    /// (unfortunately) lost.
    pub fn move_focused_to_workspace<'a, S>(&'a mut self, name: S)
    where
        S: Into<&'a str>,
    {
        // TODO
    }
}

#[derive(Default)]
pub struct Screen {
    workspace: usize,
    info: ScreenInfo,
    docks: Vec<Dock>,
}

impl Screen {
    pub fn new(workspace: usize, info: ScreenInfo) -> Self {
        Self {
            workspace,
            info,
            docks: vec![],
        }
    }

    pub fn add_dock(&mut self, window_id: WindowId, reserved: Reserved) {
        self.docks.push(Dock {
            window_id,
            reserved,
        });
    }

    pub fn remove_dock(&mut self, window_id: &WindowId) {
        self.docks
            .retain(|d| &d.window_id != window_id);
    }

    /// Figure out the usable area of the screen based on the STRUT_PARTIAL of
    /// all docks.
    pub fn viewport(&self, screen_width: u32, screen_height: u32) -> Rectangle {
        let (left, right, top, bottom) = self
            .docks
            .iter()
            .fold((0, 0, 0, 0), |(left, right, top, bottom), s| {
                // We don't bother looking at the start/end members of the
                // StrutPartial - treating it more like a Strut.
                (
                    cmp::max(left, s.reserved.left),
                    cmp::max(right, s.reserved.right),
                    cmp::max(top, s.reserved.top),
                    cmp::max(bottom, s.reserved.bottom),
                )
            });
        let viewport = Rectangle {
            x: left,
            y: top,
            width: screen_width - left - right,
            height: screen_height - top - bottom,
        };
        debug!("Calculated Viewport as {:?}", viewport);
        viewport
    }
}

pub struct Dock {
    window_id: WindowId,
    reserved: Reserved,
}

#[derive(Default)]
pub struct Workspace {
    /// Workspace name
    name: String,
    /// Floats windows.
    /// TODO: We should able to bring them to front
    floats: Vec<WindowId>,
    /// TODO: Use a mode to select between minimized
    minimized: Vec<WindowId>,
    /// Just one window maximized?
    maximized: Option<WindowId>,
    /// Focused windows into the layout
    focused: Option<usize>,
    /// Tiled windows
    layout: BSPLayout<WindowId>,
}

impl Workspace {
    pub fn new(workspace_config: WorkspaceConfig) -> Self {
        Self {
            name: workspace_config.name,
            floats: vec![],
            minimized: vec![],
            maximized: None,
            focused: None,
            layout: BSPLayout::empty(),
        }
    }
}
