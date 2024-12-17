pub fn eval(expr: String) -> String {
    let out = std::process::Command::new("qalc")
        .arg(expr)
        .output().map_err(|e| e.to_string());

    match out {
        Err(e) => e,
        Ok(out) => String::from_utf8_lossy(&out.stdout).into_owned().trim().split('=').last().unwrap().trim().to_string()
    }
}