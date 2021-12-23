#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* automatically generated by rust-bindgen */
pub const PA_RATE_MAX: u32 = 48000 * 8;

pub const PA_SAMPLE_U8: c_int = 0;
pub const PA_SAMPLE_ALAW: c_int = 1;
pub const PA_SAMPLE_ULAW: c_int = 2;
pub const PA_SAMPLE_S16LE: c_int = 3;
pub const PA_SAMPLE_S16BE: c_int = 4;
pub const PA_SAMPLE_FLOAT32LE: c_int = 5;
pub const PA_SAMPLE_FLOAT32BE: c_int = 6;
pub const PA_SAMPLE_S32LE: c_int = 7;
pub const PA_SAMPLE_S32BE: c_int = 8;
pub const PA_SAMPLE_S24LE: c_int = 9;
pub const PA_SAMPLE_S24BE: c_int = 10;
pub const PA_SAMPLE_S24_32LE: c_int = 11;
pub const PA_SAMPLE_S24_32BE: c_int = 12;
pub const PA_SAMPLE_MAX: c_int = 13;
pub const PA_SAMPLE_INVALID: c_int = -1;
pub type pa_sample_format_t = c_int;

pub const PA_VOLUME_MUTED: c_uint = 0;
pub const PA_VOLUME_NORM: c_uint = 0x10000;
pub const PA_VOLUME_MAX: c_uint = 0x7fffffff;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_pa_sample_spec {
    pub format: pa_sample_format_t,
    pub rate: u32,
    pub channels: u8,
}

impl ::std::default::Default for Struct_pa_sample_spec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_sample_spec = Struct_pa_sample_spec;
pub type pa_usec_t = u64;

// From pulse/timeval.h
pub const PA_USEC_PER_MSEC: pa_usec_t = 1_000;
pub const PA_USEC_PER_SEC: pa_usec_t = 1_000_000;

pub const PA_CONTEXT_UNCONNECTED: c_int = 0;
pub const PA_CONTEXT_CONNECTING: c_int = 1;
pub const PA_CONTEXT_AUTHORIZING: c_int = 2;
pub const PA_CONTEXT_SETTING_NAME: c_int = 3;
pub const PA_CONTEXT_READY: c_int = 4;
pub const PA_CONTEXT_FAILED: c_int = 5;
pub const PA_CONTEXT_TERMINATED: c_int = 6;
pub type pa_context_state_t = c_int;

#[allow(non_snake_case)]
pub fn PA_CONTEXT_IS_GOOD(x: pa_context_state_t) -> bool {
    x == PA_CONTEXT_CONNECTING
        || x == PA_CONTEXT_AUTHORIZING
        || x == PA_CONTEXT_SETTING_NAME
        || x == PA_CONTEXT_READY
}

pub const PA_STREAM_UNCONNECTED: c_int = 0;
pub const PA_STREAM_CREATING: c_int = 1;
pub const PA_STREAM_READY: c_int = 2;
pub const PA_STREAM_FAILED: c_int = 3;
pub const PA_STREAM_TERMINATED: c_int = 4;
pub type pa_stream_state_t = c_int;

#[allow(non_snake_case)]
pub fn PA_STREAM_IS_GOOD(x: pa_stream_state_t) -> bool {
    x == PA_STREAM_CREATING || x == PA_STREAM_READY
}

pub const PA_OPERATION_RUNNING: c_int = 0;
pub const PA_OPERATION_DONE: c_int = 1;
pub const PA_OPERATION_CANCELLED: c_int = 2;
pub type pa_operation_state_t = c_int;

pub const PA_CONTEXT_NOFLAGS: c_uint = 0;
pub const PA_CONTEXT_NOAUTOSPAWN: c_uint = 1;
pub const PA_CONTEXT_NOFAIL: c_uint = 2;
pub type pa_context_flags_t = c_uint;

pub const PA_DIRECTION_OUTPUT: c_int = 1;
pub const PA_DIRECTION_INPUT: c_int = 2;
pub type pa_direction_t = c_int;

pub const PA_DEVICE_TYPE_SINK: c_int = 0;
pub const PA_DEVICE_TYPE_SOURCE: c_int = 1;
pub type pa_device_type_t = c_int;

pub const PA_STREAM_NODIRECTION: c_int = 0;
pub const PA_STREAM_PLAYBACK: c_int = 1;
pub const PA_STREAM_RECORD: c_int = 2;
pub const PA_STREAM_UPLOAD: c_int = 3;
pub type pa_stream_direction_t = c_int;

