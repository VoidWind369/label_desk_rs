fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    embed_resource::compile("icon.rc", embed_resource::NONE);
}