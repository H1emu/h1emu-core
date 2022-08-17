import { Soeprotocol } from "../pkg/h1emu_core.js";

function bench(name: string, func: any) {
  for (let i = 0; i < 100; i++) {
    func();
  }
  console.time(name);
  func();
  console.timeEnd(name);
}

console.time("Init Protocol");
const soeProtocol = new Soeprotocol(false, 0);
console.timeEnd("Init Protocol");

console.log("\n Parse tests \n");
const sessionRequest = new Uint8Array([
  0, 1, 0, 0, 0, 3, 60, 23, 140, 99, 0, 0, 2, 0, 76, 111, 103, 105, 110, 85,
  100, 112, 95, 57, 0,
]);
const sessionReply = new Uint8Array([
  0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3,
]);
const ping = new Uint8Array([0, 6]);

const outOfOrderPacket = new Uint8Array([0, 17, 0, 1]);
const ackPacket = new Uint8Array([0, 21, 0, 1]);
const MultiPacket = new Uint8Array([
  0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64,
  165, 71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247,
  152, 225, 169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121,
  222, 106, 11, 247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11,
  180, 97, 10, 246,
]);
const dataPacket = new Uint8Array([
  0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24,
  155, 2, 113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124,
  107, 61, 62, 132, 117, 146, 204,
]);
const dataFragmentPacket = new Uint8Array([
  0, 13, 0, 2, 208, 127, 31, 117, 87, 54, 201, 180, 188, 226, 247, 253, 136, 66,
  78, 125, 224, 112, 23, 87, 147, 110, 18, 68, 183, 87, 20, 3, 65, 116, 82, 111,
  93, 219, 229, 20, 61, 238, 143, 63, 8, 137, 8, 196, 128, 89, 59, 4, 198, 191,
  207, 141, 23, 164, 242, 77, 176, 206, 49, 45, 207, 210, 17, 33, 75, 177, 157,
  242, 169, 37, 60, 87, 245, 58, 2, 130, 102, 146, 227, 66, 193, 153, 155, 105,
  230, 203, 120, 114, 160, 223, 229, 190, 129, 106, 19, 25, 8, 52, 55, 8, 100,
  68, 109, 228, 178, 186, 148, 108, 138, 242, 136, 66, 219, 25, 73, 129, 110,
  31, 121, 32, 246, 86, 156, 212, 85, 217, 213, 119, 165, 140, 83, 95, 6, 183,
  184, 251, 73, 102, 221, 156, 240, 204, 50, 217, 217, 13, 218, 2, 19, 44, 143,
  73, 168, 109, 67, 176, 129, 225, 187, 171, 12, 146, 21, 66, 252, 150, 143,
  142, 46, 39, 72, 12, 22, 222, 7, 29, 63, 201, 227, 251, 9, 28, 0, 100, 84,
  153, 84, 212, 163, 78, 135, 33, 66, 20, 195, 223, 62, 214, 32, 59, 6, 187,
  222, 99, 29, 34, 87, 81, 61, 63, 174, 255, 1, 85, 241, 6, 10, 152, 237, 52,
  51, 126, 149, 218, 125, 232, 199, 40, 113, 139, 187, 43, 232, 209, 167, 226,
  91, 236, 212, 165, 117, 19, 118, 110, 18, 0, 26, 152, 33, 115, 61,
]);

bench("Parse Session Request", () => {
  soeProtocol.parse(sessionRequest);
});

bench("Parse Session Reply", () => {
  soeProtocol.parse(sessionReply);
});

bench("Parse Ping", () => {
  soeProtocol.parse(ping);
});

bench("Parse Out of order packet", () => {
  soeProtocol.parse(outOfOrderPacket);
});

bench("Parse Ack packet", () => {
  soeProtocol.parse(ackPacket);
});

bench("Parse Multi packet", () => {
  soeProtocol.parse(MultiPacket);
});

bench("Parse Data packet", () => {
  soeProtocol.parse(dataPacket);
});

bench("Parse Data Fragment packet", () => {
  soeProtocol.parse(dataFragmentPacket);
});

console.log("\n Parse tests with Buffer.from \n");