pub const PA_STREAM_NOFLAGS: c_uint = 0x0_0000;
pub const PA_STREAM_START_CORKED: c_uint = 0x0_0001;
pub const PA_STREAM_INTERPOLATE_TIMING: c_uint = 0x0_0002;
pub const PA_STREAM_NOT_MONOTONIC: c_uint = 0x0_0004;
pub const PA_STREAM_AUTO_TIMING_UPDATE: c_uint = 0x0_0008;
pub const PA_STREAM_NO_REMAP_CHANNELS: c_uint = 0x0_0010;
pub const PA_STREAM_NO_REMIX_CHANNELS: c_uint = 0x0_0020;
pub const PA_STREAM_FIX_FORMAT: c_uint = 0x0_0040;
pub const PA_STREAM_FIX_RATE: c_uint = 0x0_0080;
pub const PA_STREAM_FIX_CHANNELS: c_uint = 0x0_0100;
pub const PA_STREAM_DONT_MOVE: c_uint = 0x0_0200;
pub const PA_STREAM_VARIABLE_RATE: c_uint = 0x0_0400;
pub const PA_STREAM_PEAK_DETECT: c_uint = 0x0_0800;
pub const PA_STREAM_START_MUTED: c_uint = 0x0_1000;
pub const PA_STREAM_ADJUST_LATENCY: c_uint = 0x0_2000;
pub const PA_STREAM_EARLY_REQUESTS: c_uint = 0x0_4000;
pub const PA_STREAM_DONT_INHIBIT_AUTO_SUSPEND: c_uint = 0x0_8000;
pub const PA_STREAM_START_UNMUTED: c_uint = 0x1_0000;
pub const PA_STREAM_FAIL_ON_SUSPEND: c_uint = 0x2_0000;
pub const PA_STREAM_RELATIVE_VOLUME: c_uint = 0x4_0000;
pub const PA_STREAM_PASSTHROUGH: c_uint = 0x8_0000;
pub type pa_stream_flags_t = c_uint;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_buffer_attr {
    pub maxlength: u32,
    pub tlength: u32,
    pub prebuf: u32,
    pub minreq: u32,
    pub fragsize: u32,
}

impl ::std::default::Default for pa_buffer_attr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub const PA_OK: c_int = 0;
pub const PA_ERR_ACCESS: c_int = 1;
pub const PA_ERR_COMMAND: c_int = 2;
pub const PA_ERR_INVALID: c_int = 3;
pub const PA_ERR_EXIST: c_int = 4;
pub const PA_ERR_NOENTITY: c_int = 5;
pub const PA_ERR_CONNECTIONREFUSED: c_int = 6;
pub const PA_ERR_PROTOCOL: c_int = 7;
pub const PA_ERR_TIMEOUT: c_int = 8;
pub const PA_ERR_AUTHKEY: c_int = 9;
pub const PA_ERR_INTERNAL: c_int = 10;
pub const PA_ERR_CONNECTIONTERMINATED: c_int = 11;
pub const PA_ERR_KILLED: c_int = 12;
pub const PA_ERR_INVALIDSERVER: c_int = 13;
pub const PA_ERR_MODINITFAILED: c_int = 14;
pub const PA_ERR_BADSTATE: c_int = 15;
pub const PA_ERR_NODATA: c_int = 16;
pub const PA_ERR_VERSION: c_int = 17;
pub const PA_ERR_TOOLARGE: c_int = 18;
pub const PA_ERR_NOTSUPPORTED: c_int = 19;
pub const PA_ERR_UNKNOWN: c_int = 20;
pub const PA_ERR_NOEXTENSION: c_int = 21;
pub const PA_ERR_OBSOLETE: c_int = 22;
pub const PA_ERR_NOTIMPLEMENTED: c_int = 23;
pub const PA_ERR_FORKED: c_int = 24;
pub const PA_ERR_IO: c_int = 25;
pub const PA_ERR_BUSY: c_int = 26;
pub const PA_ERR_MAX: c_int = 27;
pub type pa_error_code_t = c_int;

