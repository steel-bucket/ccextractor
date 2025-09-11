use lib_ccxr::common::options::CommonTimingCtx;

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
    xds_class: i32,
    xds_type: i32,
    bytes: [u8; NUM_BYTES_PER_PACKET], // Class + type (repeated for convenience) + data + zero
    used_bytes: u8,
}

impl Default for XdsBuffer {
    fn default() -> Self {
        XdsBuffer {
            in_use: 0,
            xds_class: -1,
            xds_type: -1,
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
struct CcxDecodersXdsContext {
    // Program Identification Number (Start Time) for current program
    current_xds_min: i32,
    current_xds_hour: i32,
    current_xds_date: i32,
    current_xds_month: i32,
    current_program_type_reported: i32, // No.
    xds_start_time_shown: i32,
    xds_program_length_shown: i32,
    xds_program_description: [[u8; 33]; 8],

    current_xds_network_name: [u8; 33],
    current_xds_program_name: [u8; 33],
    current_xds_call_letters: [u8; 7],
    current_xds_program_type: [u8; 33],

    xds_buffers: [XdsBuffer; NUM_XDS_BUFFERS],
    cur_xds_buffer_idx: i32,
    cur_xds_packet_class: i32,
    cur_xds_payload: *mut u8, // unsafe raw pointer
    cur_xds_payload_length: i32,
    cur_xds_packet_type: i32,
    timing: *mut CommonTimingCtx, // unsafe raw pointer

    current_ar_start: u32,
    current_ar_end: u32,

    xds_write_to_file: i32, // Set to 1 if XDS data is to be written to file
}

impl Default for CcxDecodersXdsContext {
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
            cur_xds_payload: std::ptr::null_mut(), // unsafe raw pointer
            cur_xds_payload_length: 0,
            cur_xds_packet_type: 0,
            timing: std::ptr::null_mut(), // unsafe raw pointer
            current_ar_start: 0,
            current_ar_end: 0,
            xds_write_to_file: 0,
        }
    }
}

impl CcxDecodersXdsContext {
    fn new(timing: *mut CommonTimingCtx, xds_write_to_file: i32) -> Box<Self> {
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
            timing: timing,

            current_ar_start: u32::MAX, // not set here
            current_ar_end: u32::MAX,   // not set here

            xds_write_to_file: xds_write_to_file,
        })
    }
}
