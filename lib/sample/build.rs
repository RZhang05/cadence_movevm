fn main() {
    let path = "./go_lib";
    let lib = "main";

    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/dev/work/cadence_movevm/lib/sample/go_lib");
    println!("cargo:rustc-link-lib=dylib={}", lib);
    println!("cargo:rustc-link-search=native={}", path);
    
}