pub const PA_SUBSCRIPTION_MASK_NULL: c_uint = 0x0;
pub const PA_SUBSCRIPTION_MASK_SINK: c_uint = 0x1;
pub const PA_SUBSCRIPTION_MASK_SOURCE: c_uint = 0x2;
pub const PA_SUBSCRIPTION_MASK_SINK_INPUT: c_uint = 0x4;
pub const PA_SUBSCRIPTION_MASK_SOURCE_OUTPUT: c_uint = 0x8;
pub const PA_SUBSCRIPTION_MASK_MODULE: c_uint = 0x10;
pub const PA_SUBSCRIPTION_MASK_CLIENT: c_uint = 0x20;
pub const PA_SUBSCRIPTION_MASK_SAMPLE_CACHE: c_uint = 0x40;
pub const PA_SUBSCRIPTION_MASK_SERVER: c_uint = 0x80;
pub const PA_SUBSCRIPTION_MASK_AUTOLOAD: c_uint = 0x100;
pub const PA_SUBSCRIPTION_MASK_CARD: c_uint = 0x200;
pub const PA_SUBSCRIPTION_MASK_ALL: c_uint = 0x3FF;
pub type pa_subscription_mask_t = c_uint;

pub const PA_SUBSCRIPTION_EVENT_SINK: c_int = 0;
pub const PA_SUBSCRIPTION_EVENT_SOURCE: c_int = 1;
pub const PA_SUBSCRIPTION_EVENT_SINK_INPUT: c_int = 2;
pub const PA_SUBSCRIPTION_EVENT_SOURCE_OUTPUT: c_int = 3;
pub const PA_SUBSCRIPTION_EVENT_MODULE: c_int = 4;
pub const PA_SUBSCRIPTION_EVENT_CLIENT: c_int = 5;
pub const PA_SUBSCRIPTION_EVENT_SAMPLE_CACHE: c_int = 6;
pub const PA_SUBSCRIPTION_EVENT_SERVER: c_int = 7;
pub const PA_SUBSCRIPTION_EVENT_AUTOLOAD: c_int = 8;
pub const PA_SUBSCRIPTION_EVENT_CARD: c_int = 9;
pub const PA_SUBSCRIPTION_EVENT_FACILITY_MASK: c_int = 15;
pub const PA_SUBSCRIPTION_EVENT_NEW: c_int = 0;
pub const PA_SUBSCRIPTION_EVENT_CHANGE: c_int = 16;
pub const PA_SUBSCRIPTION_EVENT_REMOVE: c_int = 32;
pub const PA_SUBSCRIPTION_EVENT_TYPE_MASK: c_int = 48;
pub type pa_subscription_event_type_t = c_int;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_timing_info {
    pub timestamp: timeval,
    pub synchronized_clocks: c_int,
    pub sink_usec: pa_usec_t,
    pub source_usec: pa_usec_t,
    pub transport_usec: pa_usec_t,
    pub playing: c_int,
    pub write_index_corrupt: c_int,
    pub write_index: i64,
    pub read_index_corrupt: c_int,
    pub read_index: i64,
    pub configured_sink_usec: pa_usec_t,
    pub configured_source_usec: pa_usec_t,
    pub since_underrun: i64,
}

impl ::std::default::Default for pa_timing_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_spawn_api {
    pub prefork: Option<extern "C" fn() -> ()>,
    pub postfork: Option<extern "C" fn() -> ()>,
    pub atfork: Option<extern "C" fn() -> ()>,
}

