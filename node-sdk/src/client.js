import net from 'net';
import { PromiseSocket } from 'promise-socket';

export async function connect(addr) {
  const socket = new PromiseSocket(new net.Socket());
  await socket.connect(addr);

  return new Client(socket);
}

class Client {
  constructor(socket) {
    this.socket = socket;
  }

  async writeString(data) {
    await this.socket.writeAll(data);
  }

  async readString() {
    const data = await this.socket.readAll();
    return data;
  }

  async register(funcSpec) {

  } 

  async invoke(fname, args) {

  }

  async get(objectRef) {
    const frame = new Get(objectRef);
    await this.writeString(frame.asString());
    return this.readAll();
  }

  async set(objectRef, value) {
    const frame = new Set_(objectRef, value);
    await this.writeString(frame.asString());
  }
}