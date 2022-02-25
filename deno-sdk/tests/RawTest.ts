const encoder = new TextEncoder();
const decoder = new TextDecoder();

async function rawGet(client: Deno.Conn, objectRef: string): Promise<string> {
    const request = {
        Get: {
            object_ref: objectRef
        }
    };
    await client.write(encoder.encode(JSON.stringify(request)));
    await client.write(encoder.encode('\n'));
    const buf = new Uint8Array(1024);
    await client.read(buf);
    return decoder.decode(buf);
}

const latteAddr = '127.0.0.1';
const port = 6379;
const client = await Deno.connect({ hostname: latteAddr, port });
const objectRef = 'my_id';
const value = await rawGet(client, objectRef);
client.close();
console.log(value);