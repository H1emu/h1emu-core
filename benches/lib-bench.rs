use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/rc4.rs"]
mod rc4;
use rc4::RC4;

#[path = "../src/crc.rs"]
mod crc;
use crc::*;

#[path = "../src/jenkins.rs"]
mod jenkins;
use jenkins::*;

#[path = "../src/utils.rs"]
mod utils;
use utils::*;

#[path = "../src/soeprotocol.rs"]
mod soeprotocol;
use soeprotocol::Soeprotocol;

#[path = "../src/soeprotocol/soeprotocol_functions.rs"]
mod soeprotocol_functions;
use soeprotocol_functions::*;

fn soeprotocol_utils_benchmarks(c: &mut Criterion) {
    let data_to_pack: Vec<u8> = [
        2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0,
    ]
    .to_vec();
    let mut wtr = vec![];
    let mut data_packet = DataPacket {
        data: data_to_pack,
        sequence: 0,
    };

    c.bench_function("write_packet_data_crc", |b| {
        b.iter(|| {
            write_packet_data(
                black_box(&mut wtr.to_owned()),
                black_box(&mut data_packet),
                black_box(0),
                black_box(true),
            )
        })
    });
    let data_to_pack: Vec<u8> = [
        2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0,
    ]
    .to_vec();
    let mut wtr = vec![];
    let mut data_packet = DataPacket {
        data: data_to_pack,
        sequence: 0,
    };
    c.bench_function("write_packet_data", |b| {
        b.iter(|| {
            write_packet_data(
                black_box(&mut wtr.to_owned()),
                black_box(&mut data_packet),
                black_box(0),
                black_box(false),
            )
        })
    });
}

