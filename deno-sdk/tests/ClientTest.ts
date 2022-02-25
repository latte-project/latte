import { LatteClient } from '../lib/LatteClient.ts';
const hostname = '127.0.0.1';
const port = 6379;

const client = await LatteClient.connect({ hostname, port });
const ref = 'my_id';
const latteObject = {
    String: 128
};

const reply1 = await client.set(ref, latteObject);
console.log(reply1);

const reply2 = await client.get(ref);
console.log(reply2);

client.close();