impl ::std::default::Default for pa_spawn_api {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub const PA_SEEK_RELATIVE: c_int = 0;
pub const PA_SEEK_ABSOLUTE: c_int = 1;
pub const PA_SEEK_RELATIVE_ON_READ: c_int = 2;
pub const PA_SEEK_RELATIVE_END: c_int = 3;
pub type pa_seek_mode_t = c_int;

pub const PA_SINK_NOFLAGS: c_uint = 0x000;
pub const PA_SINK_HW_VOLUME_CTRL: c_uint = 0x001;
pub const PA_SINK_LATENCY: c_uint = 0x002;
pub const PA_SINK_HARDWARE: c_uint = 0x004;
pub const PA_SINK_NETWORK: c_uint = 0x008;
pub const PA_SINK_HW_MUTE_CTRL: c_uint = 0x010;
pub const PA_SINK_DECIBEL_VOLUME: c_uint = 0x020;
pub const PA_SINK_FLAT_VOLUME: c_uint = 0x040;
pub const PA_SINK_DYNAMIC_LATENCY: c_uint = 0x080;
pub const PA_SINK_SET_FORMATS: c_uint = 0x100;
pub type pa_sink_flags_t = c_uint;

pub const PA_SINK_INVALID_STATE: c_int = -1;
pub const PA_SINK_RUNNING: c_int = 0;
pub const PA_SINK_IDLE: c_int = 1;
pub const PA_SINK_SUSPENDED: c_int = 2;
pub const PA_SINK_INIT: c_int = -2;
pub const PA_SINK_UNLINKED: c_int = -3;
pub type pa_sink_state_t = c_int;

pub const PA_SOURCE_NOFLAGS: c_uint = 0x00;
pub const PA_SOURCE_HW_VOLUME_CTRL: c_uint = 0x01;
pub const PA_SOURCE_LATENCY: c_uint = 0x02;
pub const PA_SOURCE_HARDWARE: c_uint = 0x04;
pub const PA_SOURCE_NETWORK: c_uint = 0x08;
pub const PA_SOURCE_HW_MUTE_CTRL: c_uint = 0x10;
pub const PA_SOURCE_DECIBEL_VOLUME: c_uint = 0x20;
pub const PA_SOURCE_DYNAMIC_LATENCY: c_uint = 0x40;
pub const PA_SOURCE_FLAT_VOLUME: c_uint = 0x80;
pub type pa_source_flags_t = c_uint;

pub const PA_SOURCE_INVALID_STATE: c_int = -1;
pub const PA_SOURCE_RUNNING: c_int = 0;
pub const PA_SOURCE_IDLE: c_int = 1;
pub const PA_SOURCE_SUSPENDED: c_int = 2;
pub const PA_SOURCE_INIT: c_int = -2;
pub const PA_SOURCE_UNLINKED: c_int = -3;
pub type pa_source_state_t = c_int;

pub type pa_free_cb_t = Option<unsafe extern "C" fn(p: *mut c_void) -> ()>;

pub const PA_PORT_AVAILABLE_UNKNOWN: c_int = 0;
pub const PA_PORT_AVAILABLE_NO: c_int = 1;
pub const PA_PORT_AVAILABLE_YES: c_int = 2;
pub type pa_port_available_t = c_int;

pub const PA_IO_EVENT_NULL: c_int = 0;
pub const PA_IO_EVENT_INPUT: c_int = 1;
pub const PA_IO_EVENT_OUTPUT: c_int = 2;
pub const PA_IO_EVENT_HANGUP: c_int = 4;
pub const PA_IO_EVENT_ERROR: c_int = 8;
pub type pa_io_event_flags_t = c_int;

pub enum pa_io_event {}
pub type pa_io_event_cb_t = Option<
    unsafe extern "C" fn(
        ea: *mut pa_mainloop_api,
        e: *mut pa_io_event,
        fd: c_int,
        events: pa_io_event_flags_t,
        userdata: *mut c_void,
    ),
>;
pub type pa_io_event_destroy_cb_t = Option<
    unsafe extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_io_event, userdata: *mut c_void),
>;
pub enum pa_time_event {}
pub type pa_time_event_cb_t = Option<
    unsafe extern "C" fn(
        a: *mut pa_mainloop_api,
        e: *mut pa_time_event,
        tv: *const timeval,
        userdata: *mut c_void,
    ),
>;
pub type pa_time_event_destroy_cb_t = Option<
    unsafe extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_time_event, userdata: *mut c_void),
>;

pub enum pa_defer_event {}
pub type pa_defer_event_cb_t = Option<
    unsafe extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_defer_event, userdata: *mut c_void),
>;
pub type pa_defer_event_destroy_cb_t = Option<
    unsafe extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_defer_event, userdata: *mut c_void),