fn soeprotocol_parse_benchmarks(c: &mut Criterion) {
    let mut soeprotocol_class = Soeprotocol::initialize(false,0);
    // define data used in benchmarks
    let session_request_to_parse: [u8; 25] = [
        0, 1, 0, 0, 0, 3, 60, 23, 140, 99, 0, 0, 2, 0, 76, 111, 103, 105, 110, 85, 100, 112, 95,
        57, 0,
    ];

    let session_reply_to_parse: [u8; 21] = [
        0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3,
    ];

    let ping_to_parse: [u8; 2] = [0, 6];

    let outoforder_to_parse: [u8; 4] = [0, 17, 0, 1];

    let ack_to_parse: [u8; 4] = [0, 21, 0, 1];

    let outoforder_to_parse_crc: [u8; 6] = [0, 17, 0, 1, 38, 184];

    let ack_to_parse_crc: [u8; 6] = [0, 21, 0, 1, 142, 100];

    let ack_to_parse_crc_fail: [u8; 6] = [0, 21, 0, 1, 142, 101];

    let multi_to_parse: [u8; 75] = [
        0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71, 228,
        114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102, 204, 158,
        233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138, 145, 21, 221,
        187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
    ];

    let multi_to_parse_crc: [u8; 77] = [
        0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71, 228,
        114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102, 204, 158,
        233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138, 145, 21, 221,
        187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246, 10, 27,
    ];

    let data_to_parse: [u8; 43] = [
        0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2, 113,
        179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132, 117, 146,
        204,
    ];
    let data_to_parse_crc: [u8; 45] = [
        0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2, 113,
        179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132, 117, 146,
        204, 94, 60,
    ];

    let data_fragment_to_parse_crc: [u8; 257] = [
        0, 13, 0, 2, 208, 127, 31, 117, 87, 54, 201, 180, 188, 226, 247, 253, 136, 66, 78, 125,
        224, 112, 23, 87, 147, 110, 18, 68, 183, 87, 20, 3, 65, 116, 82, 111, 93, 219, 229, 20, 61,
        238, 143, 63, 8, 137, 8, 196, 128, 89, 59, 4, 198, 191, 207, 141, 23, 164, 242, 77, 176,
        206, 49, 45, 207, 210, 17, 33, 75, 177, 157, 242, 169, 37, 60, 87, 245, 58, 2, 130, 102,
        146, 227, 66, 193, 153, 155, 105, 230, 203, 120, 114, 160, 223, 229, 190, 129, 106, 19, 25,
        8, 52, 55, 8, 100, 68, 109, 228, 178, 186, 148, 108, 138, 242, 136, 66, 219, 25, 73, 129,
        110, 31, 121, 32, 246, 86, 156, 212, 85, 217, 213, 119, 165, 140, 83, 95, 6, 183, 184, 251,
        73, 102, 221, 156, 240, 204, 50, 217, 217, 13, 218, 2, 19, 44, 143, 73, 168, 109, 67, 176,
        129, 225, 187, 171, 12, 146, 21, 66, 252, 150, 143, 142, 46, 39, 72, 12, 22, 222, 7, 29,
        63, 201, 227, 251, 9, 28, 0, 100, 84, 153, 84, 212, 163, 78, 135, 33, 66, 20, 195, 223, 62,
        214, 32, 59, 6, 187, 222, 99, 29, 34, 87, 81, 61, 63, 174, 255, 1, 85, 241, 6, 10, 152,
        237, 52, 51, 126, 149, 218, 125, 232, 199, 40, 113, 139, 187, 43, 232, 209, 167, 226, 91,
        236, 212, 165, 117, 19, 118, 110, 18, 0, 26, 152, 33, 115, 61, 208, 21,
    ];

    let data_fragment_to_parse: [u8; 255] = [
        0, 13, 0, 2, 208, 127, 31, 117, 87, 54, 201, 180, 188, 226, 247, 253, 136, 66, 78, 125,
        224, 112, 23, 87, 147, 110, 18, 68, 183, 87, 20, 3, 65, 116, 82, 111, 93, 219, 229, 20, 61,
        238, 143, 63, 8, 137, 8, 196, 128, 89, 59, 4, 198, 191, 207, 141, 23, 164, 242, 77, 176,
        206, 49, 45, 207, 210, 17, 33, 75, 177, 157, 242, 169, 37, 60, 87, 245, 58, 2, 130, 102,
        146, 227, 66, 193, 153, 155, 105, 230, 203, 120, 114, 160, 223, 229, 190, 129, 106, 19, 25,
        8, 52, 55, 8, 100, 68, 109, 228, 178, 186, 148, 108, 138, 242, 136, 66, 219, 25, 73, 129,
        110, 31, 121, 32, 246, 86, 156, 212, 85, 217, 213, 119, 165, 140, 83, 95, 6, 183, 184, 251,
        73, 102, 221, 156, 240, 204, 50, 217, 217, 13, 218, 2, 19, 44, 143, 73, 168, 109, 67, 176,
        129, 225, 187, 171, 12, 146, 21, 66, 252, 150, 143, 142, 46, 39, 72, 12, 22, 222, 7, 29,
        63, 201, 227, 251, 9, 28, 0, 100, 84, 153, 84, 212, 163, 78, 135, 33, 66, 20, 195, 223, 62,
        214, 32, 59, 6, 187, 222, 99, 29, 34, 87, 81, 61, 63, 174, 255, 1, 85, 241, 6, 10, 152,
        237, 52, 51, 126, 149, 218, 125, 232, 199, 40, 113, 139, 187, 43, 232, 209, 167, 226, 91,
        236, 212, 165, 117, 19, 118, 110, 18, 0, 26, 152, 33, 115, 61,
    ];

    // without crc
    c.bench_function("session_request_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(session_request_to_parse.to_vec())))
    });
    c.bench_function("session_reply_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(session_reply_to_parse.to_vec())))
    });
    c.bench_function("ping_to_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(ping_to_parse.to_vec())))
    });
    c.bench_function("outoforder_to_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(outoforder_to_parse.to_vec())))
    });
    c.bench_function("ack_to_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(ack_to_parse.to_vec())))
    });
    c.bench_function("multi_to_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(multi_to_parse.to_vec())))
    });
    c.bench_function("data_to_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(data_to_parse.to_vec())))
    });
    c.bench_function("data_fragment_to_parse", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(data_fragment_to_parse.to_vec())))
    });

    // with
    soeprotocol_class.enable_crc();
    c.bench_function("outoforder_to_parse_crc", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(outoforder_to_parse_crc.to_vec())))
    });
    c.bench_function("ack_to_parse_crc", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(ack_to_parse_crc.to_vec())))
    });
    c.bench_function("ack_to_parse_crc_fail", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(ack_to_parse_crc_fail.to_vec())))
    });
    c.bench_function("multi_to_parse_crc", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(multi_to_parse_crc.to_vec())))
    });
    c.bench_function("data_to_parse_crc", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(data_to_parse_crc.to_vec())))
    });
    c.bench_function("data_fragment_to_parse_crc", |b| {
        b.iter(|| soeprotocol_class.parse(black_box(data_fragment_to_parse_crc.to_vec())))
    });
}

