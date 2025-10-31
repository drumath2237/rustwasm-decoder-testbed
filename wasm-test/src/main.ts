import "./style.css";

import {
  deserialize_metajson,
  test_func,
  zip_test_wasm,
} from "./wasm/sog-wasm";

import sogPath from "../../sample_data/pizza.sog?url";
import metaJsonPath from "../../sample_data/pizza/meta.json?url";

async function main() {
  fetch(sogPath).then((res) => res.arrayBuffer()).then((data) => {
    const sogData = new Uint8Array(data);
    const files = zip_test_wasm(sogData).map((m) => {
      return { name: m.name, data: m.data };
    });
    console.log(files);
  });

  fetch(metaJsonPath).then((res) => res.text()).then((json) => {
    const metaData = deserialize_metajson(json);
    console.log(metaData);
  });

  console.log(test_func());
}

main();