bench("Parse Session Request", () => {
  Buffer.from(soeProtocol.parse(sessionRequest));
});

bench("Parse Session Reply", () => {
  Buffer.from(soeProtocol.parse(sessionReply));
});

bench("Parse Ping", () => {
  Buffer.from(soeProtocol.parse(ping));
});

bench("Parse Out of order packet", () => {
  Buffer.from(soeProtocol.parse(outOfOrderPacket));
});

bench("Parse Ack packet", () => {
  Buffer.from(soeProtocol.parse(ackPacket));
});

bench("Parse Multi packet", () => {
  Buffer.from(soeProtocol.parse(MultiPacket));
});

bench("Parse Data packet", () => {
  Buffer.from(soeProtocol.parse(dataPacket));
});

bench("Parse Data Fragment packet", () => {
  Buffer.from(soeProtocol.parse(dataFragmentPacket));
});

console.log("\n Pack tests \n");

const sessionRequestToPack =
  '{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}';
const sessionReplyToPack =
  '{"session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}';
const pingToPack = '{"name":"Ping"}';
const outOfOrderPacketToPack = '{"name":"OutOfOrder","sequence":1}';
const ackPacketToPack = '{"name":"Ack","sequence":1}';
const MultiPacketToPack =
  '{"sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}';
const dataPacketToPack =
  '{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}';
const dataFragmentPacketToPack =
  '{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}';

bench("Pack Session Request", () => {
  soeProtocol.pack_session_request(sessionRequestToPack);
});

bench("Pack Session Reply", () => {
  soeProtocol.pack_session_reply(sessionReplyToPack);
});

bench("Pack Ping", () => {
  soeProtocol.pack("Ping", pingToPack);
});

bench("Pack Out of order packet", () => {
  soeProtocol.pack_out_of_order(outOfOrderPacketToPack);
});

bench("Pack Ack packet", () => {
  soeProtocol.pack_ack(ackPacketToPack);
});

bench("Pack Multi packet", () => {
  soeProtocol.pack_multi(MultiPacketToPack);
});

bench("Pack Data packet", () => {
  soeProtocol.pack_data(dataPacketToPack);
});

bench("Pack Data Fragment packet", () => {
  soeProtocol.pack_data(dataFragmentPacketToPack);
});

console.log("\n Pack tests with stringify \n");

const sessionRequestToPackStringify = JSON.parse(sessionRequestToPack);
const sessionReplyToPackStringify = JSON.parse(sessionReplyToPack);
const pingToPackStringify = JSON.parse(pingToPack);
const outOfOrderPacketToPackStringify = JSON.parse(outOfOrderPacketToPack);
const ackPacketToPackStringify = JSON.parse(ackPacketToPack);
const MultiPacketToPackStringify = JSON.parse(MultiPacketToPack);
const dataPacketToPackStringify = JSON.parse(dataPacketToPack);
const dataFragmentPacketToPackStringify = JSON.parse(dataFragmentPacketToPack);

bench("Pack Session Request", () => {
  soeProtocol.pack_session_request(
    JSON.stringify(sessionRequestToPackStringify)
  );
});

bench("Pack Session Reply", () => {
  soeProtocol.pack_session_reply(JSON.stringify(sessionReplyToPackStringify));
});
bench("Pack Ping", () => {
  soeProtocol.pack("Ping", JSON.stringify(pingToPackStringify));
});
bench("Pack Out of order packet", () => {
  soeProtocol.pack_out_of_order(
    JSON.stringify(outOfOrderPacketToPackStringify)
  );
});
bench("Pack Ack packet", () => {
  soeProtocol.pack_ack(JSON.stringify(ackPacketToPackStringify));
});
bench("Pack Multi packet", () => {
  soeProtocol.pack_multi(JSON.stringify(MultiPacketToPackStringify));
});
bench("Pack Data packet", () => {
  soeProtocol.pack_data(JSON.stringify(dataPacketToPackStringify));
});
bench("Pack Data Fragment packet", () => {
  soeProtocol.pack_data(JSON.stringify(dataFragmentPacketToPackStringify));
});
