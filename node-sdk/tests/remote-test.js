import { remote, get } from "../src";

function add(a, b) {
  return a + b; 
}

// latte.start()
const addRemote = remote(add);
const ans = get(addRemote(1, 2));
assert(ans, 3);