use std::os::raw::c_int;
use std::ptr::null_mut;
use crate::bindings::*;
use crate::common::CType;
use crate::libccxr_exports::time::write_back_to_common_timing_ctx;
use crate::xds::common_constants::*;
use lib_ccxr::common::options::CommonTimingCtx;
use lib_ccxr::decoder_xds::exit_codes::*;
use lib_ccxr::time::TimingContext;
use crate::avc::FromCType;

const NUM_BYTES_PER_PACKET: usize = 35; // Class + type (repeated for convenience) + data + zero
const NUM_XDS_BUFFERS: usize = 9; // CEA recommends no more than one level of interleaving. Play it safe

/// XdsBuffer : C<->Rust conversion guide
/// Always map int → i32, unsigned → u32, char → u8.
///
/// struct xds_buffer
// {
// 	unsigned in_use;
// 	int xds_class;
// 	int xds_type;
// 	unsigned char bytes[NUM_BYTES_PER_PACKET]; // Class + type (repeated for convenience) + data + zero
// 	unsigned char used_bytes;
// };
/// to below
#[repr(C)]
#[derive(Copy, Clone)]
struct XdsBuffer {
    in_use: u32,
    xds_class: Option<XdsClass>,
    xds_type: Option<XdsType>,
    bytes: [u8; NUM_BYTES_PER_PACKET], // Class + type (repeated for convenience) + data + zero
    used_bytes: u8,
}

impl Default for XdsBuffer {
    fn default() -> Self {
        XdsBuffer {
            in_use: 0,
            xds_class: None,
            xds_type: None,
            bytes: [0; NUM_BYTES_PER_PACKET],
            used_bytes: 0,
        }
    }
}

/// CcxDecodersXdsContext : C<->Rust conversion guide
///
// typedef struct ccx_decoders_xds_context
// {
// 	// Program Identification Number (Start Time) for current program
// 	int current_xds_min;
// 	int current_xds_hour;
// 	int current_xds_date;
// 	int current_xds_month;
// 	int current_program_type_reported; // No.
// 	int xds_start_time_shown;
// 	int xds_program_length_shown;
// 	char xds_program_description[8][33];

// 	char current_xds_network_name[33];
// 	char current_xds_program_name[33];
// 	char current_xds_call_letters[7];
// 	char current_xds_program_type[33];

// 	struct xds_buffer xds_buffers[NUM_XDS_BUFFERS];
// 	int cur_xds_buffer_idx;
// 	int cur_xds_packet_class;
// 	unsigned char *cur_xds_payload;
// 	int cur_xds_payload_length;
// 	int cur_xds_packet_type;
// 	struct ccx_common_timing_ctx *timing;

// 	unsigned current_ar_start;
// 	unsigned current_ar_end;

// 	int xds_write_to_file; // Set to 1 if XDS data is to be written to file

