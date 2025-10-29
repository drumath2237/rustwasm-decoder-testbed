import "./style.css";

import { zip_test_wasm } from "./wasm/sog-wasm";

import sogPath from "../../sample_data/pizza.sog?url";

async function main() {
  fetch(sogPath).then((res) => res.arrayBuffer()).then((data) => {
    const sogData = new Uint8Array(data);
    console.log(sogData.length);

    const files = zip_test_wasm(sogData);
    console.log(files);
  });
}

main();