>;
pub type IoNewFn = Option<
    unsafe extern "C" fn(
        a: *mut pa_mainloop_api,
        fd: c_int,
        events: pa_io_event_flags_t,
        cb: pa_io_event_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_io_event,
>;
pub type TimeNewFn = Option<
    unsafe extern "C" fn(
        a: *mut pa_mainloop_api,
        tv: *const timeval,
        cb: pa_time_event_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_time_event,
>;
pub type DeferNewFn = Option<
    unsafe extern "C" fn(
        a: *mut pa_mainloop_api,
        cb: pa_defer_event_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_defer_event,
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_mainloop_api {
    pub userdata: *mut c_void,
    pub io_new: IoNewFn,
    pub io_enable: Option<unsafe extern "C" fn(e: *mut pa_io_event, events: pa_io_event_flags_t)>,
    pub io_free: Option<unsafe extern "C" fn(e: *mut pa_io_event)>,
    pub io_set_destroy:
        Option<unsafe extern "C" fn(e: *mut pa_io_event, cb: pa_io_event_destroy_cb_t)>,
    pub time_new: TimeNewFn,
    pub time_restart: Option<unsafe extern "C" fn(e: *mut pa_time_event, tv: *const timeval)>,
    pub time_free: Option<unsafe extern "C" fn(e: *mut pa_time_event)>,
    pub time_set_destroy:
        Option<unsafe extern "C" fn(e: *mut pa_time_event, cb: pa_time_event_destroy_cb_t)>,
    pub defer_new: DeferNewFn,
    pub defer_enable: Option<unsafe extern "C" fn(e: *mut pa_defer_event, b: c_int)>,
    pub defer_free: Option<unsafe extern "C" fn(e: *mut pa_defer_event)>,
    pub defer_set_destroy:
        Option<unsafe extern "C" fn(e: *mut pa_defer_event, cb: pa_defer_event_destroy_cb_t)>,
    pub quit: Option<unsafe extern "C" fn(a: *mut pa_mainloop_api, retval: c_int)>,
}

impl ::std::default::Default for pa_mainloop_api {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_mainloop_api_once_cb_t =
    Option<unsafe extern "C" fn(m: *mut pa_mainloop_api, userdata: *mut c_void)>;

pub enum pa_proplist {}

pub const PA_UPDATE_SET: c_int = 0;
pub const PA_UPDATE_MERGE: c_int = 1;
pub const PA_UPDATE_REPLACE: c_int = 2;
pub type pa_update_mode_t = c_int;

pub const PA_CHANNEL_POSITION_INVALID: c_int = -1;
pub const PA_CHANNEL_POSITION_MONO: c_int = 0;
pub const PA_CHANNEL_POSITION_FRONT_LEFT: c_int = 1;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT: c_int = 2;
pub const PA_CHANNEL_POSITION_FRONT_CENTER: c_int = 3;
pub const PA_CHANNEL_POSITION_LEFT: c_int = 1;
pub const PA_CHANNEL_POSITION_RIGHT: c_int = 2;
pub const PA_CHANNEL_POSITION_CENTER: c_int = 3;
pub const PA_CHANNEL_POSITION_REAR_CENTER: c_int = 4;
pub const PA_CHANNEL_POSITION_REAR_LEFT: c_int = 5;
pub const PA_CHANNEL_POSITION_REAR_RIGHT: c_int = 6;
pub const PA_CHANNEL_POSITION_LFE: c_int = 7;
pub const PA_CHANNEL_POSITION_SUBWOOFER: c_int = 7;
pub const PA_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER: c_int = 8;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER: c_int = 9;
pub const PA_CHANNEL_POSITION_SIDE_LEFT: c_int = 10;
pub const PA_CHANNEL_POSITION_SIDE_RIGHT: c_int = 11;
pub const PA_CHANNEL_POSITION_AUX0: c_int = 12;
pub const PA_CHANNEL_POSITION_AUX1: c_int = 13;
pub const PA_CHANNEL_POSITION_AUX2: c_int = 14;
pub const PA_CHANNEL_POSITION_AUX3: c_int = 15;
pub const PA_CHANNEL_POSITION_AUX4: c_int = 16;
pub const PA_CHANNEL_POSITION_AUX5: c_int = 17;
pub const PA_CHANNEL_POSITION_AUX6: c_int = 18;
pub const PA_CHANNEL_POSITION_AUX7: c_int = 19;
pub const PA_CHANNEL_POSITION_AUX8: c_int = 20;
pub const PA_CHANNEL_POSITION_AUX9: c_int = 21;
pub const PA_CHANNEL_POSITION_AUX10: c_int = 22;
pub const PA_CHANNEL_POSITION_AUX11: c_int = 23;
pub const PA_CHANNEL_POSITION_AUX12: c_int = 24;
pub const PA_CHANNEL_POSITION_AUX13: c_int = 25;
pub const PA_CHANNEL_POSITION_AUX14: c_int = 26;
pub const PA_CHANNEL_POSITION_AUX15: c_int = 27;
pub const PA_CHANNEL_POSITION_AUX16: c_int = 28;
pub const PA_CHANNEL_POSITION_AUX17: c_int = 29;
pub const PA_CHANNEL_POSITION_AUX18: c_int = 30;
pub const PA_CHANNEL_POSITION_AUX19: c_int = 31;
pub const PA_CHANNEL_POSITION_AUX20: c_int = 32;
pub const PA_CHANNEL_POSITION_AUX21: c_int = 33;
pub const PA_CHANNEL_POSITION_AUX22: c_int = 34;
pub const PA_CHANNEL_POSITION_AUX23: c_int = 35;
pub const PA_CHANNEL_POSITION_AUX24: c_int = 36;
pub const PA_CHANNEL_POSITION_AUX25: c_int = 37;
pub const PA_CHANNEL_POSITION_AUX26: c_int = 38;
pub const PA_CHANNEL_POSITION_AUX27: c_int = 39;
pub const PA_CHANNEL_POSITION_AUX28: c_int = 40;
pub const PA_CHANNEL_POSITION_AUX29: c_int = 41;
pub const PA_CHANNEL_POSITION_AUX30: c_int = 42;
pub const PA_CHANNEL_POSITION_AUX31: c_int = 43;
pub const PA_CHANNEL_POSITION_TOP_CENTER: c_int = 44;
pub const PA_CHANNEL_POSITION_TOP_FRONT_LEFT: c_int = 45;
pub const PA_CHANNEL_POSITION_TOP_FRONT_RIGHT: c_int = 46;
pub const PA_CHANNEL_POSITION_TOP_FRONT_CENTER: c_int = 47;
pub const PA_CHANNEL_POSITION_TOP_REAR_LEFT: c_int = 48;
pub const PA_CHANNEL_POSITION_TOP_REAR_RIGHT: c_int = 49;
pub const PA_CHANNEL_POSITION_TOP_REAR_CENTER: c_int = 50;
pub const PA_CHANNEL_POSITION_MAX: c_int = 51;
pub type pa_channel_position_t = c_int;
pub type pa_channel_position_mask_t = u64;

pub const PA_CHANNEL_MAP_AIFF: c_int = 0;
pub const PA_CHANNEL_MAP_ALSA: c_int = 1;
pub const PA_CHANNEL_MAP_AUX: c_int = 2;
pub const PA_CHANNEL_MAP_WAVEEX: c_int = 3;
pub const PA_CHANNEL_MAP_OSS: c_int = 4;
pub const PA_CHANNEL_MAP_DEF_MAX: c_int = 5;
pub const PA_CHANNEL_MAP_DEFAULT: c_int = 0;
pub type pa_channel_map_def_t = c_int;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_channel_map {
    pub channels: u8,
    pub map: [pa_channel_position_t; 32usize],
}

impl ::std::default::Default for pa_channel_map {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub const PA_ENCODING_ANY: c_int = 0;
pub const PA_ENCODING_PCM: c_int = 1;
pub const PA_ENCODING_AC3_IEC61937: c_int = 2;
pub const PA_ENCODING_EAC3_IEC61937: c_int = 3;
pub const PA_ENCODING_MPEG_IEC61937: c_int = 4;
pub const PA_ENCODING_DTS_IEC61937: c_int = 5;
pub const PA_ENCODING_MPEG2_AAC_IEC61937: c_int = 6;
pub const PA_ENCODING_MAX: c_int = 7;
pub const PA_ENCODING_INVALID: c_int = -1;
pub type pa_encoding_t = c_int;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_format_info {
    pub encoding: pa_encoding_t,
    pub plist: *mut pa_proplist,
}

impl ::std::default::Default for pa_format_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub const PA_PROP_TYPE_INT: c_int = 0;
pub const PA_PROP_TYPE_INT_RANGE: c_int = 1;
pub const PA_PROP_TYPE_INT_ARRAY: c_int = 2;
pub const PA_PROP_TYPE_STRING: c_int = 3;
pub const PA_PROP_TYPE_STRING_ARRAY: c_int = 4;
pub const PA_PROP_TYPE_INVALID: c_int = -1;
pub type pa_prop_type_t = c_int;

pub enum pa_operation {}
pub type pa_operation_notify_cb_t =
    Option<unsafe extern "C" fn(o: *mut pa_operation, userdata: *mut c_void)>;

pub enum pa_context {}
pub type pa_context_notify_cb_t =
    Option<unsafe extern "C" fn(c: *mut pa_context, userdata: *mut c_void)>;
pub type pa_context_success_cb_t =
    Option<unsafe extern "C" fn(c: *mut pa_context, success: c_int, userdata: *mut c_void)>;
pub type pa_context_event_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        p: *mut pa_proplist,
        userdata: *mut c_void,
    ),
>;

pub type pa_volume_t = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_cvolume {
    pub channels: u8,
    pub values: [pa_volume_t; 32usize],
}

impl ::std::default::Default for pa_cvolume {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub enum pa_stream {}
pub type pa_stream_success_cb_t =
    Option<unsafe extern "C" fn(s: *mut pa_stream, success: c_int, userdata: *mut c_void)>;
pub type pa_stream_request_cb_t =
    Option<unsafe extern "C" fn(p: *mut pa_stream, nbytes: usize, userdata: *mut c_void)>;
pub type pa_stream_notify_cb_t =
    Option<unsafe extern "C" fn(p: *mut pa_stream, userdata: *mut c_void)>;
pub type pa_stream_event_cb_t = Option<
    unsafe extern "C" fn(
        p: *mut pa_stream,
        name: *const c_char,
        pl: *mut pa_proplist,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_port_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub priority: u32,
    pub available: c_int,
}

impl ::std::default::Default for pa_port_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_sink_info {
    pub name: *const c_char,
    pub index: u32,
    pub description: *const c_char,
    pub sample_spec: pa_sample_spec,
    pub channel_map: pa_channel_map,
    pub owner_module: u32,
    pub volume: pa_cvolume,
    pub mute: c_int,
    pub monitor_source: u32,
    pub monitor_source_name: *const c_char,
    pub latency: pa_usec_t,
    pub driver: *const c_char,
    pub flags: pa_sink_flags_t,
    pub proplist: *mut pa_proplist,
    pub configured_latency: pa_usec_t,
    pub base_volume: pa_volume_t,
    pub state: pa_sink_state_t,
    pub n_volume_steps: u32,
    pub card: u32,
    pub n_ports: u32,
    pub ports: *mut *mut pa_port_info,
    pub active_port: *mut pa_port_info,
    pub n_formats: u8,
    pub formats: *mut *mut pa_format_info,
}

impl ::std::default::Default for pa_sink_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_sink_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_sink_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_source_info {
    pub name: *const c_char,
    pub index: u32,
    pub description: *const c_char,
    pub sample_spec: pa_sample_spec,
    pub channel_map: pa_channel_map,
    pub owner_module: u32,
    pub volume: pa_cvolume,
    pub mute: c_int,
    pub monitor_of_sink: u32,
    pub monitor_of_sink_name: *const c_char,
    pub latency: pa_usec_t,
    pub driver: *const c_char,
    pub flags: pa_source_flags_t,
    pub proplist: *mut pa_proplist,
    pub configured_latency: pa_usec_t,
    pub base_volume: pa_volume_t,
    pub state: pa_source_state_t,
    pub n_volume_steps: u32,
    pub card: u32,
    pub n_ports: u32,
    pub ports: *mut *mut pa_port_info,
    pub active_port: *mut pa_port_info,
    pub n_formats: u8,
    pub formats: *mut *mut pa_format_info,
}

impl ::std::default::Default for pa_source_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_source_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_source_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_server_info {
    pub user_name: *const c_char,
    pub host_name: *const c_char,
    pub server_version: *const c_char,
    pub server_name: *const c_char,
    pub sample_spec: pa_sample_spec,
    pub default_sink_name: *const c_char,
    pub default_source_name: *const c_char,
    pub cookie: u32,
    pub channel_map: pa_channel_map,
}

impl ::std::default::Default for pa_server_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_server_info_cb_t = Option<
    unsafe extern "C" fn(c: *mut pa_context, i: *const pa_server_info, userdata: *mut c_void),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_module_info {
    pub index: u32,
    pub name: *const c_char,
    pub argument: *const c_char,
    pub n_used: u32,
    pub auto_unload: c_int,
    pub proplist: *mut pa_proplist,
}

impl ::std::default::Default for pa_module_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_module_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_module_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;
pub type pa_context_index_cb_t =
    Option<unsafe extern "C" fn(c: *mut pa_context, idx: u32, userdata: *mut c_void)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_client_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub driver: *const c_char,
    pub proplist: *mut pa_proplist,
}

impl ::std::default::Default for pa_client_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_client_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_client_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_card_profile_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub n_sinks: u32,
    pub n_sources: u32,
    pub priority: u32,
}

impl ::std::default::Default for pa_card_profile_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_card_profile_info2 {
    pub name: *const c_char,
    pub description: *const c_char,
    pub n_sinks: u32,
    pub n_sources: u32,
    pub priority: u32,
    pub available: c_int,
}

impl ::std::default::Default for pa_card_profile_info2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_card_port_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub priority: u32,
    pub available: c_int,
    pub direction: c_int,
    pub n_profiles: u32,
    pub profiles: *mut *mut pa_card_profile_info,
    pub proplist: *mut pa_proplist,
    pub latency_offset: i64,
    pub profiles2: *mut *mut pa_card_profile_info2,
}

impl ::std::default::Default for pa_card_port_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_card_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub driver: *const c_char,
    pub n_profiles: u32,
    pub profiles: *mut pa_card_profile_info,
    pub active_profile: *mut pa_card_profile_info,
    pub proplist: *mut pa_proplist,
    pub n_ports: u32,
    pub ports: *mut *mut pa_card_port_info,
    pub profiles2: *mut *mut pa_card_profile_info2,
    pub active_profile2: *mut pa_card_profile_info2,
}

impl ::std::default::Default for pa_card_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_card_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_card_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_sink_input_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub client: u32,
    pub sink: u32,
    pub sample_spec: pa_sample_spec,
    pub channel_map: pa_channel_map,
    pub volume: pa_cvolume,
    pub buffer_usec: pa_usec_t,
    pub sink_usec: pa_usec_t,
    pub resample_method: *const c_char,
    pub driver: *const c_char,
    pub mute: c_int,
    pub proplist: *mut pa_proplist,
    pub corked: c_int,
    pub has_volume: c_int,
    pub volume_writable: c_int,
    pub format: *mut pa_format_info,
}

impl ::std::default::Default for pa_sink_input_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_sink_input_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_sink_input_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_source_output_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub client: u32,
    pub source: u32,
    pub sample_spec: pa_sample_spec,
    pub channel_map: pa_channel_map,
    pub buffer_usec: pa_usec_t,
    pub source_usec: pa_usec_t,
    pub resample_method: *const c_char,
    pub driver: *const c_char,
    pub proplist: *mut pa_proplist,
    pub corked: c_int,
    pub volume: pa_cvolume,
    pub mute: c_int,
    pub has_volume: c_int,
    pub volume_writable: c_int,
    pub format: *mut pa_format_info,
}

impl ::std::default::Default for pa_source_output_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_source_output_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_source_output_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_stat_info {
    pub memblock_total: u32,
    pub memblock_total_size: u32,
    pub memblock_allocated: u32,
    pub memblock_allocated_size: u32,
    pub scache_size: u32,
}

impl ::std::default::Default for pa_stat_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_stat_info_cb_t =
    Option<unsafe extern "C" fn(c: *mut pa_context, i: *const pa_stat_info, userdata: *mut c_void)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_sample_info {
    pub index: u32,
    pub name: *const c_char,
    pub volume: pa_cvolume,
    pub sample_spec: pa_sample_spec,
    pub channel_map: pa_channel_map,
    pub duration: pa_usec_t,
    pub bytes: u32,
    pub lazy: c_int,
    pub filename: *const c_char,
    pub proplist: *mut pa_proplist,
}

impl ::std::default::Default for pa_sample_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_sample_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_sample_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;

pub const PA_AUTOLOAD_SINK: c_int = 0;
pub const PA_AUTOLOAD_SOURCE: c_int = 1;
pub type pa_autoload_type_t = c_int;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct pa_autoload_info {
    pub index: u32,
    pub name: *const c_char,
    pub _type: pa_autoload_type_t,
    pub module: *const c_char,
    pub argument: *const c_char,
}

impl ::std::default::Default for pa_autoload_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type pa_autoload_info_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        i: *const pa_autoload_info,
        eol: c_int,
        userdata: *mut c_void,
    ),
>;
pub type pa_context_subscribe_cb_t = Option<
    unsafe extern "C" fn(
        c: *mut pa_context,
        t: pa_subscription_event_type_t,
        idx: u32,
        userdata: *mut c_void,
    ),
>;
pub type pa_context_play_sample_cb_t =
    Option<unsafe extern "C" fn(c: *mut pa_context, idx: u32, userdata: *mut c_void)>;

pub enum pa_threaded_mainloop {}
pub enum pollfd {}
pub enum pa_mainloop {}

pub type pa_poll_func = Option<
    unsafe extern "C" fn(
        ufds: *mut pollfd,
        nfds: c_ulong,
        timeout: c_int,
        userdata: *mut c_void,
    ) -> c_int,
>;
pub enum pa_signal_event {}

pub type pa_signal_cb_t = Option<
    unsafe extern "C" fn(
        api: *mut pa_mainloop_api,
        e: *mut pa_signal_event,
        sig: c_int,
        userdata: *mut c_void,
    ),
>;
pub type pa_signal_destroy_cb_t = Option<
    unsafe extern "C" fn(api: *mut pa_mainloop_api, e: *mut pa_signal_event, userdata: *mut c_void),
>;