// } ccx_decoders_xds_context_t;
/// to below
#[repr(C)]
pub struct CcxDecodersXdsContext<'a> {
    // Program Identification Number (Start Time) for current program
    pub current_xds_min: i32,
    pub current_xds_hour: i32,
    pub current_xds_date: i32,
    pub current_xds_month: i32,
    pub current_program_type_reported: i32, // No.
    pub xds_start_time_shown: i32,
    pub xds_program_length_shown: i32,
    pub xds_program_description: [[i8; 33]; 8],
    pub current_xds_network_name: [i8; 33],
    pub current_xds_program_name: [i8; 33],
    pub current_xds_call_letters: [i8; 7],
    pub current_xds_program_type: [i8; 33],

    pub xds_buffers: [XdsBuffer; NUM_XDS_BUFFERS],
    pub cur_xds_buffer_idx: i32,
    pub cur_xds_packet_class: i32,
    pub cur_xds_payload: *mut u8,
    pub cur_xds_payload_length: i32,
    pub cur_xds_packet_type: i32,
    pub timing: Option<&'a mut TimingContext>,
    pub current_ar_start: u32,
    pub current_ar_end: u32,
    pub xds_write_to_file: bool,
}
pub unsafe fn copy_xds_context_from_rust_to_c(
    bitstream_ptr: *mut ccx_decoders_xds_context,
    rust_ctx: &CcxDecodersXdsContext<'_>,
) {
    if bitstream_ptr.is_null() {
        return;
    }
    let output = ccx_decoders_xds_context {
        current_xds_min: rust_ctx.current_xds_min,
        current_xds_hour: rust_ctx.current_xds_hour,
        current_xds_date: rust_ctx.current_xds_date,
        current_xds_month: rust_ctx.current_xds_month,
        current_program_type_reported: rust_ctx.current_program_type_reported,
        xds_start_time_shown: rust_ctx.xds_start_time_shown,
        xds_program_length_shown: rust_ctx.xds_program_length_shown,
        xds_program_description: rust_ctx.xds_program_description,
        current_xds_network_name: rust_ctx.current_xds_network_name,
        current_xds_program_name: rust_ctx.current_xds_program_name,
        current_xds_call_letters: rust_ctx.current_xds_call_letters,
        current_xds_program_type: rust_ctx.current_xds_program_type,
        xds_buffers: rust_ctx
            .xds_buffers
            .map(|buf| buf.to_ctype()),
        cur_xds_buffer_idx: rust_ctx.cur_xds_buffer_idx,
        cur_xds_packet_class: rust_ctx.cur_xds_packet_class,
        cur_xds_payload: rust_ctx.cur_xds_payload,
        cur_xds_payload_length: rust_ctx.cur_xds_payload_length,
        cur_xds_packet_type: rust_ctx.cur_xds_packet_type,
        timing: null_mut(),
        current_ar_start: rust_ctx.current_ar_start,
        current_ar_end: rust_ctx.current_ar_end,
        xds_write_to_file: if rust_ctx.xds_write_to_file { 1 } else { 0 },
    };
    std::ptr::write(bitstream_ptr, output);
    if let Some(ref timing_ctx) = rust_ctx.timing {
        write_back_to_common_timing_ctx((*bitstream_ptr).timing, timing_ctx);
    }
}
impl FromCType<ccx_decoders_xds_context> for CcxDecodersXdsContext<'_> {
    unsafe fn from_ctype(c_value: ccx_decoders_xds_context) -> Option<Self> {
        let mut xds_buffers = [XdsBuffer::default(); NUM_XDS_BUFFERS];

        // Convert each xds_buffer from C to Rust
        for (i, c_buffer) in c_value.xds_buffers.iter().enumerate() {
            if let Some(rust_buffer) = XdsBuffer::from_ctype(*c_buffer) {
                xds_buffers[i] = rust_buffer;
            }
        }

        Some(CcxDecodersXdsContext {
            current_xds_min: c_value.current_xds_min,
            current_xds_hour: c_value.current_xds_hour,
            current_xds_date: c_value.current_xds_date,
            current_xds_month: c_value.current_xds_month,
            current_program_type_reported: c_value.current_program_type_reported,
            xds_start_time_shown: c_value.xds_start_time_shown,
            xds_program_length_shown: c_value.xds_program_length_shown,
            xds_program_description: c_value.xds_program_description,
            current_xds_network_name: c_value.current_xds_network_name,
            current_xds_program_name: c_value.current_xds_program_name,
            current_xds_call_letters: c_value.current_xds_call_letters,
            current_xds_program_type: c_value.current_xds_program_type,
            xds_buffers,
            cur_xds_buffer_idx: c_value.cur_xds_buffer_idx,
            cur_xds_packet_class: c_value.cur_xds_packet_class,
            cur_xds_payload: c_value.cur_xds_payload,
            cur_xds_payload_length: c_value.cur_xds_payload_length,
            cur_xds_packet_type: c_value.cur_xds_packet_type,
            timing: None, // Cannot directly convert raw pointer to reference - needs to be handled separately
            current_ar_start: c_value.current_ar_start,
            current_ar_end: c_value.current_ar_end,
            xds_write_to_file: c_value.xds_write_to_file != 0,
        })
    }
}
impl CType<xds_buffer> for XdsBuffer {
    unsafe fn to_ctype(&self) -> xds_buffer {
        xds_buffer {
            in_use: self.in_use,
            xds_class: self.xds_class.map(|c| c.to_c_int()).unwrap_or(-1),
            xds_type: self.xds_type.map(|t| t.to_c_int()).unwrap_or(-1),
            bytes: self.bytes,
            used_bytes: self.used_bytes,
        }
    }
}

