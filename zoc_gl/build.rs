// See LICENSE file for copyright and license details.

extern crate gl_generator;
extern crate khronos_api;

fn main() {
    let dest = Path::new(std::os::getenv("OUT_DIR").unwrap());
    let mut file = std::io::File::create(&dest.join("gl_bindings.rs")).unwrap();
    gl_generator::generate_bindings(
        gl_generator::StaticStructGenerator,
        gl_generator::registry::Ns::Gles2,
        khronos_api::GL_XML,
        vec![],
        "2.0",
        "core",
        &mut file
    ).unwrap();
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab: