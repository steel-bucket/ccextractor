
static XDS_CLASSES: [&str; 8] = [
    "Current",
    "Future",
    "Channel",
    "Miscellaneous",
    "Public service",
    "Reserved",
    "Private data",
    "End",
];

static XDS_PROGRAM_TYPES: [&str; 96] = [
    "Education",
    "Entertainment",
    "Movie",
    "News",
    "Religious",
    "Sports",
    "Other",
    "Action",
    "Advertisement",
    "Animated",
    "Anthology",
    "Automobile",
    "Awards",
    "Baseball",
    "Basketball",
    "Bulletin",
    "Business",
    "Classical",
    "College",
    "Combat",
    "Comedy",
    "Commentary",
    "Concert",
    "Consumer",
    "Contemporary",
    "Crime",
    "Dance",
    "Documentary",
    "Drama",
    "Elementary",
    "Erotica",
    "Exercise",
    "Fantasy",
    "Farm",
    "Fashion",
    "Fiction",
    "Food",
    "Football",
    "Foreign",
    "Fund-Raiser",
    "Game/Quiz",
    "Garden",
    "Golf",
    "Government",
    "Health",
    "High_School",
    "History",
    "Hobby",
    "Hockey",
    "Home",
    "Horror",
    "Information",
    "Instruction",
    "International",
    "Interview",
    "Language",
    "Legal",
    "Live",
    "Local",
    "Math",
    "Medical",
    "Meeting",
    "Military",
    "Mini-Series",
    "Music",
    "Mystery",
    "National",
    "Nature",
    "Police",
    "Politics",
    "Premiere",
    "Pre-Recorded",
    "Product",
    "Professional",
    "Public",
    "Racing",
    "Reading",
    "Repair",
    "Repeat",
    "Review",
    "Romance",
    "Science",
    "Series",
    "Service",
    "Shopping",
    "Soap_Opera",
    "Special",
    "Suspense",
    "Talk",
    "Technical",
    "Tennis",
    "Travel",
    "Variety",
    "Video",
    "Weather",
    "Western",
];

const XDS_CLASS_CURRENT: u8 = 0;
const XDS_CLASS_FUTURE: u8 = 1;
const XDS_CLASS_CHANNEL: u8 = 2;
const XDS_CLASS_MISC: u8 = 3;
const XDS_CLASS_PUBLIC: u8 = 4;
const XDS_CLASS_RESERVED: u8 = 5;
const XDS_CLASS_PRIVATE: u8 = 6;
const XDS_CLASS_END: u8 = 7;

const XDS_CLASS_OUT_OF_BAND: u8 = 0x40; // Not a real class, a marker for packets for out-of-band data.

const XDS_TYPE_PIN_START_TIME: u8 = 1;
const XDS_TYPE_LENGH_AND_CURRENT_TIME: u8 = 2;
const XDS_TYPE_PROGRAM_NAME: u8 = 3;
const XDS_TYPE_PROGRAM_TYPE: u8 = 4;
const XDS_TYPE_CONTENT_ADVISORY: u8 = 5;
const XDS_TYPE_AUDIO_SERVICES: u8 = 6;
const XDS_TYPE_CGMS: u8 = 8; // Copy Generation Management System
const XDS_TYPE_ASPECT_RATIO_INFO: u8 = 9; // Appears in CEA-608-B but in E it's been removed as is "reserved"

enum XDS_TYPE_PROGRAM_DESC {
    XDS_TYPE_PROGRAM_DESC_1 = 0x10,
    XDS_TYPE_PROGRAM_DESC_2 = 0x11,
    XDS_TYPE_PROGRAM_DESC_3 = 0x12,
    XDS_TYPE_PROGRAM_DESC_4 = 0x13,
    XDS_TYPE_PROGRAM_DESC_5 = 0x14,
    XDS_TYPE_PROGRAM_DESC_6 = 0x15,
    XDS_TYPE_PROGRAM_DESC_7 = 0x16,
    XDS_TYPE_PROGRAM_DESC_8 = 0x17,
}

// Types for the class channel
const XDS_TYPE_NETWORK_NAME: u8 = 1;
const XDS_TYPE_CALL_LETTERS_AND_CHANNEL: u8 = 2;
const XDS_TYPE_TSID: u8 = 4; // Transmission Signal Identifier

// Types for miscellaneous packets
const XDS_TYPE_TIME_OF_DAY: u8 = 1;
const XDS_TYPE_LOCAL_TIME_ZONE: u8 = 4;
const XDS_TYPE_OUT_OF_BAND_CHANNEL_NUMBER: u8 = 0x40;