impl XdsClass {
    fn from_c_int(value: c_int) -> Option<Self> {
        match value {
            0 => Some(XdsClass::Current),
            1 => Some(XdsClass::Future),
            2 => Some(XdsClass::Channel),
            3 => Some(XdsClass::Misc),
            4 => Some(XdsClass::Public),
            5 => Some(XdsClass::Reserved),
            6 => Some(XdsClass::Private),
            7 => Some(XdsClass::End),
            0x40 => Some(XdsClass::OutOfBand),
            _ => None,
        }
    }

    fn to_c_int(&self) -> c_int {
        match self {
            XdsClass::Current => 0,
            XdsClass::Future => 1,
            XdsClass::Channel => 2,
            XdsClass::Misc => 3,
            XdsClass::Public => 4,
            XdsClass::Reserved => 5,
            XdsClass::Private => 6,
            XdsClass::End => 7,
            XdsClass::OutOfBand => 0x40,
        }
    }
}

impl XdsType {
    fn from_c_int(class: Option<XdsClass>, type_value: c_int) -> Option<Self> {
        match class? {
            XdsClass::Channel => {
                match type_value {
                    1 => Some(XdsType::Channel(XdsChannelType::NetworkName)),
                    2 => Some(XdsType::Channel(XdsChannelType::CallLettersAndChannel)),
                    4 => Some(XdsType::Channel(XdsChannelType::Tsid)),
                    _ => None,
                }
            }
            XdsClass::Misc => {
                match type_value {
                    1 => Some(XdsType::Misc(XdsMiscType::TimeOfDay)),
                    4 => Some(XdsType::Misc(XdsMiscType::LocalTimeZone)),
                    0x40 => Some(XdsType::Misc(XdsMiscType::OutOfBandChannelNumber)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn to_c_int(&self) -> c_int {
        match self {
            XdsType::Channel(t) => match t {
                XdsChannelType::NetworkName => 1,
                XdsChannelType::CallLettersAndChannel => 2,
                XdsChannelType::Tsid => 4,
            },
            XdsType::Misc(t) => match t {
                XdsMiscType::TimeOfDay => 1,
                XdsMiscType::LocalTimeZone => 4,
                XdsMiscType::OutOfBandChannelNumber => 0x40,
            },
        }
    }
}
impl FromCType<xds_buffer> for XdsBuffer {
    unsafe fn from_ctype(c_value: xds_buffer) -> Option<Self> {
        let xds_class = if c_value.xds_class == -1 {
            None
        } else {
            XdsClass::from_c_int(c_value.xds_class)
        };

        let xds_type = if c_value.xds_type == -1 {
            None
        } else {
            XdsType::from_c_int(xds_class, c_value.xds_type)
        };

        Some(XdsBuffer {
            in_use: c_value.in_use,
            xds_class,
            xds_type,
            bytes: c_value.bytes,
            used_bytes: c_value.used_bytes,
        })
    }
}

impl Default for CcxDecodersXdsContext<'_> {
    fn default() -> Self {
        CcxDecodersXdsContext {
            current_xds_min: 0,
            current_xds_hour: 0,
            current_xds_date: 0,
            current_xds_month: 0,
            current_program_type_reported: 0,
            xds_start_time_shown: 0,
            xds_program_length_shown: 0,
            xds_program_description: [[0; 33]; 8],
            current_xds_network_name: [0; 33],
            current_xds_program_name: [0; 33],
            current_xds_call_letters: [0; 7],
            current_xds_program_type: [0; 33],
            xds_buffers: [XdsBuffer::default(); NUM_XDS_BUFFERS],
            cur_xds_buffer_idx: 0,
            cur_xds_packet_class: 0,
            cur_xds_payload: std::ptr::null_mut(),
            cur_xds_payload_length: 0,
            cur_xds_packet_type: 0,
            timing: None,
            current_ar_start: 0,
            current_ar_end: 0,
            xds_write_to_file: false,
        }
    }
}

impl<'a> CcxDecodersXdsContext<'a> {
    pub(crate) fn new(timing: &'a mut TimingContext, xds_write_to_file: i32) -> Box<Self> {
        Box::new(Self {
            current_xds_min: -1,
            current_xds_hour: -1,
            current_xds_date: -1,
            current_xds_month: -1,
            current_program_type_reported: 0, //No
            xds_start_time_shown: 0,
            xds_program_length_shown: 0,

            xds_program_description: [[0; 33]; 8],

            current_xds_network_name: [0; 33],
            current_xds_program_name: [0; 33],
            current_xds_call_letters: [0; 7],
            current_xds_program_type: [0; 33],

            xds_buffers: [XdsBuffer::default(); NUM_XDS_BUFFERS],

            cur_xds_buffer_idx: -1,
            cur_xds_packet_class: -1,
            cur_xds_payload: std::ptr::null_mut(), // unsafe raw pointer
            cur_xds_payload_length: 0,
            cur_xds_packet_type: 0,
            timing: Some(timing),

            current_ar_start: u32::MAX, // not set here
            current_ar_end: u32::MAX,   // not set here

            xds_write_to_file: xds_write_to_file != 0,
        })
    }
}

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct cc_subtitle {
//     #[doc = " A generic data which contain data according to decoder\n @warn decoder cant output multiple types of data"]
//     pub data: *mut ::std::os::raw::c_void,
//     pub datatype: subdatatype,
//     #[doc = " number of data"]
//     pub nb_data: ::std::os::raw::c_uint,
//     #[doc = "  type of subtitle"]
//     pub type_: subtype,
//     #[doc = " Encoding type of Text, must be ignored in case of subtype as bitmap or cc_screen"]
//     pub enc_type: ccx_encoding_type,
//     pub start_time: LLONG,
//     pub end_time: LLONG,
//     pub flags: ::std::os::raw::c_int,
//     pub lang_index: ::std::os::raw::c_int,
//     #[doc = " flag to tell that decoder has given output"]
//     pub got_output: ::std::os::raw::c_int,
//     pub mode: [::std::os::raw::c_char; 5usize],
//     pub info: [::std::os::raw::c_char; 4usize],
//     #[doc = " Used for DVB end time in ms"]
//     pub time_out: ::std::os::raw::c_int,
//     pub next: *mut cc_subtitle,
//     pub prev: *mut cc_subtitle,
// }

// pub struct eia608_screen {
//     #[doc = " format of data inside this structure"]
//     pub format: ccx_eia608_format,
//     pub characters: [[::std::os::raw::c_uchar; 33usize]; 15usize],
//     pub colors: [[ccx_decoder_608_color_code; 33usize]; 15usize],
//     pub fonts: [[font_bits; 33usize]; 15usize],
//     pub row_used: [::std::os::raw::c_int; 15usize],
//     pub empty: ::std::os::raw::c_int,
//     #[doc = " start time of this CC buffer"]
//     pub start_time: LLONG,
//     #[doc = " end time of this CC buffer"]
//     pub end_time: LLONG,
//     pub mode: cc_modes,
//     pub channel: ::std::os::raw::c_int,
//     pub my_field: ::std::os::raw::c_int,
//     #[doc = " XDS string"]
//     pub xds_str: *mut ::std::os::raw::c_char,
//     #[doc = " length of XDS string"]
//     pub xds_len: usize,
//     #[doc = " Class of XDS string"]
//     pub cur_xds_packet_class: ::std::os::raw::c_int,
// }
