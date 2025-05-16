
pub fn route_path_from_mod_path(mod_path: &str) -> String {

    let mut path = mod_path.split("::").collect::<Vec<_>>();
    for i in 0..path.len() {
        if path[i] == "routes" {
            path.remove(i);
            break;
        } else {
            path[i] = ""
        }
    }



    let mut path = path.join("/").replace("::", "/");
    if !path.starts_with("/") {
        path = format!("/{}", path);
    }
    if path != "/" && path.ends_with('/') {
        path.pop();
    }
    path

}

