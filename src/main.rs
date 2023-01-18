use std::fs;
extern crate app_dirs;
use directories::BaseDirs;
fn main() {
    let sspoverride = "[application]
networking/mapdb_api=\"https://soundspaceplus.dev/api/\"
networking/test_url=\"https://google.com\"";

    if let Some(base_dirs) = BaseDirs::new() {
        let appdata = base_dirs.config_dir().to_str().expect("hehehehaw"); // put appdata path into string

        let dapath = format!(r"{}\SoundSpacePlus\override.cfg", appdata); // combine appdata with override location

        fs::write(dapath, sspoverride).expect("uh oh"); // write override
    }
}
