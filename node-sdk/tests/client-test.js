import { connect } from "../src/client";

async function main() {
  const client = await connect('127.0.0.1:6379');
  const latteObject = {
    String: "Hello"
  };
  await client.set('1234', latteObject);
  const answer = await client.get('1234');
  console.log(answer);
}

main();