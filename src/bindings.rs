/* automatically generated by rust-bindgen 0.54.1 */

pub const aa_attribute_AA_NORMAL: aa_attribute = 0;
pub const aa_attribute_AA_DIM: aa_attribute = 1;
pub const aa_attribute_AA_BOLD: aa_attribute = 2;
pub const aa_attribute_AA_BOLDFONT: aa_attribute = 3;
pub const aa_attribute_AA_REVERSE: aa_attribute = 4;
pub const aa_attribute_AA_SPECIAL: aa_attribute = 5;
pub type aa_attribute = u32;
pub const aa_dithering_mode_AA_NONE: aa_dithering_mode = 0;
pub const aa_dithering_mode_AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const aa_dithering_mode_AA_FLOYD_S: aa_dithering_mode = 2;
pub const aa_dithering_mode_AA_DITHERTYPES: aa_dithering_mode = 3;
pub type aa_dithering_mode = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_edit {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_hardware_params {
    pub font: *const aa_font,
    pub supported: ::std::os::raw::c_int,
    pub minwidth: ::std::os::raw::c_int,
    pub minheight: ::std::os::raw::c_int,
    pub maxwidth: ::std::os::raw::c_int,
    pub maxheight: ::std::os::raw::c_int,
    pub recwidth: ::std::os::raw::c_int,
    pub recheight: ::std::os::raw::c_int,
    pub mmwidth: ::std::os::raw::c_int,
    pub mmheight: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub dimmul: f64,
    pub boldmul: f64,
}
#[test]
fn bindgen_test_layout_aa_hardware_params() {
    assert_eq!(
        ::std::mem::size_of::<aa_hardware_params>(),
        72usize,
        concat!("Size of: ", stringify!(aa_hardware_params))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_hardware_params>(),
        8usize,
        concat!("Alignment of ", stringify!(aa_hardware_params))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).font as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(font)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).supported as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(supported)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).minwidth as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(minwidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).minheight as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(minheight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).maxwidth as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(maxwidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).maxheight as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(maxheight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).recwidth as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(recwidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).recheight as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(recheight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).mmwidth as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(mmwidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).mmheight as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(mmheight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).width as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).height as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).dimmul as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(dimmul)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_hardware_params>())).boldmul as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_hardware_params),
            "::",
            stringify!(boldmul)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_context {
    pub driver: *const aa_driver,
    pub kbddriver: *const aa_kbddriver,
    pub mousedriver: *const aa_mousedriver,
    pub params: aa_hardware_params,
    pub driverparams: aa_hardware_params,
    pub mulx: ::std::os::raw::c_int,
    pub muly: ::std::os::raw::c_int,
    pub imgwidth: ::std::os::raw::c_int,
    pub imgheight: ::std::os::raw::c_int,
    pub imagebuffer: *mut ::std::os::raw::c_uchar,
    pub textbuffer: *mut ::std::os::raw::c_uchar,
    pub attrbuffer: *mut ::std::os::raw::c_uchar,
    pub table: *mut ::std::os::raw::c_ushort,
    pub filltable: *mut ::std::os::raw::c_ushort,
    pub parameters: *mut parameters,
    pub cursorx: ::std::os::raw::c_int,
    pub cursory: ::std::os::raw::c_int,
    pub cursorstate: ::std::os::raw::c_int,
    pub mousex: ::std::os::raw::c_int,
    pub mousey: ::std::os::raw::c_int,
    pub buttons: ::std::os::raw::c_int,
    pub mousemode: ::std::os::raw::c_int,
    pub resizehandler: ::std::option::Option<unsafe extern "C" fn(arg1: *mut aa_context)>,
    pub driverdata: *mut ::std::os::raw::c_void,
    pub kbddriverdata: *mut ::std::os::raw::c_void,
    pub mousedriverdata: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_aa_context() {
    assert_eq!(
        ::std::mem::size_of::<aa_context>(),
        296usize,
        concat!("Size of: ", stringify!(aa_context))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_context>(),
        8usize,
        concat!("Alignment of ", stringify!(aa_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).driver as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(driver)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).kbddriver as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(kbddriver)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).mousedriver as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(mousedriver)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).params as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(params)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).driverparams as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(driverparams)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).mulx as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(mulx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).muly as *const _ as usize },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(muly)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).imgwidth as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(imgwidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).imgheight as *const _ as usize },
        180usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(imgheight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).imagebuffer as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(imagebuffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).textbuffer as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(textbuffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).attrbuffer as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(attrbuffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).table as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).filltable as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(filltable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).parameters as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(parameters)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).cursorx as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(cursorx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).cursory as *const _ as usize },
        236usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(cursory)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).cursorstate as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(cursorstate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).mousex as *const _ as usize },
        244usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(mousex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).mousey as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(mousey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).buttons as *const _ as usize },
        252usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(buttons)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).mousemode as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(mousemode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).resizehandler as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(resizehandler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).driverdata as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(driverdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).kbddriverdata as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(kbddriverdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_context>())).mousedriverdata as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_context),
            "::",
            stringify!(mousedriverdata)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_driver {
    pub shortname: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const aa_hardware_params,
            arg2: *const ::std::os::raw::c_void,
            arg3: *mut aa_hardware_params,
            arg4: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub uninit: ::std::option::Option<unsafe extern "C" fn(arg1: *mut aa_context)>,
    pub getsize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut aa_context,
            arg2: *mut ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_int,
        ),
    >,
    pub setattr: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut aa_context, arg2: ::std::os::raw::c_int),
    >,
    pub print: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut aa_context, arg2: *const ::std::os::raw::c_char),
    >,
    pub gotoxy: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut aa_context,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub flush: ::std::option::Option<unsafe extern "C" fn(arg1: *mut aa_context)>,
    pub cursormode: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut aa_context, arg2: ::std::os::raw::c_int),
    >,
}
#[test]
fn bindgen_test_layout_aa_driver() {
    assert_eq!(
        ::std::mem::size_of::<aa_driver>(),
        80usize,
        concat!("Size of: ", stringify!(aa_driver))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_driver>(),
        8usize,
        concat!("Alignment of ", stringify!(aa_driver))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).shortname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(shortname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).init as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).uninit as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(uninit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).getsize as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(getsize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).setattr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(setattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).print as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(print)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).gotoxy as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(gotoxy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).flush as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(flush)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_driver>())).cursormode as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_driver),
            "::",
            stringify!(cursormode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_kbddriver {
    pub shortname: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut aa_context,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub uninit: ::std::option::Option<unsafe extern "C" fn(arg1: *mut aa_context)>,
    pub getkey: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut aa_context,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
#[test]
fn bindgen_test_layout_aa_kbddriver() {
    assert_eq!(
        ::std::mem::size_of::<aa_kbddriver>(),
        48usize,
        concat!("Size of: ", stringify!(aa_kbddriver))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_kbddriver>(),
        8usize,
        concat!("Alignment of ", stringify!(aa_kbddriver))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_kbddriver>())).shortname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_kbddriver),
            "::",
            stringify!(shortname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_kbddriver>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_kbddriver),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_kbddriver>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_kbddriver),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_kbddriver>())).init as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_kbddriver),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_kbddriver>())).uninit as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_kbddriver),
            "::",
            stringify!(uninit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_kbddriver>())).getkey as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_kbddriver),
            "::",
            stringify!(getkey)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_mousedriver {
    pub shortname: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut aa_context,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub uninit: ::std::option::Option<unsafe extern "C" fn(arg1: *mut aa_context)>,
    pub getmouse: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut aa_context,
            arg2: *mut ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_int,
        ),
    >,
    pub cursormode: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut aa_context, arg2: ::std::os::raw::c_int),
    >,
}
#[test]
fn bindgen_test_layout_aa_mousedriver() {
    assert_eq!(
        ::std::mem::size_of::<aa_mousedriver>(),
        56usize,
        concat!("Size of: ", stringify!(aa_mousedriver))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_mousedriver>(),
        8usize,
        concat!("Alignment of ", stringify!(aa_mousedriver))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).shortname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(shortname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).init as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).uninit as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(uninit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).getmouse as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(getmouse)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_mousedriver>())).cursormode as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_mousedriver),
            "::",
            stringify!(cursormode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_renderparams {
    pub bright: ::std::os::raw::c_int,
    pub contrast: ::std::os::raw::c_int,
    pub gamma: f32,
    pub dither: aa_dithering_mode,
    pub inversion: ::std::os::raw::c_int,
    pub randomval: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_aa_renderparams() {
    assert_eq!(
        ::std::mem::size_of::<aa_renderparams>(),
        24usize,
        concat!("Size of: ", stringify!(aa_renderparams))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_renderparams>(),
        4usize,
        concat!("Alignment of ", stringify!(aa_renderparams))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_renderparams>())).bright as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_renderparams),
            "::",
            stringify!(bright)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_renderparams>())).contrast as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_renderparams),
            "::",
            stringify!(contrast)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_renderparams>())).gamma as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_renderparams),
            "::",
            stringify!(gamma)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_renderparams>())).dither as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_renderparams),
            "::",
            stringify!(dither)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_renderparams>())).inversion as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_renderparams),
            "::",
            stringify!(inversion)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_renderparams>())).randomval as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_renderparams),
            "::",
            stringify!(randomval)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_font {
    pub data: *const ::std::os::raw::c_uchar,
    pub height: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
    pub shortname: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_aa_font() {
    assert_eq!(
        ::std::mem::size_of::<aa_font>(),
        32usize,
        concat!("Size of: ", stringify!(aa_font))
    );
    assert_eq!(
        ::std::mem::align_of::<aa_font>(),
        8usize,
        concat!("Alignment of ", stringify!(aa_font))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_font>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_font),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_font>())).height as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_font),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_font>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_font),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<aa_font>())).shortname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(aa_font),
            "::",
            stringify!(shortname)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aa_linkedlist {
    _unused: [u8; 0],
}
pub type aa_palette = [::std::os::raw::c_int; 256usize];
extern "C" {
    pub fn aa_scrwidth(a: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_scrheight(a: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_mmwidth(a: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_mmheight(a: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_imgwidth(a: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_imgheight(a: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_image(a: *mut aa_context) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn aa_text(a: *mut aa_context) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn aa_attrs(a: *mut aa_context) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn aa_currentfont(a: *mut aa_context) -> *const aa_font;
}
extern "C" {
    pub fn aa_autoinit(params: *const aa_hardware_params) -> *mut aa_context;
}
extern "C" {
    pub fn aa_autoinitkbd(
        context: *mut aa_context,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_autoinitmouse(
        c: *mut aa_context,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_recommendhi(l: *mut *mut aa_linkedlist, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_recommendlow(l: *mut *mut aa_linkedlist, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_getfirst(l: *mut *mut aa_linkedlist) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn aa_init(
        driver: *const aa_driver,
        defparams: *const aa_hardware_params,
        driverdata: *const ::std::os::raw::c_void,
    ) -> *mut aa_context;
}
extern "C" {
    pub fn aa_initkbd(
        context: *mut aa_context,
        drv: *const aa_kbddriver,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_initmouse(
        c: *mut aa_context,
        d: *const aa_mousedriver,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_close(c: *mut aa_context);
}
extern "C" {
    pub fn aa_uninitkbd(context: *mut aa_context);
}
extern "C" {
    pub fn aa_uninitmouse(context: *mut aa_context);
}
extern "C" {
    pub fn aa_fastrender(
        c: *mut aa_context,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn aa_render(
        c: *mut aa_context,
        p: *const aa_renderparams,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn aa_renderpalette(
        c: *mut aa_context,
        table: *mut ::std::os::raw::c_int,
        p: *const aa_renderparams,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn aa_getrenderparams() -> *mut aa_renderparams;
}
extern "C" {
    pub fn aa_flush(c: *mut aa_context);
}
extern "C" {
    pub fn aa_puts(
        c: *mut aa_context,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        attr: aa_attribute,
        s: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn aa_printf(
        c: *mut aa_context,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        attr: aa_attribute,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_gotoxy(c: *mut aa_context, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn aa_hidecursor(c: *mut aa_context);
}
extern "C" {
    pub fn aa_showcursor(c: *mut aa_context);
}
extern "C" {
    pub fn aa_getmouse(
        c: *mut aa_context,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        b: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn aa_hidemouse(c: *mut aa_context);
}
extern "C" {
    pub fn aa_showmouse(c: *mut aa_context);
}
extern "C" {
    pub fn aa_registerfont(f: *const aa_font) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_setsupported(c: *mut aa_context, supported: ::std::os::raw::c_int);
}
extern "C" {
    pub fn aa_setfont(c: *mut aa_context, font: *const aa_font);
}
extern "C" {
    pub fn aa_getevent(c: *mut aa_context, wait: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_getkey(c: *mut aa_context, wait: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_resize(c: *mut aa_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_resizehandler(
        c: *mut aa_context,
        handler: ::std::option::Option<unsafe extern "C" fn(arg1: *mut aa_context)>,
    );
}
extern "C" {
    pub fn aa_parseoptions(
        p: *mut aa_hardware_params,
        r: *mut aa_renderparams,
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aa_edit(
        c: *mut aa_context,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
        s: *mut ::std::os::raw::c_char,
        maxsize: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn aa_createedit(
        c: *mut aa_context,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
        s: *mut ::std::os::raw::c_char,
        maxsize: ::std::os::raw::c_int,
    ) -> *mut aa_edit;
}
extern "C" {
    pub fn aa_editkey(e: *mut aa_edit, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn aa_putpixel(
        c: *mut aa_context,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn aa_recommendhikbd(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_recommendlowkbd(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_recommendhimouse(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_recommendlowmouse(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_recommendhidisplay(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn aa_recommendlowdisplay(name: *const ::std::os::raw::c_char);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct parameters {
    pub _address: u8,
}
