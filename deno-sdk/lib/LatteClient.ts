// Now the deno-sdk for latte programming only support 
// LatteObject as plain string. 
// The serde type model support will be implemented 
// in the future.
export class LatteClient {
    private socket: Deno.Conn;
    private encoder = new TextEncoder();
    private decoder = new TextDecoder();

    public static async connect(addr: { hostname: string, port: number} ): Promise<LatteClient> {
        const socket = await Deno.connect({ 
            hostname: addr.hostname, 
            port: addr.port });
        return new LatteClient(socket);
    }

    private constructor(socket: Deno.Conn) {
        this.socket = socket;
    }

    private async writeString(data: string) {
        await this.socket.write(this.encoder.encode(data + '\n'));
    }

    private async readAll(): Promise<string> {
        const buf = new Uint8Array(1024);
        await this.socket.read(buf);
        return this.decoder.decode(buf);
    }

    public close() {
        this.socket.close();
    }

    public async get(objectRef: string): Promise<string> {
        const req = {
            Get: {
                object_ref: objectRef
            }
        };
        await this.writeString(JSON.stringify(req));
        const reply = await this.readAll();
        return reply;
    }

    public async set(objectRef: string, latteObject: Record<string, unknown>) {
        const req = {
            Set: {
                object_ref: objectRef,
                latte_object: latteObject
            }
        };
        await this.writeString(JSON.stringify(req));
        const reply = await this.readAll();
        return reply;
    }
}
