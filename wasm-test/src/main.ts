import "./style.css";

import { zip_test_wasm } from "./wasm/sog-wasm";

import sogPath from "../../sample_data/pizza.sog?url";

async function main() {
  fetch(sogPath).then((res) => res.arrayBuffer()).then((data) => {
    const sogData = new Uint8Array(data);
    const files = zip_test_wasm(sogData).map((m) => {
      return { name: m.name, data: m.data };
    });
    console.log(files);
  });
}

main();
