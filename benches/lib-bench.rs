use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/lib.rs"]
mod lib;
use lib::crc::*;
use lib::gatewayprotocol::*;

use lib::jenkins::*;
use lib::rc4::*;
use lib::soeprotocol::*;
use lib::soeprotocol_functions::*;
use lib::soeprotocol_packets_structs::*;
use lib::spatial_hash_grid::*;
use lib::utils::*;

fn soeprotocol_utils_benchmarks(c: &mut Criterion) {
    let data_to_pack: Vec<u8> = [
        2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0,
    ]
    .to_vec();
    let wtr = vec![];
    let mut data_packet = DataPacket {
        data: data_to_pack,
        sequence: 0,
        error: None,
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
    let wtr = vec![];
    let mut data_packet = DataPacket {
        data: data_to_pack,
        sequence: 0,
        error: None,
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
    let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
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
    let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
    let session_request_to_pack =
        r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#
            .to_string();
    let session_reply_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
    let ping_to_pack: String = r#"{"name":"Ping"}"#.to_owned();
    let outoforder_to_pack: String = r#"{"name":"OutOfOrder","sequence":1}"#.to_owned();
    let ack_to_pack: String = r#"{"name":"Ack","sequence":1}"#.to_owned();
    let multi_to_pack: String =
        r#"{"sub_packets":[[0, 21, 0, 1],[0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165,
    71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225,
    169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11,
    247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]]}"#
            .to_owned();
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
    let session_request_to_pack_object =
        soeprotocol_class.get_session_request_object(session_request_to_pack.to_string());
    c.bench_function("session_request_pack_from_object", |b| {
        b.iter(|| {
            soeprotocol_class.pack_session_request_object(session_request_to_pack_object.clone())
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
    let session_reply_to_pack_object =
        soeprotocol_class.get_session_reply_object(session_reply_to_pack.to_string());
    c.bench_function("session_reply_to_pack_from_object", |b| {
        b.iter(|| soeprotocol_class.pack_session_reply_object(session_reply_to_pack_object.clone()))
    });
    c.bench_function("ping_to_pack", |b| {
        b.iter(|| soeprotocol_class.pack("Ping".to_owned(), black_box(ping_to_pack.to_string())))
    });
    c.bench_function("outoforder_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "OutOfOrder".to_owned(),
                black_box(outoforder_to_pack.to_string()),
            )
        })
    });
    let outoforder_to_pack_object =
        soeprotocol_class.get_ack_object(outoforder_to_pack.to_string());
    c.bench_function("outoforder_to_pack_from_object", |b| {
        b.iter(|| soeprotocol_class.pack_out_of_order_object(outoforder_to_pack_object.clone()))
    });
    c.bench_function("ack_to_pack", |b| {
        b.iter(|| soeprotocol_class.pack("Ack".to_owned(), black_box(ack_to_pack.to_string())))
    });
    let ack_to_pack_object = soeprotocol_class.get_ack_object(ack_to_pack.to_string());
    c.bench_function("ack_to_pack_from_object", |b| {
        b.iter(|| soeprotocol_class.pack_ack_object(ack_to_pack_object.clone()))
    });
    c.bench_function("multi_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "MultiPacket".to_owned(),
                black_box(multi_to_pack.to_string()),
            )
        })
    });
    let multi_to_pack_object = soeprotocol_class.get_multi_object(multi_to_pack.to_string());
    c.bench_function("multi_to_pack_from_object", |b| {
        b.iter(|| soeprotocol_class.pack_multi_object(multi_to_pack_object.clone()))
    });
    c.bench_function("data_to_pack", |b| {
        b.iter(|| soeprotocol_class.pack("Data".to_owned(), black_box(data_to_pack.to_string())))
    });
    let data_to_pack_object = soeprotocol_class.get_data_object(data_to_pack.to_string());
    c.bench_function("data_to_pack_from_object", |b| {
        b.iter(|| soeprotocol_class.pack_data_object(data_to_pack_object.clone()))
    });
    c.bench_function("data_fragment_to_pack", |b| {
        b.iter(|| {
            soeprotocol_class.pack(
                "DataFragment".to_owned(),
                black_box(data_fragment_to_pack.to_string()),
            )
        })
    });
    let data_fragment_to_pack_object =
        soeprotocol_class.get_data_object(data_fragment_to_pack.to_string());
    c.bench_function("data_fragment_to_pack_from_object", |b| {
        b.iter(|| soeprotocol_class.pack_fragment_data_object(data_fragment_to_pack_object.clone()))
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
        b.iter(|| soeprotocol_class.pack("Ping".to_owned(), black_box(ping_to_pack.to_string())))
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
        b.iter(|| soeprotocol_class.pack("Ack".to_owned(), black_box(ack_to_pack.to_string())))
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
        b.iter(|| soeprotocol_class.pack("Data".to_owned(), black_box(data_to_pack.to_string())))
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
fn gatewayprotocol_parse_benchmarks(c: &mut Criterion) {
    let mut gatewayprotocol = GatewayProtocol::initialize();
    // define data used in benchmarks
    let login_request_to_parse: [u8; 59] = [
        1, 244, 221, 253, 245, 153, 56, 150, 124, 5, 0, 0, 0, 105, 116, 115, 109, 101, 19, 0, 0, 0,
        67, 108, 105, 101, 110, 116, 80, 114, 111, 116, 111, 99, 111, 108, 95, 49, 48, 56, 48, 14,
        0, 0, 0, 48, 46, 49, 57, 53, 46, 52, 46, 49, 52, 55, 53, 56, 54,
    ];

    let tunnel_data_to_parse: [u8; 32] = [
        70, 254, 3, 237, 98, 176, 99, 0, 109, 235, 2, 98, 113, 5, 229, 11, 115, 16, 119, 61, 0, 0,
        0, 0, 0, 0, 0, 0, 48, 33, 0, 0,
    ];

    c.bench_function("login_request_parse", |b| {
        b.iter(|| gatewayprotocol.parse(black_box(login_request_to_parse.to_vec())))
    });
    c.bench_function("login_reply_parse", |b| {
        b.iter(|| gatewayprotocol.parse(black_box([2, 1].to_vec())))
    });
    c.bench_function("tunnel_data_parse", |b| {
        b.iter(|| gatewayprotocol.parse(black_box(tunnel_data_to_parse.to_vec())))
    });
}
fn gatewayprotocol_pack_benchmarks(c: &mut Criterion) {
    let mut gatewayprotocol = GatewayProtocol::initialize();
    // define data used in benchmarks
    let tunnel_data_to_pack = [68, 82, 37, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0];

    c.bench_function("tunnel_data_pack", |b| {
        b.iter(|| {
            gatewayprotocol
                .pack_tunnel_data_packet_for_client(black_box(tunnel_data_to_pack.to_vec()))
        })
    });
    c.bench_function("login_request_pack", |b| {
        b.iter(|| {
            gatewayprotocol.pack_login_request_packet(
                8977425141117869556,
                "itsme".to_owned(),
                "ClientProtocol_1080".to_owned(),
                "0.195.4.147586".to_owned(),
            )
        })
    });
    c.bench_function("login_reply_pack", |b| {
        b.iter(|| gatewayprotocol.pack_login_reply_packet(black_box(true)))
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
    let data: Vec<u8> = [
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
    c.bench_function("generate_random_guid", |b| b.iter(generate_random_guid));

    c.bench_function("eul2quat", |b| {
        b.iter(|| eul2quat([1.0, 2.0, 3.0].to_vec()))
    });

    c.bench_function("is_pos_in_radius", |b| {
        b.iter(|| {
            is_pos_in_radius(
                20.0,
                [0.0, 1.0, 2.0, 0.0].to_vec(),
                [-19.0, 1.0, 20.0, 0.0].to_vec(),
            )
        })
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
fn spatial_grid_benchmark(c: &mut Criterion) {
    let dimensions = [100, 100].to_vec();
    let bounds = [-1000.0, -1000.0, 1000.0, 1000.0].to_vec();
    c.bench_function("SpatialGrid::new", |b| {
        b.iter(|| SpatialHashGrid::new(bounds.clone(), dimensions.clone()))
    });
    c.bench_function("SpatialGrid::create_client", |b| {
        let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
        let id: u64 = 1;
        let position = [10.0, 20.0, 3.0].to_vec();
        b.iter(|| sgrid.create_client(position.clone(), id))
    });
    c.bench_function("SpatialGrid::create_clients", |b| {
        let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
        let mut id: u64 = 1;
        let position = [10.0, 20.0, 3.0].to_vec();
        b.iter(|| {
            id += 1;
            sgrid.create_client(position.clone(), id)
        })
    });
    c.bench_function("SpatialGrid::create_clients on different positions", |b| {
        let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
        let mut id: u64 = 1;
        let mut position = [10.0, 20.0, 3.0].to_vec();
        b.iter(|| {
            id += 1;
            position[0] += 1.0;
            if position[0] == 1000.0 {
                position[0] -= 2000.0;
            }
            sgrid.create_client(position.clone(), id)
        })
    });
    c.bench_function(
        "SpatialGrid::remove_client (contains client creation)",
        |b| {
            let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
            let id: u64 = 1;
            let position = [10.0, 20.0, 3.0].to_vec();
            b.iter(|| {
                let idx = sgrid.create_client(position.clone(), id);
                sgrid.remove(idx, id);
            })
        },
    );
    c.bench_function("SpatialGrid::find_nearby", |b| {
        let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
        let id: u64 = 1;
        let position = [10.0, 20.0, 3.0].to_vec();
        sgrid.create_client(position.clone(), id);
        b.iter(|| {
            sgrid.find_nearby(position.clone(), 300.0);
        })
    });
    c.bench_function("SpatialGrid::find_nearby 500 clients", |b| {
        let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
        let position = [10.0, 20.0, 3.0].to_vec();
        for i in 0..500 {
            sgrid.create_client(position.clone(), i as u64);
        }
        b.iter(|| {
            sgrid.find_nearby(position.clone(), 300.0);
        })
    });
    c.bench_function(
        "SpatialGrid::find_nearby 500 clients in diffent positions",
        |b| {
            let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
            let mut position = [10.0, 20.0, 3.0].to_vec();
            for i in 0..500 {
                position[0] += 1.0;
                sgrid.create_client(position.clone(), i as u64);
            }
            b.iter(|| {
                sgrid.find_nearby(position.clone(), 300.0);
            })
        },
    );
    c.bench_function("SpatialGrid::find_nearby 50000 clients", |b| {
        let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
        let position = [10.0, 20.0, 3.0].to_vec();
        for i in 0..50000 {
            sgrid.create_client(position.clone(), i as u64);
        }
        b.iter(|| {
            sgrid.find_nearby(position.clone(), 300.0);
        })
    });
    c.bench_function(
        "SpatialGrid::find_nearby 50000 clients in diffent positions",
        |b| {
            let mut sgrid = SpatialHashGrid::new(bounds.clone(), dimensions.clone());
            let mut position = [10.0, 20.0, 3.0].to_vec();
            for i in 0..50000 {
                position[0] += 1.0;
                if position[0] == 1000.0 {
                    position[0] -= 2000.0;
                }
                sgrid.create_client(position.clone(), i as u64);
            }
            b.iter(|| {
                sgrid.find_nearby(position.clone(), 300.0);
            })
        },
    );
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
    gatewayprotocol_parse_benchmarks(c);
    gatewayprotocol_pack_benchmarks(c);
    spatial_grid_benchmark(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
