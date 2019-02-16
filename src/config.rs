/*
   * Layout Settings
       defaultSplitProportion
       defaultWindowInsert
       * gaps
           width
           draggerWidth
   * Decoration
       Position
       Behavior (Float|Maximize|Minimized|BSPTiled)
       Looks (Buttons Order take xmobar config as reference)
   * Autostarts apps use xdg standard
*/

pub struct Config {
    pub workspaces_config: Vec<WorkspaceConfig>,
}

pub struct WorkspaceConfig {
    pub name: String,
    pub rules: (),
}