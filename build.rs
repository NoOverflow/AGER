fn main() {
    glib_build_tools::compile_resources(
        &["src/views/template/debugger"],
        "src/views/template/debugger/resources.gresource.xml",
        "debugger.gresource",
    );
}