fn soeprotocol_pack_benchmarks(c: &mut Criterion) {
    let mut soeprotocol_class = Soeprotocol::initialize(false,0);
    let session_request_to_pack =
        r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#
            .to_string();
    let session_reply_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
    let ping_to_pack: String = r#"{"name":"Ping"}"#.to_owned();
    let outoforder_to_pack: String = r#"{"name":"OutOfOrder","sequence":1}"#.to_owned();
    let ack_to_pack: String = r#"{"name":"Ack","sequence":1}"#.to_owned();
    let multi_to_pack:String = r#"{"sub_packets":[{"name":"Ack","channel":0,"sequence":206},{"name":"Data","channel":0,"sequence":1,"crc":0,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#.to_owned();
    let data_to_pack =
        r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
    let data_fragment_to_pack =
        r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();

    // without crc

    c.bench_function("session_request_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "SessionRequest".to_owned(),
                black_box(session_request_to_pack.to_string()),
            )
        })
    });
    c.bench_function("session_reply_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "SessionReply".to_owned(),
                black_box(session_reply_to_pack.to_string()),
            )
        })
    });
    c.bench_function("ping_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "Ping".to_owned(),
                black_box(ping_to_pack.to_string()),
            )
        })
    });
    c.bench_function("outoforder_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "OutOfOrder".to_owned(),
                black_box(outoforder_to_pack.to_string()),
            )
        })
    });
    c.bench_function("ack_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "Ack".to_owned(),
                black_box(ack_to_pack.to_string()),
            )
        })
    });
    c.bench_function("multi_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "MultiPacket".to_owned(),
                black_box(multi_to_pack.to_string()),
            )
        })
    });
    c.bench_function("data_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "Data".to_owned(),
                black_box(data_to_pack.to_string()),
            )
        })
    });
    c.bench_function("data_fragment_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "DataFragment".to_owned(),
                black_box(data_fragment_to_pack.to_string()),
            )
        })
    });

    // with crc
    soeprotocol_class.enable_crc();
    c.bench_function("session_request_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "SessionRequest".to_owned(),
                black_box(session_request_to_pack.to_string()),
            )
        })
    });
    c.bench_function("session_reply_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "SessionReply".to_owned(),
                black_box(session_reply_to_pack.to_string()),
            )
        })
    });
    c.bench_function("ping_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "Ping".to_owned(),
                black_box(ping_to_pack.to_string()),
            )
        })
    });
    c.bench_function("outoforder_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "OutOfOrder".to_owned(),
                black_box(outoforder_to_pack.to_string()),
            )
        })
    });
    c.bench_function("ack_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "Ack".to_owned(),
                black_box(ack_to_pack.to_string()),
            )
        })
    });
    c.bench_function("multi_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "MultiPacket".to_owned(),
                black_box(multi_to_pack.to_string()),
            )
        })
    });
    c.bench_function("data_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "Data".to_owned(),
                black_box(data_to_pack.to_string()),
            )
        })
    });
    c.bench_function("data_fragment_to_pack_crc", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "DataFragment".to_owned(),
                black_box(data_fragment_to_pack.to_string()),
            )
        })
    });
}

fn crc_legacy_benchmark(c: &mut Criterion) {
    let data: [u8; 24] = [
        0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26, 83,
        24, 212,
    ];
    c.bench_function("append_crc_legacy", |b| {
        b.iter(|| append_crc_legacy(black_box(&data), black_box(0)))
    });
    drop(data);
    let data: [u8; 5] = [0, 21, 0, 0, 2];
    c.bench_function("crc32_legacy", |b| {
        b.iter(|| crc32_legacy(black_box(&data), black_box(0)))
    });
    drop(data);
}

fn crc_benchmark(c: &mut Criterion) {
    let mut data: Vec<u8> = [
        0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26, 83,
        24, 212,
    ]
    .to_vec();
    c.bench_function("append_crc", |b| {
        b.iter(|| append_crc(black_box(&mut data.to_owned()), black_box(0)))
    });
    drop(data);
    let mut data: Vec<u8> = [0, 21, 0, 0, 2].to_vec();
    c.bench_function("crc32", |b| {
        b.iter(|| crc32(black_box(&&mut data), black_box(0)))
    });
    drop(data);
}

fn utils_benchmark(c: &mut Criterion) {
    c.bench_function("generate_random_guid", |b| {
        b.iter(|| generate_random_guid())
    });
}

fn jooat_benchmark(c: &mut Criterion) {
    c.bench_function("joaat", |b| b.iter(|| joaat(black_box("HAX"))));
}

fn rc4_benchmark(c: &mut Criterion) {
    let key: [u8; 16] = [
        23, 189, 8, 107, 27, 148, 240, 47, 240, 236, 83, 215, 99, 88, 155, 95,
    ];
    c.bench_function("RC4::initialize", |b| {
        b.iter(|| RC4::initialize(black_box(key.to_vec())))
    });
    let key: [u8; 16] = [
        23, 189, 8, 107, 27, 148, 240, 47, 240, 236, 83, 215, 99, 88, 155, 95,
    ];
    let data: [u8; 34] = [
        5, 1, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 2, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0,
        116, 101, 115, 116,
    ];
    let mut rc4_obj = RC4::initialize(key.to_vec());
    c.bench_function("RC4::encrypt", |b| {
        b.iter(|| rc4_obj.encrypt(black_box(data.to_vec())))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    crc_legacy_benchmark(c);
    crc_benchmark(c);
    utils_benchmark(c);
    jooat_benchmark(c);
    rc4_benchmark(c);
    soeprotocol_parse_benchmarks(c);
    soeprotocol_pack_benchmarks(c);
    soeprotocol_utils_benchmarks(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
